use crate::params::PUBLIC_PARAMS;
use unknown_order::BigNumber;

pub fn verify(
    c: (BigNumber, BigNumber),
    (k, v): (String, BigNumber),
    proof_k: (BigNumber, BigNumber),
) -> bool {
    // Verify the proof Logic

    let params = &*PUBLIC_PARAMS;

    let z = (params.hash_function)(&k);

    let k2_z = proof_k.1.modpow(&z, &params.group.modulus);

    let k1_z = proof_k.0.modpow(&z, &params.group.modulus);
    let k2_v = proof_k.1.modpow(&v, &params.group.modulus);

    println!("k2_z: {:?}", k2_z);
    println!("c.1: {:?}", c.1);

    if ((k1_z * k2_v) % &params.group.modulus) != c.0 {
        println!("k1_z * k2_v != c.0");
        return false;
    }

    if k2_z != c.1 {
        println!("k2_z != c.1");
        return false;
    }

    true
}
