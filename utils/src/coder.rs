use bincode;
pub use serde::{Deserialize, Serialize};

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

#[cfg(test)]
mod coder_test {
    use crate::coder::*;
    use crate::test::*;

    #[test]
    fn coder_work() {
        let point = Point { x: 1, y: 1 };
        let re = my_serialize(&point);
        let de: Point = my_deserialzie(&re);
        assert_eq!(point, de);
    }
}
