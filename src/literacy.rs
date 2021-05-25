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

#[cfg(feature = "std")]
use std::boxed::Box as AllocBox;

#[cfg(not(feature = "std"))]
use alloc::boxed::Box as AllocBox;

/// The Read trait allows for reading bytes from a source.
pub trait Read {
    /// The error type returned in Result
    type Error: ErrorTrait;

    /// see [std::io::Read::read]
    fn read(&mut self, buf: &mut [u8]) -> ::core::result::Result<usize, Self::Error>;

    /// see [std::io::Read::read_exact]
    fn read_exact(&mut self, buf: &mut [u8]) -> ::core::result::Result<(), Self::Error>;
}

/// The Write trait allows to write bytes in the object implementing it.
pub trait Write {
    /// The error type returned in Result
    type Error: ErrorTrait;

    /// see [std::io::Write::write]
    fn write(&mut self, buf: &[u8]) -> ::core::result::Result<usize, Self::Error>;

    /// see [std::io::Write::write_all]
    fn write_all(&mut self, buf: &[u8]) -> ::core::result::Result<(), Self::Error>;

    /// see [std::io::Write::flush]
    fn flush(&mut self) -> ::core::result::Result<(), Self::Error>;
}

/// The literacy Error trait, custom errors must implement this
pub trait ErrorTrait {
    /// The error category
    fn kind(&self) -> ErrorKind;
}

/// Same as [std::io::ErrorKind] that we have to duplicate because ErrorKind is not in `core`
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(missing_docs)]
pub enum ErrorKind {
    NotFound,
    PermissionDenied,
    ConnectionRefused,
    ConnectionReset,
    ConnectionAborted,
    NotConnected,
    AddrInUse,
    AddrNotAvailable,
    BrokenPipe,
    AlreadyExists,
    WouldBlock,
    InvalidInput,
    InvalidData,
    TimedOut,
    WriteZero,
    Interrupted,
    Other,
    UnexpectedEof,
}

/// The Error type in case we are not using [std::io::Error] or [core2::io::Error]
#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,

    error: AllocBox<dyn InnerError>,
}

trait InnerError: ::core::fmt::Debug + ::core::any::Any {}

impl InnerError for () {}

impl ErrorTrait for Error {
    fn kind(&self) -> ErrorKind {
        self.kind
    }
}

#[cfg(all(feature = "std", not(feature = "use-core2")))]
impl ErrorTrait for ::std::io::Error {
    fn kind(&self) -> ErrorKind {
        match self.kind() {
            ::std::io::ErrorKind::NotFound => ErrorKind::NotFound,
            ::std::io::ErrorKind::PermissionDenied => ErrorKind::PermissionDenied,
            ::std::io::ErrorKind::ConnectionRefused => ErrorKind::ConnectionRefused,
            ::std::io::ErrorKind::ConnectionReset => ErrorKind::ConnectionReset,
            ::std::io::ErrorKind::ConnectionAborted => ErrorKind::ConnectionAborted,
            ::std::io::ErrorKind::NotConnected => ErrorKind::NotConnected,
            ::std::io::ErrorKind::AddrInUse => ErrorKind::AddrInUse,
            ::std::io::ErrorKind::AddrNotAvailable => ErrorKind::AddrNotAvailable,
            ::std::io::ErrorKind::BrokenPipe => ErrorKind::BrokenPipe,
            ::std::io::ErrorKind::AlreadyExists => ErrorKind::AlreadyExists,
            ::std::io::ErrorKind::WouldBlock => ErrorKind::WouldBlock,
            ::std::io::ErrorKind::InvalidInput => ErrorKind::InvalidInput,
            ::std::io::ErrorKind::InvalidData => ErrorKind::InvalidData,
            ::std::io::ErrorKind::TimedOut => ErrorKind::TimedOut,
            ::std::io::ErrorKind::WriteZero => ErrorKind::WriteZero,
            ::std::io::ErrorKind::Interrupted => ErrorKind::Interrupted,
            ::std::io::ErrorKind::Other => ErrorKind::Other,
            ::std::io::ErrorKind::UnexpectedEof => ErrorKind::UnexpectedEof,
            _ => ErrorKind::Other,
        }
    }
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
impl ErrorTrait for core2::io::Error {
    fn kind(&self) -> ErrorKind {
        match self.kind() {
            core2::io::ErrorKind::NotFound => ErrorKind::NotFound,
            core2::io::ErrorKind::PermissionDenied => ErrorKind::PermissionDenied,
            core2::io::ErrorKind::ConnectionRefused => ErrorKind::ConnectionRefused,
            core2::io::ErrorKind::ConnectionReset => ErrorKind::ConnectionReset,
            core2::io::ErrorKind::ConnectionAborted => ErrorKind::ConnectionAborted,
            core2::io::ErrorKind::NotConnected => ErrorKind::NotConnected,
            core2::io::ErrorKind::AddrInUse => ErrorKind::AddrInUse,
            core2::io::ErrorKind::AddrNotAvailable => ErrorKind::AddrNotAvailable,
            core2::io::ErrorKind::BrokenPipe => ErrorKind::BrokenPipe,
            core2::io::ErrorKind::AlreadyExists => ErrorKind::AlreadyExists,
            core2::io::ErrorKind::WouldBlock => ErrorKind::WouldBlock,
            core2::io::ErrorKind::InvalidInput => ErrorKind::InvalidInput,
            core2::io::ErrorKind::InvalidData => ErrorKind::InvalidData,
            core2::io::ErrorKind::TimedOut => ErrorKind::TimedOut,
            core2::io::ErrorKind::WriteZero => ErrorKind::WriteZero,
            core2::io::ErrorKind::Interrupted => ErrorKind::Interrupted,
            core2::io::ErrorKind::Other => ErrorKind::Other,
            core2::io::ErrorKind::UnexpectedEof => ErrorKind::UnexpectedEof,
            _ => ErrorKind::Other,
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
    use super::{Read, Write, Error, ErrorKind};

    impl<'a> Read for &'a [u8] {
        type Error = Error;

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
                return Err( Self::Error {
                    kind: ErrorKind::UnexpectedEof,
                    error: alloc::boxed::Box::new(()),
                });
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
        type Error = Error;

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
