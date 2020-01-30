#[derive(Debug)]
pub struct Error {}

pub type Result<T> = std::result::Result<T, Error>;

macro_rules! try_unsafe {
    ($x:expr) => {
        match unsafe { $x } {
            0 => (),
            _ => return Err(Error {}),
        }
    };
}
