
#[macro_export]
#[cfg(feature="serde")]
/// Implements `Serialize` and `Deserialize` for a type `$t` which
/// represents a newtype over a byte-slice over length `$len`.
macro_rules! serde_impl(
    ($t:ident, $len:expr) => (
        impl $crate::serde::Serialize for $t {
            fn serialize<S: $crate::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
                use $crate::hex::ToHex;
                if s.is_human_readable() {
                    s.serialize_str(&self.to_hex())
                } else {
                    s.serialize_bytes(&self[..])
                }
            }
        }

        impl<'de> $crate::serde::Deserialize<'de> for $t {
            fn deserialize<D: $crate::serde::Deserializer<'de>>(d: D) -> Result<$t, D::Error> {
                use $crate::hex::FromHex;

                if d.is_human_readable() {
                    struct HexVisitor;

                    impl<'de> $crate::serde::de::Visitor<'de> for HexVisitor {
                        type Value = $t;

                        fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                            formatter.write_str("an ASCII hex string")
                        }

                        fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
                        where
                            E: $crate::serde::de::Error,
                        {
                            if let Ok(hex) = ::std::str::from_utf8(v) {
                                $t::from_hex(hex).map_err(E::custom)
                            } else {
                                return Err(E::invalid_value($crate::serde::de::Unexpected::Bytes(v), &self));
                            }
                        }

                        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                        where
                            E: $crate::serde::de::Error,
                        {
                            $t::from_hex(v).map_err(E::custom)
                        }
                    }

                    d.deserialize_str(HexVisitor)
                } else {
                    struct BytesVisitor;

                    impl<'de> $crate::serde::de::Visitor<'de> for BytesVisitor {
                        type Value = $t;

                        fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                            formatter.write_str("a bytestring")
                        }

                        fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
                        where
                            E: $crate::serde::de::Error,
                        {
                            $t::from_slice(v).map_err(|_| {
                                // from_slice only errors on incorrect length
                                E::invalid_length(v.len(), &stringify!($len))
                            })
                        }
                    }

                    d.deserialize_bytes(BytesVisitor)
                }
            }
        }
    )
);

/// Does an "empty" serde implementation for the configuration without serde feature
#[macro_export]
#[cfg(not(feature="serde"))]
macro_rules! serde_impl(
    ($t:ident, $len:expr) => ()
);
