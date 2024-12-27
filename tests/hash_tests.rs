#[cfg(test)]
mod tests {
    use rust_kvac::common::group_hash::*;
    use unknown_order::*;

    #[test]
    fn test_next_prime() {
        let start = BigNumber::from(10);
        let prime = next_prime(start);
        assert!(prime.is_prime(), "The result should be a prime number");
        assert!(
            prime >= BigNumber::from(10),
            "The prime should be greater than or equal to the start value"
        );
    }

    #[test]
    fn test_consistency() {
        let input = "consistent";
        let prime1 = map_string_to_prime(input);
        let prime2 = map_string_to_prime(input);
        assert_eq!(
            prime1, prime2,
            "The hash function should be consistent for the same input"
        );
    }

    #[test]
    fn test_friendly() {
        let input = "test";
        let input2 = "test2";
        let prime1 = map_string_to_prime(input);
        let prime2 = map_string_to_prime(input2);
        assert_ne!(prime1, prime2, "The result should be drastically different");
    }

    #[test]
    fn test_empty_input() {
        let prime = map_string_to_prime("");
        assert!(prime.is_prime(), "Should handle empty string");
    }

    #[test]
    fn test_long_string() {
        let input = "a".repeat(1000); // A long string of 1000 'a's
        let prime = map_string_to_prime(&input);
        assert!(prime > BigNumber::from(0), "Prime should be positive");
        assert!(prime.is_prime(), "Result should be prime");
    }

    #[test]
    fn test_map_string_to_prime() {
        let input = "test";
        let prime = map_string_to_prime(input);
        assert!(prime.is_prime(), "The result should be a prime number");
    }
}
