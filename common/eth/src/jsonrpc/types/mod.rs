mod block;
mod bytes;
mod log;
mod receipt;
mod trace;
mod transaction;

use serde::de::Error;
use serde::Deserialize;
use serde::Serialize;

pub use self::block::*;
pub use self::bytes::*;
pub use self::log::*;
pub use self::receipt::*;
pub use self::trace::*;
pub use self::transaction::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct StringU64(pub u64);

impl From<u64> for StringU64 {
    fn from(v: u64) -> Self {
        Self(v)
    }
}

impl From<StringU64> for u64 {
    fn from(v: StringU64) -> Self {
        v.0
    }
}

impl Serialize for StringU64 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        serializer.serialize_str(&self.0.to_string())
    }
}

impl<'de> Deserialize<'de> for StringU64 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        <&str>::deserialize(deserializer)
            .and_then(|str| str.parse().map(Self).map_err(D::Error::custom))
    }
}
