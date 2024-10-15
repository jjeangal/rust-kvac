extern crate rust_kvac;

// what is a appropriate lambda

#[test]
fn test_keygen() {
    let result = rust_kvac::keygen::keygen(4 as usize);
    // Add assertions
    // assert!(result.is_ok(), "Key generation failed");
    println!("{:?}", result.1);
}
