use rust_kvac::params::{Commitment, KeyValue, Proof, PublicParameters, PUBLIC_PARAMS};
use unknown_order::BigNumber;

pub struct TestContext {
    pub params: &'static PublicParameters,
    pub initial_commitment: Commitment,
    pub kv1: KeyValue,
    pub kv2: KeyValue,
    pub kv3: KeyValue,
}

impl TestContext {
    pub fn setup() -> Self {
        let params = &*PUBLIC_PARAMS;
        let initial_commitment = Commitment::new(params.one.clone(), params.g.clone());

        Self {
            params,
            initial_commitment,
            kv1: KeyValue::new("test".to_string(), BigNumber::from(8)),
            kv2: KeyValue::new("test".to_string(), BigNumber::from(10)),
            kv3: KeyValue::new("test".to_string(), BigNumber::from(18)),
        }
    }

    pub fn create_invalid_kv() -> KeyValue {
        KeyValue::new("invalid".to_string(), BigNumber::from(-1))
    }

    pub fn create_invalid_commitment() -> Commitment {
        Commitment::new(BigNumber::from(0), BigNumber::from(0))
    }

    pub fn create_dummy_proof(commitment: &Commitment) -> Proof {
        Proof::new(
            commitment.clone(),
            (BigNumber::from(1), BigNumber::from(1), BigNumber::from(1)),
            BigNumber::from(1),
        )
    }
}
