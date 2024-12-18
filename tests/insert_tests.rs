extern crate rust_kvac;

mod tests {
    use super::*;
    use rust_kvac::insert::insert;
    use rust_kvac::params::Commitment;
    use rust_kvac::params::KeyValue;
    use rust_kvac::params::PUBLIC_PARAMS;
    use unknown_order::BigNumber;

    #[test]
    fn test_insert() {
        let params = &*PUBLIC_PARAMS;
        let commitment = Commitment::new(params.one.clone(), params.g.clone());
        let kv: KeyValue = KeyValue::new("non_empty_key".to_string(), BigNumber::from(1));

        let (_, proof_k, upd) = insert(&commitment, &kv);

        assert_eq!(&upd.key(), &kv.key());
        assert_eq!(&upd.value(), &kv.value());
        assert_eq!(proof_k.lambda_k(), &commitment);
    }

    #[test]
    fn test_insert_with_large_value() {
        let params = &*PUBLIC_PARAMS;
        let commitment = Commitment::new(params.one.clone(), params.g.clone());
        let kv: KeyValue =
            KeyValue::new("large_value_key".to_string(), BigNumber::from(1_000_000u32));

        let (_, proof_k, upd) = insert(&commitment, &kv);

        assert_eq!(&upd.key(), &kv.key());
        assert_eq!(&upd.value(), &kv.value());
        assert_eq!(proof_k.lambda_k(), &commitment);
    }

    #[test]
    fn test_insert_with_zero_value() {
        let params = &*PUBLIC_PARAMS;
        let commitment = Commitment::new(params.one.clone(), params.g.clone());
        let kv: KeyValue = KeyValue::new("zero_value_key".to_string(), BigNumber::from(0u32));

        let (_, proof_k, upd) = insert(&commitment, &kv);

        assert_eq!(&upd.key(), &kv.key());
        assert_eq!(&upd.value(), &kv.value());
        assert_eq!(proof_k.lambda_k(), &commitment);
    }
    #[test]
    fn test_insert_with_empty_key() {
        let params = &*PUBLIC_PARAMS;
        let commitment = Commitment::new(params.one.clone(), params.g.clone());
        let kv: KeyValue = KeyValue::new("".to_string(), BigNumber::from(3u32));

        let (_, proof_k, upd) = insert(&commitment, &kv);

        assert_eq!(&upd.key(), &kv.key());
        assert_eq!(&upd.value(), &kv.value());
        assert_eq!(proof_k.lambda_k(), &commitment);
    }

    #[test]
    fn test_insert_with_special_characters_in_key() {
        let params = &*PUBLIC_PARAMS;
        let commitment = Commitment::new(params.one.clone(), params.g.clone());
        let kv: KeyValue = KeyValue::new("!@#$%^&*()_+".to_string(), BigNumber::from(3u32));

        let (_, proof_k, upd) = insert(&commitment, &kv);

        assert_eq!(&upd.key(), &kv.key());
        assert_eq!(&upd.value(), &kv.value());
        assert_eq!(proof_k.lambda_k(), &commitment);
    }

    #[test]
    fn test_insert_with_duplicate_key() {
        let params = &*PUBLIC_PARAMS;
        let commitment = Commitment::new(params.one.clone(), params.g.clone());

        let kv1: KeyValue = KeyValue::new("duplicate_key".to_string(), BigNumber::from(3u32));
        let kv2: KeyValue = KeyValue::new("duplicate_key".to_string(), BigNumber::from(7u32));

        let _ = insert(&commitment, &kv1);
        let (_, proof_k, upd2) = insert(&commitment, &kv2);

        assert_eq!(&upd2.key(), &kv2.key());
        assert_eq!(&upd2.value(), &kv2.value());
        assert_eq!(proof_k.lambda_k(), &commitment);
    }
}
