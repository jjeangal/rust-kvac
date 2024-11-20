use crate::params::PUBLIC_PARAMS;
use unknown_order::*;

// String not used?
// 1 !!!!!      -----       (string not used?)
pub fn proof_update(
    st: String,
    proof_k: (BigNumber, BigNumber),
    upd: (String, BigNumber),
) -> (BigNumber, BigNumber) {
    // Proof update logic

    let params = &*PUBLIC_PARAMS;

    let z = (params.hash_function)(&upd.0);

    let k1_z = proof_k.0.modpow(&z, &params.group.modulus);
    let k2_upd2 = proof_k.1.modpow(&upd.1, &params.group.modulus);

    let new_k1 = (k1_z.clone() * k2_upd2.clone()) % &params.group.modulus;
    let new_k2 = proof_k.1.modpow(&z, &params.group.modulus);

    (new_k1, new_k2)
}
