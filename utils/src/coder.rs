use bincode;
use crypto::digest::Digest;
use crypto::sha3::Sha3;
use serde::{Deserialize, Serialize};

// 序列化器
pub fn my_serialize<T: ?Sized>(value: &T) -> Vec<u8>
    // ?Sized:动态大小
    // 要求泛型可序列化
    where T: Serialize {
    let serialized = bincode::serialize(value).unwrap();
    serialized
}

// 反序列化器
pub fn my_deserialize<'a, T>(bytes: &'a [u8]) -> T
    where T: Deserialize<'a> {
    // 要求泛型可反序列化
    // 反序列化，注意生命周期
    let deserialized = bincode::deserialize(bytes).unwrap();
    deserialized
}

pub fn get_hash(value: &[u8]) -> String {
    // 创建 SHA3-256 对象
    let mut hasher = Sha3::sha3_256();
    // 写入输入消息
    hasher.input(value);
    // 获取哈希摘要并返回
    hasher.result_str()
}

// for test
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

#[cfg(test)]
mod tests {
    use crate::coder::Point;
    use crate::coder::{my_serialize, my_deserialize};

    #[test]
    fn coders_works() {
        let pt = Point { x: 1, y: 2 };
        let se = my_serialize(&pt);
        let de: Point = my_deserialize(&se);
        assert_eq!(de, pt);
    }
}