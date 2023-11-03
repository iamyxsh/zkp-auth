use num_bigint::{BigInt, Sign};

use crate::{
    models::ZKP,
    utils::{self, exponetiate, find_solve},
};

pub trait ZKPClient {
    fn generate_y(&self, password: &str) -> (BigInt, BigInt);
    fn generate_r(&self, k: &BigInt) -> (BigInt, BigInt);
    fn solve(&self, k: &BigInt, c: &BigInt, password: &str) -> BigInt;
}

impl ZKPClient for ZKP {
    fn generate_y(&self, password: &str) -> (BigInt, BigInt) {
        (
            utils::exponetiate(
                &self.alpha,
                &BigInt::from_bytes_be(Sign::NoSign, password.as_bytes()),
                &self.p,
            ),
            utils::exponetiate(
                &self.beta,
                &BigInt::from_bytes_be(Sign::NoSign, password.as_bytes()),
                &self.p,
            ),
        )
    }

    fn generate_r(&self, k: &BigInt) -> (BigInt, BigInt) {
        (
            exponetiate(&self.alpha, k, &self.p),
            exponetiate(&self.beta, k, &self.p),
        )
    }

    fn solve(&self, k: &BigInt, c: &BigInt, password: &str) -> BigInt {
        find_solve(
            k,
            c,
            &BigInt::from_bytes_be(Sign::NoSign, password.as_bytes()),
            &self.q,
        )
    }
}
