use crate::params::PUBLIC_PARAMS;
use unknown_order::*;

pub fn insert(
    c: (BigNumber, BigNumber),
    (k, v): (String, BigNumber),
) -> (
    (BigNumber, BigNumber),
    (BigNumber, BigNumber),
    (String, BigNumber),
) {
    let params = &*PUBLIC_PARAMS;

    // Calculate z using the hash function
    let z = (params.hash_function)(&k);

    // Calculate C1^z mod modulus and C2^v mod modulus
    let c1_z = c.0.modpow(&z, &params.group.modulus);
    let c2_v = c.1.modpow(&v, &params.group.modulus);

    // C = (C1^z * C2^v, C2^z)
    let new_c1 = (c1_z.clone() * c2_v.clone()) % &params.group.modulus;
    let new_c2 = c.1.modpow(&z, &params.group.modulus);

    let new_c = (new_c1, new_c2);
    let proof_k = c;
    let upd = (k, v);

    (new_c, proof_k, upd)
}
