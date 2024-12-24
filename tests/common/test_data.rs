// tests/common/test_data.rs
use once_cell::sync::Lazy;
use rust_kvac::params::KeyValue;
use unknown_order::BigNumber;

pub static KV1: Lazy<KeyValue> =
    Lazy::new(|| KeyValue::new("test_key".to_string(), BigNumber::from(8)));

pub static KV2: Lazy<KeyValue> =
    Lazy::new(|| KeyValue::new("test_key2".to_string(), BigNumber::from(10)));

pub static LARGE_KV: Lazy<KeyValue> =
    Lazy::new(|| KeyValue::new("large_key".to_string(), BigNumber::from(1_000_000u32)));

pub static ZERO_KV: Lazy<KeyValue> =
    Lazy::new(|| KeyValue::new("zero_key".to_string(), BigNumber::from(0)));

pub static SPECIAL_CHARS_KV: Lazy<KeyValue> =
    Lazy::new(|| KeyValue::new("!@#$%^&*()_+".to_string(), BigNumber::from(3)));

pub static DUPLICATE_KV: Lazy<KeyValue> =
    Lazy::new(|| KeyValue::new(KV1.key().clone(), BigNumber::from(42)));
