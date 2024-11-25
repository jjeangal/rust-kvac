use crate::params::PUBLIC_PARAMS;
use unknown_order::*;

// String not used

/// Updates the proof for a given key and update information.
///
/// # Arguments
///
/// * `proof_k` - A tuple representing the current proof components.
/// * `upd` - A tuple containing the key (as a string) and the update value.
///
/// # Returns
///
/// A tuple containing the updated proof components.
pub fn proof_update(
    proof_k: (BigNumber, BigNumber),
    upd: (String, BigNumber),
) -> (BigNumber, BigNumber) {
    let params = &*PUBLIC_PARAMS;

    // Compute the hash of the key
    let z = (params.hash_function)(&upd.0);

    // Compute the new proof components
    let k1_z = proof_k.0.modpow(&z, &params.group.modulus);
    let k2_upd2 = proof_k.1.modpow(&upd.1, &params.group.modulus);

    let new_k1 = k1_z.modmul(&k2_upd2, &params.group.modulus);
    let new_k2 = proof_k.1.modpow(&z, &params.group.modulus);

    // Return the updated proof components
    (new_k1, new_k2)
}
