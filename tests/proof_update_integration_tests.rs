extern crate rust_kvac;

#[cfg(test)]
mod tests {
    use super::*;
    use rust_kvac::insert::insert;
    use rust_kvac::params::PUBLIC_PARAMS;
    use rust_kvac::proof_update::proof_update;
    use unknown_order::BigNumber;

    #[test]
    fn test_proof_update_consistency() {
        // Access the public parameters
        let params = &*PUBLIC_PARAMS;

        // Initial values
        let c = (params.one.clone(), params.g.clone());
        let k = "test".to_string();
        let v = BigNumber::from(8);

        // Insert a key-value pair
        let (.., proof_k, upd) = insert(c, (k.clone(), v.clone()));

        // Update the proof
        let new_proof_k = proof_update(proof_k.clone(), upd.clone());

        // Consistency check: applying the same update should yield the same result
        let new_proof_k_again = proof_update(proof_k.clone(), upd.clone());
        assert_eq!(
            new_proof_k, new_proof_k_again,
            "Proof update should be consistent"
        );
    }
}
