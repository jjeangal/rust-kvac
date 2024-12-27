#[cfg(test)]
mod tests {
    use rust_kvac::common::{error::CryptoError, keygen::*};
    use unknown_order::BigNumber;

    #[test]
    fn test_keygen() {
        let (pp, c) = keygen(1024).unwrap();
        let (group, g, _) = pp;

        assert!(g != BigNumber::zero(), "g should not be zero");

        // Ensure the group modulus is correct
        let modulus = &group.modulus;
        assert!(g < *modulus, "g should be less than the group modulus");

        // Ensure the commitment is correctly generated
        let (c1, c2) = c;
        assert!(c1 == BigNumber::from(1), "c1 should be 1");
        assert!(c2 == g, "c2 should be equal to g");
    }

    #[test]
    fn test_jacobi_with_negative_k() {
        let n = BigNumber::from(5);
        let k = BigNumber::from(-3);
        let result = jacobi(&n, &k);
        assert!(matches!(result, Err(CryptoError::InvalidJacobiInput(_))));
    }

    #[test]
    fn test_jacobi_with_even_k() {
        let n = BigNumber::from(5);
        let k = BigNumber::from(4); // Even k
        let result = jacobi(&n, &k);
        assert!(matches!(result, Err(CryptoError::InvalidJacobiInput(_))));
    }

    #[test]
    fn test_sample_element_with_jacobi() {
        let (pp, _) = keygen(1024).unwrap();
        let (group, _, _) = pp;

        let g = sample_element_with_jacobi_safe(&group.modulus).unwrap();
        assert_eq!(
            jacobi(&g, &group.modulus).unwrap(),
            1,
            "Sampled element should have a Jacobi symbol of 1"
        );
    }

    #[test]
    fn test_keygen_with_invalid_modulus() {
        let invalid_modulus = BigNumber::from(0);
        let result = sample_element_with_jacobi_safe(&invalid_modulus);

        assert!(matches!(result, Err(CryptoError::InvalidGroup(_))));
    }
}
