#[cfg(all(feature = "std", feature = "use-core2"))]
compile_error!("feature \"std\" and \"use-core2\" cannot be enabled together.");

#[derive(Debug)]
pub enum Error {
    Wrapped(::alloc::boxed::Box<dyn Marker>),

    // Needed for write_all blanket implementation
    WriteZero,
    Interrupted,
}

pub trait Marker: ::core::fmt::Display + ::core::fmt::Debug {}

pub trait Read{
    fn read(&mut self, buf: &mut [u8]) -> ::core::result::Result<usize, Error>;
}

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> ::core::result::Result<usize, Error>;
    fn flush(&mut self) -> ::core::result::Result<(), Error>;

    fn write_all(&mut self, mut buf: &[u8]) -> ::core::result::Result<(), Error> {
        while !buf.is_empty() {
            match self.write(buf) {
                Ok(0) => {
                    return Err(Error::WriteZero);
                }
                Ok(n) => buf = &buf[n..],
                Err(Error::Interrupted) => {}
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }
}

#[cfg(feature = "std")]
mod std_impl {
    use super::{Read, Write, Error, Marker};

    impl Marker for ::std::io::Error {}

    impl From<::std::io::Error> for Error {
        fn from(error: ::std::io::Error) -> Self {
            if let ::std::io::ErrorKind::Interrupted = error.kind() {
                Error::Interrupted
            } else {
                Error::Wrapped(::std::boxed::Box::new(error))
            }
        }
    }

    impl<R: ::std::io::Read> Read for R {
        fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
            Ok(<Self as ::std::io::Read>::read(self, buf)?)
        }
    }

    impl<W: ::std::io::Write> Write for W {
        fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
            Ok(<Self as ::std::io::Write>::write(self, buf)?)
        }

        fn flush(&mut self) -> Result<(), Error> {
            Ok(<Self as ::std::io::Write>::flush(self)?)
        }
    }
}

#[cfg(feature = "use-core2")]
mod core2_impl {
    use super::{Read, Write, Error, Marker};

    impl Marker for core2::io::Error {}

    impl From<core2::io::Error> for Error {
        fn from(error: core2::io::Error) -> Self {
            if let core2::io::ErrorKind::Interrupted = error.kind() {
                Error::Interrupted
            } else {
                Error::Wrapped(::alloc::boxed::Box::new(error))
            }
        }
    }

    impl<R: core2::io::Read> Read for R {
        fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
            Ok(<Self as core2::io::Read>::read(self, buf)?)
        }
    }

    impl<W: core2::io::Write> Write for W {
        fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
            Ok(<Self as core2::io::Write>::write(self, buf)?)
        }

        fn flush(&mut self) -> Result<(), Error> {
            Ok(<Self as core2::io::Write>::flush(self)?)
        }
    }
}

#[cfg(test)]
mod tests {

    #[cfg(feature = "std")]
    mod std_test {
        use ::literacy::{Read, Write};

        #[test]
        fn test_std_read() {
            let mut cursor = ::std::io::Cursor::new(vec![10u8]);
            let mut buf = [0u8; 1];
            cursor.read(&mut buf).unwrap();
            assert_eq!(buf, [10u8]);
        }

        #[test]
        fn test_std_write() {
            let mut cursor = ::std::io::Cursor::new(vec![]);
            let mut buf = [10u8; 1];
            cursor.write(&mut buf).unwrap();
            assert_eq!(cursor.into_inner(), vec![10u8]);
        }
    }

    #[cfg(feature = "use-core2")]
    mod tests {
        use ::literacy::{Read, Write};

        #[test]
        fn test_core2_read() {
            let mut cursor = core2::io::Cursor::new(vec![10u8]);
            let mut buf = [0u8; 1];
            cursor.read(&mut buf).unwrap();
            assert_eq!(buf, [10u8]);
        }

        #[test]
        fn test_core2_write() {
            let mut cursor = core2::io::Cursor::new(vec![]);
            let mut buf = [10u8; 1];
            cursor.write(&mut buf).unwrap();
            assert_eq!(cursor.into_inner(), vec![10u8]);
        }
    }
}
