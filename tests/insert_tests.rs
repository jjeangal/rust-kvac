mod common;

#[cfg(test)]
mod tests {
    use crate::common::test_data::*;
    use rust_kvac::{
        insert::insert,
        params::{Commitment, PUBLIC_PARAMS},
    };

    // Test setup
    fn setup_commitment() -> Commitment {
        let params = &*PUBLIC_PARAMS;
        Commitment::new(params.one.clone(), params.g.clone())
    }

    #[test]
    fn test_basic_insert() {
        let commitment = setup_commitment();
        let (new_c, proof_k, upd) = insert(&commitment, &KV1);

        assert_eq!(&upd.key(), &KV1.key());
        assert_eq!(&upd.value(), &KV1.value());
        assert_eq!(proof_k.lambda_k(), &commitment);
        assert_ne!(&new_c, &commitment);
    }

    #[test]
    fn test_insert_with_large_value() {
        let commitment = setup_commitment();
        let (_, proof_k, upd) = insert(&commitment, &LARGE_KV);

        assert_eq!(&upd.key(), &LARGE_KV.key());
        assert_eq!(&upd.value(), &LARGE_KV.value());
        assert_eq!(proof_k.lambda_k(), &commitment);
    }

    #[test]
    fn test_insert_with_zero_value() {
        let commitment = setup_commitment();
        let (_, proof_k, upd) = insert(&commitment, &ZERO_KV);

        assert_eq!(&upd.key(), &ZERO_KV.key());
        assert_eq!(&upd.value(), &ZERO_KV.value());
        assert_eq!(proof_k.lambda_k(), &commitment);
    }

    #[test]
    fn test_insert_with_special_characters() {
        let commitment = setup_commitment();
        let (_, proof_k, upd) = insert(&commitment, &SPECIAL_CHARS_KV);

        assert_eq!(&upd.key(), &SPECIAL_CHARS_KV.key());
        assert_eq!(&upd.value(), &SPECIAL_CHARS_KV.value());
        assert_eq!(proof_k.lambda_k(), &commitment);
    }

    #[test]
    fn test_sequential_inserts() {
        let commitment = setup_commitment();

        // First insert
        let (commitment2, proof1, upd1) = insert(&commitment, &KV1);
        assert_eq!(&upd1.key(), &KV1.key());
        assert_eq!(proof1.lambda_k(), &commitment);

        // Second insert
        let (_, proof2, upd2) = insert(&commitment2, &KV2);
        assert_eq!(&upd2.key(), &KV2.key());
        assert_eq!(proof2.lambda_k(), &commitment2);
    }

    #[test]
    fn test_insert_with_duplicate_key() {
        let commitment = setup_commitment();

        let (commitment2, _, _) = insert(&commitment, &KV1);
        let (_, proof_k, upd2) = insert(&commitment2, &DUPLICATE_KV);

        assert_eq!(&upd2.key(), &DUPLICATE_KV.key());
        assert_eq!(&upd2.value(), &DUPLICATE_KV.value());
        assert_eq!(proof_k.lambda_k(), &commitment2);
    }

    #[test]
    fn test_insert_commitment_changes() {
        let commitment = setup_commitment();

        let (new_commitment, _, _) = insert(&commitment, &KV1);

        assert_ne!(
            &new_commitment, &commitment,
            "Commitment should change after insert"
        );
    }
}
