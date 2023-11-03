use num_bigint::{BigInt, RandBigInt};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

/// n ^ exp mod p
pub fn exponetiate(n: &BigInt, exp: &BigInt, p: &BigInt) -> BigInt {
    n.modpow(exp, p)
}

/// s = k - c * x mod q
pub fn find_solve(k: &BigInt, c: &BigInt, x: &BigInt, q: &BigInt) -> BigInt {
    if *k >= c * x {
        (k - c * x).modpow(&BigInt::from(1u32), q)
    } else {
        q - (c * x - k).modpow(&BigInt::from(1u32), q)
    }
}

pub fn gen_rand(bound: &BigInt) -> BigInt {
    rand::thread_rng().gen_bigint_range(&BigInt::from(0), bound)
}

pub fn gen_rand_string(length: usize) -> String {
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect();
    rand_string
}

/// r1 = alpha ^ s * y1 ^ c
/// r2 = beta ^ s * y2 ^ c
pub fn verify(
    r1: &BigInt,
    r2: &BigInt,
    alpha: &BigInt,
    beta: &BigInt,
    y1: &BigInt,
    y2: &BigInt,
    s: &BigInt,
    c: &BigInt,
    p: &BigInt,
) -> bool {
    *r1 == (alpha.modpow(s, p) * y1.modpow(c, p)).modpow(&BigInt::from(1u32), &p)
        && *r2 == (beta.modpow(s, p) * y2.modpow(c, p)).modpow(&BigInt::from(1u32), &p)
}

pub fn generate_constants() -> (BigInt, BigInt, BigInt, BigInt) {
    let alpha = BigInt::from(4u32);
    let beta = BigInt::from(9u32);
    let p = BigInt::from(23u32);
    let q = BigInt::from(11u32);

    (alpha, beta, p, q)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;

    #[test]
    fn it_exponetiates() {
        let result = exponetiate(&BigInt::from(3), &BigInt::from(1), &BigInt::from(2));
        assert_eq!(result.to_string(), "1");
    }

    #[test]
    fn it_finds_solution() {
        let result = find_solve(
            &BigInt::from(3),
            &BigInt::from(1),
            &BigInt::from(2),
            &BigInt::from(2),
        );
        assert_eq!(result.to_string(), "1");
    }

    #[test]
    fn it_gens_rand() {
        let rand = gen_rand(&BigInt::from(10));
        assert!(rand < BigInt::from(10))
    }

    #[test]
    fn it_verifies() {
        let (alpha, beta, p, q) = generate_constants();
        let x = BigInt::from(6u32);
        let k = gen_rand(&q);
        let c = gen_rand(&q);
        let y1 = exponetiate(&alpha, &x, &p);
        let y2 = exponetiate(&beta, &x, &p);
        let r1 = exponetiate(&alpha, &k, &p);
        let r2 = exponetiate(&beta, &k, &p);
        let s = find_solve(&k, &c, &x, &q);

        let result = verify(&r1, &r2, &alpha, &beta, &y1, &y2, &s, &c, &p);
        assert_eq!(result, true);
    }
}
