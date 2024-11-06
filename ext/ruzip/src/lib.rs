use magnus::{
    class, exception, function, method, prelude::*, Error, Integer, RString, Ruby, Symbol, Value,
};
use std::cell::RefCell;
use std::fs;
use std::io::{self, Read};
use std::os::fd::FromRawFd;
use zip::ZipArchive;

#[magnus::wrap(class = "RuZip::Archive", free_immediately, size)]
struct Archive(RefCell<ZipArchive<fs::File>>);

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
    fn new(r_io: Value) -> Result<Self, Error> {
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
        Ok(Self(zip.into()))
    }

    fn len(&self) -> usize {
        self.0.borrow().len()
    }

    fn by_name(&self, name: RString) -> Result<Option<File>, Error> {
        let name_string = name
            .to_string()
            .map_err(|e| Error::new(exception::runtime_error(), format!("{}", e)))?;
        let archive = self.0.borrow_mut();
        Ok(archive.index_for_name(&name_string).map(File))
    }
}

#[magnus::wrap(class = "RuZip::File")]
struct File(usize);

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    let module = ruby.define_module("RuZip")?;

    let archive_class = module.define_class("Archive", ruby.class_object())?;
    archive_class.define_singleton_method("new", function!(Archive::new, 1))?;
    archive_class.define_method("length", method!(Archive::len, 0))?;

    let file_class = module.define_class("File", ruby.class_object())?;
    archive_class.define_method("by_name", method!(Archive::by_name, 1))?;
    Ok(())
}
