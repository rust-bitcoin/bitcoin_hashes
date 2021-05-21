// Bitcoin Hashes Library
// Written in 2021 by
//   The rust-bitcoin developers.
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the CC0 Public Domain Dedication
// along with this software.
// If not, see <http://creativecommons.org/publicdomain/zero/1.0/>.
//

//! # Literacy traits
//!
//! literacy: "the ability to [Read] and [Write]."
//!
//! * With `std` enabled, traits are automatically implemented for `std::io::{Read, Write}`
//! * Without `std` but using feature `use-core2`, they are implemented for `core2::io::{Read, Write}`
//! * Without neither `std` and nor `core2` default implementation `impl Read for &[u8]`
//!   and `impl Write for ::alloc::vec::Vec<u8>` are provided
//!

/// The Read trait allows for reading bytes from a source.
pub trait Read {
    /// The error type returned in Result
    type Error;
    /// The type to implement limited reads
    type Take;
    /// see [std::io::Read::read]
    fn read(&mut self, buf: &mut [u8]) -> ::core::result::Result<usize, Self::Error>;
    /// see [std::io::Read::read_exact]
    fn read_exact(&mut self, buf: &mut [u8]) -> ::core::result::Result<(), Self::Error>;
    /// see [std::io::Read::take]
    fn take(self, limit: u64) -> Self::Take;
}

/// The Write trait allows to write bytes in the object implementing it.
pub trait Write {
    /// The error type returned in Result
    type Error;
    /// see [std::io::Write::write]
    fn write(&mut self, buf: &[u8]) -> ::core::result::Result<usize, Self::Error>;
    /// see [std::io::Write::write_all]
    fn write_all(&mut self, buf: &[u8]) -> ::core::result::Result<(), Self::Error>;
    /// see [std::io::Write::flush]
    fn flush(&mut self) -> ::core::result::Result<(), Self::Error>;
}

#[cfg(all(feature = "std", not(feature = "use-core2")))]
mod std_impl {
    use super::{Read, Write};

    impl<R: ::std::io::Read> Read for R {
        type Error = ::std::io::Error;
        type Take = ::std::io::Take<R>;

        fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
            <Self as ::std::io::Read>::read(self, buf)
        }

        fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), Self::Error> {
            <Self as ::std::io::Read>::read_exact(self, buf)
        }

        fn take(self, limit: u64) -> Self::Take {
            <Self as ::std::io::Read>::take(self, limit)
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
        type Take = core2::io::Take<R>;

        fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
            <Self as core2::io::Read>::read(self, buf)
        }

        fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), Self::Error> {
            <Self as core2::io::Read>::read_exact(self, buf)
        }

        fn take(self, limit: u64) -> Self::Take {
            <Self as core2::io::Read>::take(self, limit)
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

    #[derive(Debug)]
    pub enum DefaultError {
        UnexpectedEof,
    }

    impl<'a> Read for &'a [u8] {
        type Error = DefaultError;
        type Take = &'a [u8];

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

        fn take(self, limit: u64) -> Self::Take {
            &self[..limit as usize]
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
