use once_cell::sync::Lazy;
use rust_kvac::params::*;
use unknown_order::BigNumber;

// Pre-generated test vectors with different sizes
pub static KEYS_1K: Lazy<Vec<KeyValue>> = Lazy::new(|| generate_key_values(1_000));

// Same key, different values
pub static SAME_KEY_VALUES_10K: Lazy<Vec<KeyValue>> =
    Lazy::new(|| generate_same_key_values(10_000));

pub fn generate_key_values(size: usize) -> Vec<KeyValue> {
    (1..=size)
        .map(|i| KeyValue::new(format!("key_{}", i), BigNumber::from(i)))
        .collect()
}

pub fn generate_same_key_values(size: usize) -> Vec<KeyValue> {
    let key = "test_key".to_string();
    (1..=size)
        .map(|i| KeyValue::new(key.clone(), BigNumber::from(i)))
        .collect()
}

pub fn initial_commitment() -> Commitment {
    let pp = &PUBLIC_PARAMS;
    Commitment::new(pp.one.clone(), pp.g.clone())
}
