use common::TypeInfo;
use init::coin::Coin;
use serde::de::{Deserialize, Deserializer, MapAccess, Visitor};
use serde::ser::{Serialize, SerializeStruct, Serializer};
use std::fmt;
use tx::data::address::ExtendedAddr;
use tx::data::Tx;

/// Tx Output composed of an address and a coin value
/// TODO: timelock?
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TxOut {
    pub address: ExtendedAddr,
    pub value: Coin,
}

impl fmt::Display for TxOut {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} -> {}", self.address, self.value)
    }
}

impl TypeInfo for TxOut {
    #[inline]
    fn type_name() -> &'static str {
        "TxOut"
    }
}

/// TODO: switch to cbor_event or equivalent simple raw cbor library when serialization is finalized
/// TODO: backwards/forwards-compatible serialization (adding/removing fields, variants etc. should be possible)
impl Serialize for TxOut {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct(Tx::type_name(), 2)?;
        s.serialize_field("address", &self.address)?;
        s.serialize_field("value", &self.value)?;
        s.end()
    }
}

impl<'de> Deserialize<'de> for TxOut {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TxOutVisitor;

        impl<'de> Visitor<'de> for TxOutVisitor {
            type Value = TxOut;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("transaction output")
            }

            #[inline]
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let address = match map.next_entry::<u64, ExtendedAddr>()? {
                    Some((0, v)) => v,
                    _ => return Err(serde::de::Error::missing_field("address")),
                };
                let value = match map.next_entry::<u64, Coin>()? {
                    Some((1, v)) => v,
                    _ => return Err(serde::de::Error::missing_field("value")),
                };
                Ok(TxOut::new(address, value))
            }
        }

        deserializer.deserialize_struct(TxOut::type_name(), &["address", "value"], TxOutVisitor)
    }
}

impl TxOut {
    /// creates a TX output (mainly for testing/tools)
    pub fn new(address: ExtendedAddr, value: Coin) -> Self {
        TxOut { address, value }
    }
}