
#[cfg(feature="serde")]
/// Implements `Serialize` and `Deserialize` for a type `$t` which
/// represents a newtype over a byte-slice over length `$len`.
macro_rules! serde_impl(
    ($t:ident, $len:expr) => (
        impl ::serde::Serialize for $t {
            fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
                use hex::ToHex;
                if s.is_human_readable() {
                    s.serialize_str(&self.to_hex())
                } else {
                    s.serialize_bytes(&self[..])
                }
            }
        }

        impl<'de> ::serde::Deserialize<'de> for $t {
            fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<$t, D::Error> {
                use ::serde::de::Error;
                use hex::FromHex;

                if d.is_human_readable() {
                    let sl: String = ::serde::Deserialize::deserialize(d)?;
                    $t::from_hex(&sl).map_err(D::Error::custom)
                } else {
                    struct BytesVisitor;

                    impl<'de> ::serde::de::Visitor<'de> for BytesVisitor {
                        type Value = $t;

                        fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                            formatter.write_str("a bytestring")
                        }

                        fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
                        where
                            E: ::serde::de::Error,
                        {
                            if v.len() != $t::LEN {
                                Err(E::invalid_length(v.len(), &stringify!($len)))
                            } else {
                                let mut ret = [0; $len];
                                ret.copy_from_slice(v);
                                Ok($t(ret))
                            }
                        }
                    }

                    d.deserialize_bytes(BytesVisitor)
                }
            }
        }
    )
);

#[cfg(not(feature="serde"))]
macro_rules! serde_impl(
    ($t:ident, $len:expr) => ()
);
