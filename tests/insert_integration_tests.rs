extern crate rust_kvac;
use rust_kvac::insert::insert;
use rust_kvac::params::PUBLIC_PARAMS;
use unknown_order::BigNumber;

#[test]
fn test_insert_with_empty_key() {
    let params = &*PUBLIC_PARAMS;
    let c = (BigNumber::one(), BigNumber::from(2u32));
    let k = "".to_string(); // Empty key
    let v = BigNumber::from(3u32);

    let (new_c, proof_k, upd) = insert(c.clone(), (k.clone(), v.clone()));

    assert_eq!(upd, (k.clone(), v.clone()));
    assert_eq!(proof_k, c);

    let z = (params.hash_function)(&k.clone());
    let c1_z = c.0.clone().modpow(&z, &params.group.modulus);
    let c2_v = c.1.clone().modpow(&v, &params.group.modulus);
    let c2_z = c.1.clone().modpow(&z, &params.group.modulus);

    // C = (C1^z * C2^v, C2^z)
    let new_c1 = c1_z.clone().modmul(&c2_v.clone(), &params.group.modulus);

    assert_eq!(new_c, (new_c1, c2_z));
}
