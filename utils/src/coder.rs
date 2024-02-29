use bincode;
use serde::{Deserialize, Serialize};
use crypto::digest::Digest;
use crypto::sha3::Sha3;

pub fn my_serialize<T: ?Sized>(value: &T) -> Vec<u8>
where
    T: Serialize,
{
   let serialized = bincode::serialize(value).unwrap();
   serialized
}

pub fn my_deserialize<'a, T>(bytes: &'a [u8]) -> T
where
    T: Deserialize<'a>,
{
    let deserialized: T = bincode::deserialize(bytes).unwrap();
    deserialized
}

pub fn get_hash(data: &[u8]) -> String {
    let mut hasher = Sha3::sha3_256();
    hasher.input(data);
    hasher.result_str()
}   

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

#[cfg(test)]
mod tests {
    use crate::coder::Point;
    use crate::coder::{my_deserialize, my_serialize};

    #[test]
    fn codeers_works() {
        let point = Point{x: 1, y: 2};
        let serialized = my_serialize(&point);
        let deserialized = my_deserialize::<Point>(&serialized);
        assert_eq!(point, deserialized);
    }
}