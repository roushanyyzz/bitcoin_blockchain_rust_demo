use bincode;
use serde::{Deserialize, Serialize};

pub fn my_serialize<T>(value: &T) -> Vec<u8>
where
    T: Serialize,
{
    bincode::serialize(value).unwrap()
}

pub fn my_deserialzie<'a, T>(bytes: &'a [u8]) -> T
where
    T: Deserialize<'a>,
{
    bincode::deserialize(bytes).unwrap()
}
