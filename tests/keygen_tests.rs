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
}
