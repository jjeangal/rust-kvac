#[cfg(test)]
mod tests {
    use rust_kvac::group_hash_function::*;
    use unknown_order::BigNumber;

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
    fn test_empty_string() {
        let input = "";
        let prime = map_string_to_prime(input);
        println!("prime for empty string: {}", prime);
    }

    #[test]
    fn test_long_string() {
        let input = "a".repeat(1000); // A long string of 1000 'a's
        let prime = map_string_to_prime(&input);
        println!("prime for long string: {}", prime);
    }

    #[test]
    fn test_map_string_to_prime() {
        // let limit = BigNumber::from(1000);
        // let exclude = BigNumber::from(101);
        let input = "test";
        let prime = map_string_to_prime(input);
        assert!(prime.is_prime(), "The result should be a prime number");
        // assert!(
        //     prime != exclude,
        //     "The prime should not be equal to the excluded prime"
        // );
    }
}
