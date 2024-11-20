use rust_kvac::group_hash_function::{map_string_to_prime, next_prime};

#[cfg(test)]
mod tests {
    use super::*;
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
