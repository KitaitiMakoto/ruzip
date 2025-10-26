use magnus::{
    class, exception, function, method, prelude::*, Error, Integer, RString, Ruby, Symbol, Value,
};
use std::fs;
use std::io::{self, Read};
use std::os::fd::FromRawFd;
use std::sync::{Arc, Mutex};
use zip::ZipArchive;

type Result<T> = std::result::Result<T, Error>;

#[magnus::wrap(class = "RuZip::Archive", free_immediately, size)]
struct Archive(Arc<Mutex<ZipArchive<fs::File>>>);

struct IO(Value);

impl io::Read for IO {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let buf_len = buf.len();
        let r_string: Value = self.0.funcall_public("read", (buf_len,)).unwrap(); // FIXME: unwrap(). Should use second arg?
        if r_string.is_nil() {
            Ok(0)
        } else {
            let data = r_string.to_string();
            let len = data.len();
            buf.clone_from_slice(data.as_bytes()); // FIXME: clone_from_slice panics when sizes of buf and data are not the same
            Ok(len)
        }
    }
}

impl io::Seek for IO {
    fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
        use io::SeekFrom::*;
        let value = self.0;

        match pos {
            Start(i) => value
                .funcall_public::<&str, (u64, Symbol), u64>("seek", (i, Symbol::new("SET")))
                .unwrap(), // FIXME: unwrap()
            End(i) => value
                .funcall_public("seek", (i, Symbol::new("END")))
                .unwrap(),
            Current(i) => value
                .funcall_public("seek", (i, Symbol::new("CUR")))
                .unwrap(),
        };
        Ok(value.funcall_public("pos", ()).unwrap())
    }
}

impl Archive {
    fn new(r_io: Value) -> Result<Self> {
        let io = if r_io.is_kind_of(class::io()) {
            let fileno: Integer = r_io.funcall_public("fileno", ())?;
            let raw_fd = fileno.to_i32()?;
            unsafe { fs::File::from_raw_fd(raw_fd) }
        } else if r_io.respond_to("to_path", false)? {
            fs::File::open(
                r_io.funcall_public::<&str, (), RString>("to_path", ())?
                    .to_string()?,
            )
            .unwrap()
        } else if r_io.respond_to("read", false)? {
            // IO(r_io);
            todo!("#read");
        } else if r_io.is_kind_of(class::string()) {
            fs::File::open(r_io.to_r_string()?.to_string()?).unwrap() // FIXME: unwrap()
        } else {
            return Err(Error::new(
                exception::type_error(),
                format!("Unsupported argument type: {}", r_io.inspect()),
            ));
        };
        let zip: ZipArchive<fs::File> = ZipArchive::new(io)
            .map_err(|e| Error::new(exception::runtime_error(), format!("{}", e)))?;
        Ok(Self(Arc::new(Mutex::new(zip))))
    }

    fn len(&self) -> Result<usize> {
        Ok(self
            .0
            .lock()
            .map_err(|e| Error::new(exception::runtime_error(), format!("{}", e)))?
            .len())
    }

    fn by_name(&self, name: RString) -> Result<Option<File>> {
        let name_string = name
            .to_string()
            .map_err(|e| Error::new(exception::runtime_error(), format!("{}", e)))?;
        Ok(self
            .0
            .lock()
            .map_err(|e| Error::new(exception::runtime_error(), format!("{}", e)))?
            .index_for_name(&name_string)
            .map(|i| File(self.0.clone(), i)))
    }
}

#[magnus::wrap(class = "RuZip::File")]
struct File(Arc<Mutex<ZipArchive<fs::File>>>, usize);

impl File {
    // FIXME: exception::runtime_error() -> ruby.exception_runtime_error()
    fn size(&self) -> Result<u64> {
        let size = self
            .0
            .lock()
            .map_err(|e| Error::new(exception::runtime_error(), format!("{}", e)))?
            .by_index(self.1)
            .map_err(|e| Error::new(exception::runtime_error(), format!("{}", e)))?
            .size();
        Ok(size)
    }

    // TODO: Use ExtendedTimestamp if available
    fn last_modified(&self) -> Result<Option<Value>> {
        let last_modified = self
            .0
            .lock()
            .map_err(|e| Error::new(exception::runtime_error(), format!("{}", e)))?
            .by_index(self.1)
            .map_err(|e| Error::new(exception::runtime_error(), format!("{}", e)))?
            .last_modified();
        match last_modified {
            Some(mtime) => Ok(Some(magnus::class::time().new_instance((mtime.year(), mtime.month(), mtime.day(), mtime.hour(), mtime.minute(), mtime.second()))?)),
            None => Ok(None),
        }
    }

    fn read(&self) -> Result<String> {
        let mut buf = String::new();
        self
            .0
            .lock()
            .map_err(|e| Error::new(exception::runtime_error(), format!("{}", e)))?
            .by_index(self.1)
            .map_err(|e| Error::new(exception::runtime_error(), format!("{}", e)))?
            .read_to_string(&mut buf)
            .map_err(|e| Error::new(exception::runtime_error(), format!("{}", e)))?;
        Ok(buf)
    }
}

#[magnus::init]
fn init(ruby: &Ruby) -> Result<()> {
    let module = ruby.define_module("RuZip")?;

    let archive_class = module.define_class("Archive", ruby.class_object())?;
    archive_class.define_singleton_method("new", function!(Archive::new, 1))?;
    archive_class.define_method("length", method!(Archive::len, 0))?;

    let file_class = module.define_class("File", ruby.class_object())?;
    archive_class.define_method("by_name", method!(Archive::by_name, 1))?;
    file_class.define_method("size", method!(File::size, 0))?;
    file_class.define_method("last_modified", method!(File::last_modified, 0))?;
    file_class.define_method("read", method!(File::read, 0))?;

    Ok(())
}
