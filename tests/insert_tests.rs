extern crate rust_kvac;
use rust_kvac::insert::insert;
use rust_kvac::params::Commitment;
use rust_kvac::params::PUBLIC_PARAMS;
use unknown_order::BigNumber;

#[test]
fn test_insert() {
    let params = &*PUBLIC_PARAMS;
    let commitment = Commitment::new(params.one.clone(), params.g.clone());
    let key = "non_empty_key".to_string();
    let value = BigNumber::from(5u32);

    let (_, proof_k, upd) = insert(commitment.clone(), (key.clone(), value.clone()));

    assert_eq!(upd.key(), &key);
    assert_eq!(upd.value(), &value);
    assert_eq!(proof_k.first(), &commitment);
}

#[test]
fn test_insert_with_large_value() {
    let params = &*PUBLIC_PARAMS;
    let commitment = Commitment::new(params.one.clone(), params.g.clone());
    let key = "large_value_key".to_string();
    let value = BigNumber::from(1_000_000u32);

    let (_, proof_k, upd) = insert(commitment.clone(), (key.clone(), value.clone()));

    assert_eq!(upd.key(), &key);
    assert_eq!(upd.value(), &value);
    assert_eq!(proof_k.first(), &commitment);
}

#[test]
fn test_insert_with_zero_value() {
    let params = &*PUBLIC_PARAMS;
    let commitment = Commitment::new(params.one.clone(), params.g.clone());
    let key = "zero_value_key".to_string();
    let value = BigNumber::from(0u32);

    let (_, proof_k, upd) = insert(commitment.clone(), (key.clone(), value.clone()));

    assert_eq!(upd.key(), &key);
    assert_eq!(upd.value(), &value);
    assert_eq!(proof_k.first(), &commitment);
}
#[test]
fn test_insert_with_empty_key() {
    let params = &*PUBLIC_PARAMS;
    let commitment = Commitment::new(params.one.clone(), params.g.clone());
    let key = "".to_string();
    let value = BigNumber::from(3u32);

    let (_, proof_k, upd) = insert(commitment.clone(), (key.clone(), value.clone()));

    assert_eq!(upd.key(), &key);
    assert_eq!(upd.value(), &value);
    assert_eq!(proof_k.first(), &commitment);
}

#[test]
fn test_insert_with_special_characters_in_key() {
    let params = &*PUBLIC_PARAMS;
    let commitment = Commitment::new(params.one.clone(), params.g.clone());
    let key = "!@#$%^&*()_+".to_string();
    let value = BigNumber::from(3u32);

    let (_, proof_k, upd) = insert(commitment.clone(), (key.clone(), value.clone()));

    assert_eq!(upd.key(), &key);
    assert_eq!(upd.value(), &value);
    assert_eq!(proof_k.first(), &commitment);
}

#[test]
fn test_insert_with_duplicate_key() {
    let params = &*PUBLIC_PARAMS;
    let commitment = Commitment::new(params.one.clone(), params.g.clone());
    let key = "duplicate_key".to_string();
    let value1 = BigNumber::from(3u32);
    let value2 = BigNumber::from(7u32);

    let _ = insert(commitment.clone(), (key.clone(), value1.clone()));
    let (_, proof_k, upd2) = insert(commitment.clone(), (key.clone(), value2.clone()));

    assert_eq!(upd2.key(), &key);
    assert_eq!(upd2.value(), &value2);
    assert_eq!(proof_k.first(), &commitment);
}
