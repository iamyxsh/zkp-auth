use num_bigint::BigInt;

use crate::utils;

#[derive(Debug, Default, Clone)]
pub struct ZKP {
    pub alpha: BigInt,
    pub beta: BigInt,
    pub p: BigInt,
    pub q: BigInt,
}

impl ZKP {
    pub fn generate_rand(bound: &BigInt) -> BigInt {
        utils::gen_rand(bound)
    }

    pub fn generate_rand_string(length: usize) -> String {
        utils::gen_rand_string(length)
    }

    pub fn generate_constants() -> (BigInt, BigInt, BigInt, BigInt) {
        utils::generate_constants()
    }
}
