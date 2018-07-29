
#[cfg(feature="serde")]
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
                    let sl: &str = ::serde::Deserialize::deserialize(d)?;
                    $t::from_hex(sl).map_err(D::Error::custom)
                } else {
                    let sl: &[u8] = ::serde::Deserialize::deserialize(d)?;
                    if sl.len() != $t::len() {
                        Err(D::Error::invalid_length(sl.len(), &stringify!($len)))
                    } else {
                        let mut ret = [0; $len];
                        ret.copy_from_slice(sl);
                        Ok($t(ret))
                    }
                }
            }
        }
    )
);

#[cfg(not(feature="serde"))]
macro_rules! serde_impl(
    ($t:ident, $len:expr) => ()
);
