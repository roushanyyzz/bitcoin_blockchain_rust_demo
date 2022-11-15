use crypto::{digest::Digest, sha3::Sha3};

pub fn get_hash(value: &[u8]) -> String {
    let mut hasher = Sha3::sha3_256();
    hasher.input(value);
    hasher.result_str()
}

#[cfg(test)]
mod hash_test {
    use super::*;
    use crate::coder::my_serialize;
    use crate::test::*;

    #[test]
    fn hash_work() {
        let point = _get_point();
        let v = my_serialize(&point);
        let result = get_hash(&v);
        println!("{}", result);
    }
}
