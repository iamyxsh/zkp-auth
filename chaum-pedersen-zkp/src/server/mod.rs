use num_bigint::BigInt;

use crate::{models::ZKP, utils};

pub trait ZKPServer {
    fn verify(
        &self,
        r1: &BigInt,
        r2: &BigInt,
        y1: &BigInt,
        y2: &BigInt,
        c: &BigInt,
        s: &BigInt,
    ) -> bool;
}

impl ZKPServer for ZKP {
    fn verify(
        &self,
        r1: &BigInt,
        r2: &BigInt,
        y1: &BigInt,
        y2: &BigInt,
        c: &BigInt,
        s: &BigInt,
    ) -> bool {
        utils::verify(r1, r2, &self.alpha, &self.beta, y1, y2, s, c, &self.p)
    }
}
