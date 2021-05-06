#[cfg(all(feature = "std", feature = "use-core2"))]
compile_error!("feature \"std\" and \"use-core2\" cannot be enabled together.");


pub trait Read{
    type Error;
    fn read(&mut self, buf: &mut [u8]) -> ::core::result::Result<usize, Self::Error>;
}

pub trait Write {
    type Error;
    fn write(&mut self, buf: &[u8]) -> ::core::result::Result<usize, Self::Error>;
    fn flush(&mut self) -> ::core::result::Result<(), Self::Error>;
    fn write_all(&mut self, mut buf: &[u8]) -> ::core::result::Result<(), Self::Error> {
        while !buf.is_empty() {
            match self.write(buf) {
                /*Ok(0) => {
                    return Err(Error::new(ErrorKind::WriteZero, "failed to write whole buffer"));
                }*/
                Ok(n) => buf = &buf[n..],
                //Err(ref e) if e.kind() == ErrorKind::Interrupted => {}
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }
}

#[cfg(feature = "std")]
mod std_impl {
    use super::{Read, Write};

    impl<R: ::std::io::Read> Read for R {
        type Error = ::std::io::Error;

        fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
            Ok(<Self as ::std::io::Read>::read(self, buf)?)
        }
    }

    impl<W: ::std::io::Write> Write for W {
        type Error = ::std::io::Error;

        fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
            <Self as ::std::io::Write>::write(self, buf)
        }

        fn flush(&mut self) -> Result<(), Self::Error> {
            <Self as ::std::io::Write>::flush(self)
        }
    }
}

#[cfg(feature = "use-core2")]
mod core2_impl {
    use super::{Read, Write};

    impl<R: core2::io::Read> Read for R {
        type Error = core2::io::Error;

        fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
            Ok(<Self as core2::io::Read>::read(self, buf)?)
        }
    }

    impl<W: core2::io::Write> Write for W {
        type Error = core2::io::Error;

        fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
            Ok(<Self as core2::io::Write>::write(self, buf)?)
        }

        fn flush(&mut self) -> Result<(), Self::Error> {
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
