use rust_kvac::keygen::*;
use unknown_order::BigNumber;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keygen() {
        let (pp, c) = keygen();
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
    fn test_sample_element_with_jacobi() {
        let (pp, _) = keygen();
        let (group, _, _) = pp;

        // Test the sample_element_with_jacobi function
        let g = sample_element_with_jacobi(&group.modulus);
        assert_eq!(
            jacobi(&g, &group.modulus),
            1,
            "Sampled element should have a Jacobi symbol of 1"
        );
    }

    #[test]
    #[should_panic(expected = "k must be positive and odd")]
    fn test_jacobi_with_negative_k() {
        let n = BigNumber::from(5);
        let k = BigNumber::from(-3); // Negative k, should panic

        jacobi(&n, &k);
    }

    #[test]
    #[should_panic(expected = "k must be positive and odd")]
    fn test_jacobi_with_even_k() {
        let n = BigNumber::from(5);
        let k = BigNumber::from(4); // Even k, should panic

        jacobi(&n, &k);
    }

    #[test]
    fn test_keygen_with_invalid_modulus() {
        // Directly test the sample_element_with_jacobi function with an invalid modulus
        let invalid_modulus = BigNumber::from(0); // Invalid modulus

        let result = std::panic::catch_unwind(|| {
            sample_element_with_jacobi(&invalid_modulus);
        });

        assert!(
            result.is_err(),
            "Function should panic with invalid modulus"
        );
    }
}
