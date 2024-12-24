mod common;

#[cfg(test)]
mod tests {
    use super::common::context::TestContext;
    use rust_kvac::{insert::insert, verify::verify};

    #[test]
    fn test_verify_insert() {
        let ctx = TestContext::setup();
        let (new_c, proof_k, op) = insert(&ctx.initial_commitment, &ctx.kv1);

        let result = verify(&new_c, &op, &proof_k);
        assert!(
            result.is_ok(),
            "Verification should succeed with generated data"
        );
    }

    #[test]
    fn test_verify_with_wrong_key() {
        let ctx = TestContext::setup();
        let (_, proof_k, _) = insert(&ctx.initial_commitment, &ctx.kv1);

        let result = verify(&ctx.initial_commitment, &ctx.kv3, &proof_k);
        assert!(result.is_err(), "Verification should fail with wrong key");
    }

    #[test]
    fn test_verify_with_invalid_commitment() {
        let ctx = TestContext::setup();
        let invalid_commitment = TestContext::create_invalid_commitment();
        let dummy_proof = TestContext::create_dummy_proof(&ctx.initial_commitment);

        let result = verify(&invalid_commitment, &ctx.kv2, &dummy_proof);
        assert!(
            result.is_err(),
            "Verification should fail with invalid commitment"
        );
    }

    #[test]
    fn test_verify_with_invalid_proof() {
        let ctx = TestContext::setup();
        let invalid_commitment = TestContext::create_invalid_commitment();
        let invalid_proof = TestContext::create_dummy_proof(&invalid_commitment);

        let result = verify(&ctx.initial_commitment, &ctx.kv3, &invalid_proof);
        assert!(
            result.is_err(),
            "Verification should fail with invalid proof"
        );
    }

    #[test]
    fn test_verify_with_invalid_key_value_pair() {
        let ctx = TestContext::setup();
        let invalid_kv = TestContext::create_invalid_kv();
        let dummy_proof = TestContext::create_dummy_proof(&ctx.initial_commitment);

        let result = verify(&ctx.initial_commitment, &invalid_kv, &dummy_proof);
        assert!(
            result.is_err(),
            "Verification should fail with invalid key-value pair"
        );
    }

    #[test]
    fn test_verify_with_wrong_commitment_proof_pair() {
        let ctx = TestContext::setup();
        let (c1, _, _) = insert(&ctx.initial_commitment, &ctx.kv1);
        let (_, proof2, _) = insert(&ctx.initial_commitment, &ctx.kv2);

        let result = verify(&c1, &ctx.kv2, &proof2);
        assert!(
            result.is_err(),
            "Verification should fail with mismatched commitment-proof pair"
        );
    }
}
