use std::str::FromStr;

use chaum_pedersen_zkp::{
    client::ZKPClient,
    models::{self, ZKP},
};
use num_bigint::BigInt;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct GenY {
    y1: String,
    y2: String,
}

#[derive(Serialize, Deserialize)]
pub struct GenR {
    r1: String,
    r2: String,
}

#[derive(Serialize, Deserialize)]
pub struct Solve {
    solve: String,
}

#[wasm_bindgen]
pub fn generate_y(password: &str) -> JsValue {
    let (alpha, beta, p, q) = models::ZKP::generate_constants();
    let zkp = models::ZKP { alpha, beta, p, q };
    let (y1, y2) = zkp.generate_y(password);
    let gen_y = GenY {
        y1: y1.to_string(),
        y2: y2.to_string(),
    };
    JsValue::from_str(&serde_json::to_string(&gen_y).unwrap())
}

#[wasm_bindgen]
pub fn generate_r(k: &str) -> JsValue {
    let (alpha, beta, p, q) = models::ZKP::generate_constants();
    let zkp = models::ZKP { alpha, beta, p, q };
    let (r1, r2) = zkp.generate_r(&BigInt::from_str(k).unwrap());
    let gen_r = GenR {
        r1: r1.to_string(),
        r2: r2.to_string(),
    };
    JsValue::from_str(&serde_json::to_string(&gen_r).unwrap())
}

#[wasm_bindgen]
pub fn find_solve(k: &str, c: &str, password: &str) -> JsValue {
    let (alpha, beta, p, q) = models::ZKP::generate_constants();
    let zkp = models::ZKP { alpha, beta, p, q };
    let solve = zkp.solve(
        &BigInt::from_str(k).unwrap(),
        &BigInt::from_str(c).unwrap(),
        password,
    );

    let solve = Solve {
        solve: solve.to_string(),
    };

    JsValue::from_str(&serde_json::to_string(&solve).unwrap())
}

#[wasm_bindgen]
pub fn gen_random_below() -> JsValue {
    let (_, _, _, q) = models::ZKP::generate_constants();
    let rand = ZKP::generate_rand(&q);

    JsValue::from_str(&rand.to_string())
}
