use rust_kvac::keygen::keygen;
use unknown_order::BigNumber;

#[test]
fn test_keygen() {
    let lambda = 128;
    let (pp, c) = keygen(lambda);

    // Check that the public parameters are correctly generated
    let (group, g, hash_function) = pp;

    // Ensure a, b, and g are not zero
    // assert!(a != BigNumber::zero(), "a should not be zero");
    // assert!(b != BigNumber::zero(), "b should not be zero");
    assert!(g != BigNumber::zero(), "g should not be zero");

    // Ensure the group modulus is correct
    let modulus = &group.modulus;
    // assert!(a < *modulus, "a should be less than the group modulus");
    // assert!(b < *modulus, "b should be less than the group modulus");
    assert!(g < *modulus, "g should be less than the group modulus");

    // Ensure the commitment is correctly generated
    let (c1, c2) = c;
    assert!(c1 == BigNumber::from(1), "c1 should be 1");
    assert!(c2 == g, "c2 should be equal to g");

    // Test the hash function
    let input = "test";
    let prime = hash_function(input);
    assert!(
        prime.is_prime(),
        "The result of the hash function should be a prime number"
    );
}
