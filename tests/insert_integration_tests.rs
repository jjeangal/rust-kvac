extern crate rust_kvac;
use rust_kvac::insert::insert;
use rust_kvac::params::PUBLIC_PARAMS;
use unknown_order::BigNumber;

#[test]
fn test_insert() {
    let params = &*PUBLIC_PARAMS;
    let c = (params.one.clone(), params.g.clone());
    let k = "non_empty_key".to_string();
    let v = BigNumber::from(5u32);

    let (_, proof_k, upd) = insert(c.clone(), (k.clone(), v.clone()));

    assert_eq!(upd, (k.clone(), v.clone()));
    assert_eq!(proof_k, c);
}

#[test]
fn test_insert_with_large_value() {
    let params = &*PUBLIC_PARAMS;
    let c = (params.one.clone(), params.g.clone());
    let k = "large_value_key".to_string();
    let v = BigNumber::from(1_000_000u32);

    let (_, proof_k, upd) = insert(c.clone(), (k.clone(), v.clone()));

    assert_eq!(upd, (k.clone(), v.clone()));
    assert_eq!(proof_k, c);
}

#[test]
fn test_insert_with_zero_value() {
    let params = &*PUBLIC_PARAMS;
    let c = (params.one.clone(), params.g.clone());
    let k = "zero_value_key".to_string();
    let v = BigNumber::from(0u32);

    let (_, proof_k, upd) = insert(c.clone(), (k.clone(), v.clone()));

    assert_eq!(upd, (k.clone(), v.clone()));
    assert_eq!(proof_k, c);
}
#[test]
fn test_insert_with_empty_key() {
    let params = &*PUBLIC_PARAMS;
    let c = (params.one.clone(), params.g.clone());
    let k = "".to_string();
    let v = BigNumber::from(3u32);

    let (_, proof_k, upd) = insert(c.clone(), (k.clone(), v.clone()));

    assert_eq!(upd, (k.clone(), v.clone()));
    assert_eq!(proof_k, c);
}

#[test]
fn test_insert_with_special_characters_in_key() {
    let params = &*PUBLIC_PARAMS;
    let c = (params.one.clone(), params.g.clone());
    let k = "!@#$%^&*()_+".to_string();
    let v = BigNumber::from(3u32);

    let (_, proof_k, upd) = insert(c.clone(), (k.clone(), v.clone()));

    assert_eq!(upd, (k.clone(), v.clone()));
    assert_eq!(proof_k, c);
}

#[test]
fn test_insert_with_duplicate_key() {
    let params = &*PUBLIC_PARAMS;
    let c = (params.one.clone(), params.g.clone());
    let k = "duplicate_key".to_string();
    let v1 = BigNumber::from(3u32);
    let v2 = BigNumber::from(7u32);

    let _ = insert(c.clone(), (k.clone(), v1.clone()));
    let (_, proof_k, upd2) = insert(c.clone(), (k.clone(), v2.clone()));

    assert_eq!(upd2, (k.clone(), v2.clone()));
    assert_eq!(proof_k, c);
}
