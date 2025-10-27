use magnus::{Error, Integer, RString, Ruby, Value, class, function, method, prelude::*};
use std::fs;
use std::io::Read;
use std::os::fd::FromRawFd;
use std::sync::{Arc, Mutex};
use zip::ZipArchive;

type Result<T> = std::result::Result<T, Error>;

fn map_err(err: impl std::error::Error, ruby: &Ruby) -> Error {
    Error::new(ruby.exception_runtime_error(), format!("{}", err))
}

#[magnus::wrap(class = "RuZip::Archive", free_immediately, size)]
struct Archive(Arc<Mutex<ZipArchive<fs::File>>>);

impl Archive {
    fn new(ruby: &Ruby, r_io: Value) -> Result<Self> {
        let io = if r_io.is_kind_of(class::io()) {
            let fileno: Integer = r_io.funcall_public("fileno", ())?;
            let raw_fd = fileno.to_i32()?;
            unsafe { fs::File::from_raw_fd(raw_fd) }
        } else if r_io.respond_to("to_path", false)? {
            fs::File::open(
                r_io.funcall_public::<&str, (), RString>("to_path", ())?
                    .to_string()?,
            )
            .map_err(|e| map_err(e, ruby))?
        } else if r_io.respond_to("read", false)? {
            // IO(r_io);
            todo!("#read");
        } else if r_io.is_kind_of(class::string()) {
            fs::File::open(r_io.to_r_string()?.to_string()?).map_err(|e| map_err(e, ruby))?
        } else {
            return Err(Error::new(
                ruby.exception_type_error(),
                format!("Unsupported argument type: {}", r_io.inspect()),
            ));
        };
        let zip: ZipArchive<fs::File> = ZipArchive::new(io).map_err(|e| map_err(e, ruby))?;
        Ok(Self(Arc::new(Mutex::new(zip))))
    }

    fn len(ruby: &Ruby, rb_self: &Self) -> Result<usize> {
        Ok(rb_self.0.lock().map_err(|e| map_err(e, ruby))?.len())
    }

    fn by_index(ruby: &Ruby, rb_self: &Self, index: usize) -> Result<File> {
        match rb_self
            .0
            .lock()
            .map_err(|e| map_err(e, ruby))?
            .by_index(index)
        {
            Ok(_) => Ok(File(rb_self.0.clone(), index)),
            Err(e) => Err(map_err(e, ruby)),
        }
    }

    fn by_name(ruby: &Ruby, rb_self: &Self, name: RString) -> Result<Option<File>> {
        let name_string = name
            .to_string()
            .map_err(|e| Error::new(ruby.exception_runtime_error(), format!("{}", e)))?;
        let mut archive = rb_self.0.lock().map_err(|e| map_err(e, ruby))?;
        // TODO: Cache entries
        for i in 0..archive.len() {
            let file = archive.by_index(i).map_err(|e| map_err(e, ruby))?;
            if file.name_raw() == name_string.clone().into_bytes() {
                return Ok(Some(File(rb_self.0.clone(), i)));
            }
        }
        Ok(None)
    }
}

#[magnus::wrap(class = "RuZip::File")]
struct File(Arc<Mutex<ZipArchive<fs::File>>>, usize);

impl File {
    fn name(ruby: &Ruby, rb_self: &Self) -> Result<String> {
        String::from_utf8(
            rb_self
                .0
                .lock()
                .map_err(|e| map_err(e, ruby))?
                .by_index(rb_self.1)
                .map_err(|e| map_err(e, ruby))?
                .name_raw()
                .into(),
        )
        .map_err(|e| map_err(e, ruby))
    }

    fn size(ruby: &Ruby, rb_self: &Self) -> Result<u64> {
        let size = rb_self
            .0
            .lock()
            .map_err(|e| map_err(e, ruby))?
            .by_index(rb_self.1)
            .map_err(|e| map_err(e, ruby))?
            .size();
        Ok(size)
    }

    // TODO: Use ExtendedTimestamp if available
    fn last_modified(ruby: &Ruby, rb_self: &Self) -> Result<Option<Value>> {
        let last_modified = rb_self
            .0
            .lock()
            .map_err(|e| map_err(e, ruby))?
            .by_index(rb_self.1)
            .map_err(|e| map_err(e, ruby))?
            .last_modified();
        match last_modified {
            Some(mtime) => Ok(Some(ruby.class_time().new_instance((
                mtime.year(),
                mtime.month(),
                mtime.day(),
                mtime.hour(),
                mtime.minute(),
                mtime.second(),
            ))?)),
            None => Ok(None),
        }
    }

    fn read(ruby: &Ruby, rb_self: &Self) -> Result<RString> {
        let mut archive = rb_self.0.lock().map_err(|e| map_err(e, ruby))?;
        let mut file = archive.by_index(rb_self.1).map_err(|e| map_err(e, ruby))?;
        let mut buf = Vec::with_capacity(file.size() as usize);
        file.read_to_end(&mut buf).map_err(|e| map_err(e, ruby))?;
        Ok(RString::from_slice(&buf))
    }
}

#[magnus::init]
fn init(ruby: &Ruby) -> Result<()> {
    let module = ruby.define_module("RuZip")?;

    let archive_class = module.define_class("Archive", ruby.class_object())?;
    archive_class.define_singleton_method("new", function!(Archive::new, 1))?;
    archive_class.define_method("length", method!(Archive::len, 0))?;

    let file_class = module.define_class("File", ruby.class_object())?;
    archive_class.define_method("by_index", method!(Archive::by_index, 1))?;
    archive_class.define_method("by_name", method!(Archive::by_name, 1))?;
    file_class.define_method("name", method!(File::name, 0))?;
    file_class.define_method("size", method!(File::size, 0))?;
    file_class.define_method("last_modified", method!(File::last_modified, 0))?;
    file_class.define_method("read", method!(File::read, 0))?;

    Ok(())
}
