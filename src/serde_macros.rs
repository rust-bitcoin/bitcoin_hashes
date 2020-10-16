#[cfg(feature = "serde")]
pub mod serde_details {
    use Error;

    use std::marker::PhantomData;
    struct HexVisitor<ValueT>(PhantomData<ValueT>);
    use serde::{de, Serializer, Deserializer};
    use hex;

    impl<'de, ValueT> de::Visitor<'de> for HexVisitor<ValueT>
    where
        ValueT: hex::FromHex,
    {
        type Value = ValueT;

        fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            formatter.write_str("an ASCII hex string")
        }

        fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            if let Ok(hex) = ::std::str::from_utf8(v) {
                Self::Value::from_hex(hex).map_err(E::custom)
            } else {
                return Err(E::invalid_value(
                    de::Unexpected::Bytes(v),
                    &self,
                ));
            }
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Self::Value::from_hex(v).map_err(E::custom)
        }
    }

    struct BytesVisitor<ValueT>(PhantomData<ValueT>);

    impl<'de, ValueT> de::Visitor<'de> for BytesVisitor<ValueT>
    where ValueT : SerdeHash {
        type Value = ValueT;

        fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            formatter.write_str("a bytestring")
        }

        fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            SerdeHash::from_slice_delegated(v).map_err(|_| {
                // from_slice only errors on incorrect length
                E::invalid_length(v.len(), &stringify!(N))
            })
        }
    }

    pub trait SerdeHash
    where
        Self: Sized
            + hex::ToHex
            + hex::FromHex
            + std::ops::Index<usize, Output = u8>
            + std::ops::Index<std::ops::RangeFull, Output = [u8]>
    {
        const N: usize;

        fn from_slice_delegated(sl: &[u8]) -> Result<Self, Error>;
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            if s.is_human_readable() {
                s.serialize_str(&self.to_hex())
            } else {
                s.serialize_bytes(&self[..])
            }
        }

        fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
            if d.is_human_readable() {
                d.deserialize_str(HexVisitor::<Self>(PhantomData))
            } else {
                d.deserialize_bytes(BytesVisitor::<Self>(PhantomData))
            }
        }
    }
}

#[macro_export]
#[cfg(feature = "serde")]
/// Implements `Serialize` and `Deserialize` for a type `$t` which
/// represents a newtype over a byte-slice over length `$len`
macro_rules! serde_impl(
    ($t:ident, $len:expr) => (
        impl $crate::serde_macros::serde_details::SerdeHash for $t {
            const N : usize = $len;
            fn from_slice_delegated(sl: &[u8]) -> Result<Self, $crate::Error> {
                $t::from_slice(sl)
            }
        }

        impl $crate::serde::Serialize for $t {
            fn serialize<S: $crate::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
                $crate::serde_macros::serde_details::SerdeHash::serialize(self, s)
            }
        }

        impl<'de> $crate::serde::Deserialize<'de> for $t {
            fn deserialize<D: $crate::serde::Deserializer<'de>>(d: D) -> Result<$t, D::Error> {
                $crate::serde_macros::serde_details::SerdeHash::deserialize(d)
            }
        }
));

/// Does an "empty" serde implementation for the configuration without serde feature
#[macro_export]
#[cfg(not(feature = "serde"))]
macro_rules! serde_impl(
        ($t:ident, $len:expr) => ()
);
