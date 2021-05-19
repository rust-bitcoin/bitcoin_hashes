
pub trait Read {
    type Error;
    fn read(&mut self, buf: &mut [u8]) -> ::core::result::Result<usize, Self::Error>;
    fn read_exact(&mut self, buf: &mut [u8]) -> ::core::result::Result<(), Self::Error>;
}

pub trait Write {
    type Error;
    fn write(&mut self, buf: &[u8]) -> ::core::result::Result<usize, Self::Error>;
    fn write_all(&mut self, buf: &[u8]) -> ::core::result::Result<(), Self::Error>;
    fn flush(&mut self) -> ::core::result::Result<(), Self::Error>;
}

#[cfg(all(feature = "std", not(feature = "use-core2")))]
mod std_impl {
    use super::{Read, Write};

    impl<R: ::std::io::Read> Read for R {
        type Error = ::std::io::Error;

        fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
            <Self as ::std::io::Read>::read(self, buf)
        }

        fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), Self::Error> {
            <Self as ::std::io::Read>::read_exact(self, buf)
        }
    }

    impl<W: ::std::io::Write> Write for W {
        type Error = ::std::io::Error;

        fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
            <Self as ::std::io::Write>::write(self, buf)
        }

        fn write_all(&mut self, buf: &[u8]) -> Result<(), Self::Error> {
            <Self as ::std::io::Write>::write_all(self, buf)
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
            <Self as core2::io::Read>::read(self, buf)
        }

        fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), Self::Error> {
            <Self as core2::io::Read>::read_exact(self, buf)
        }
    }

    impl<W: core2::io::Write> Write for W {
        type Error = core2::io::Error;

        fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
            <Self as core2::io::Write>::write(self, buf)
        }

        fn write_all(&mut self, buf: &[u8]) -> Result<(), Self::Error> {
            <Self as core2::io::Write>::write_all(self, buf)
        }

        fn flush(&mut self) -> Result<(), Self::Error> {
            <Self as core2::io::Write>::flush(self)
        }
    }
}

#[cfg(all(not(feature = "use-core2"), not(feature = "std")))]
mod default_impl {
    use super::{Read, Write};

    pub enum DefaultError {
        UnexpectedEof,
    }

    impl Read for &[u8] {
        type Error = DefaultError;

        fn read(&mut self, buf: &mut [u8]) -> ::core::result::Result<usize, Self::Error> {
            let amt = ::core::cmp::min(buf.len(), self.len());
            let (a, b) = self.split_at(amt);

            // First check if the amount of bytes we want to read is small:
            // `copy_from_slice` will generally expand to a call to `memcpy`, and
            // for a single byte the overhead is significant.
            if amt == 1 {
                buf[0] = a[0];
            } else {
                buf[..amt].copy_from_slice(a);
            }

            *self = b;
            Ok(amt)
        }

        fn read_exact(&mut self, buf: &mut [u8]) -> ::core::result::Result<(), Self::Error>  {
            if buf.len() > self.len() {
                return Err(Self::Error::UnexpectedEof);
            }
            let (a, b) = self.split_at(buf.len());

            // First check if the amount of bytes we want to read is small:
            // `copy_from_slice` will generally expand to a call to `memcpy`, and
            // for a single byte the overhead is significant.
            if buf.len() == 1 {
                buf[0] = a[0];
            } else {
                buf.copy_from_slice(a);
            }

            *self = b;
            Ok(())
        }
    }

    impl Write for ::alloc::vec::Vec<u8> {
        type Error = DefaultError;

        fn write(&mut self, buf: &[u8]) -> ::core::result::Result<usize, Self::Error> {
            self.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn write_all(&mut self, buf: &[u8]) -> ::core::result::Result<(), Self::Error> {
            self.extend_from_slice(buf);
            Ok(())
        }

        fn flush(&mut self) -> ::core::result::Result<(), Self::Error> {
            Ok(())
        }
    }
}


#[cfg(test)]
mod tests {

    #[cfg(all(feature = "std", not(feature = "use-core2")))]
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
        #[cfg(feature = "use-core2-std")]
        fn test_core2_write_cursor() {
            let mut cursor = core2::io::Cursor::new(vec![]);
            let mut buf = [10u8; 1];
            cursor.write(&mut buf).unwrap();
            assert_eq!(cursor.into_inner(), vec![10u8]);
        }

        #[test]
        fn test_core2_write() {
            let mut write_buf = [0u8; 1];
            let mut buf = [10u8; 1];
            (&mut write_buf[..]).write(&mut buf).unwrap();
            assert_eq!(write_buf, [10u8; 1]);
        }
    }
}
