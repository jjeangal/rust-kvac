use rust_kvac::insert::insert;
use rust_kvac::params::*;
use rust_kvac::proof_update::proof_update;
use rust_kvac::update::update;
use rust_kvac::verify::verify;
use unknown_order::BigNumber;

fn main() {
    // Access the public parameters
    let pp = &PUBLIC_PARAMS;

    // Initialize the initial commitment
    let commitment = Commitment::new(pp.one.clone(), pp.g.clone());

    // Define keys and values
    let kv1: KeyValue = KeyValue::new("test".to_string(), BigNumber::from(8));
    let kv2: KeyValue = KeyValue::new("test".to_string(), BigNumber::from(10));
    let kv3: KeyValue = KeyValue::new("test".to_string(), BigNumber::from(18));

    // Insert the first key-value pair
    let (commitment2, proof, kv1) = insert(&commitment, &kv1);

    // Verify the first insert
    let verify_insert = verify(&commitment2, &kv1, &proof);
    println!("First insert verification: {:?}", verify_insert);

    // Update the first key-value pair
    let (commitment3, kv2) = update(&commitment2, &kv2);

    // Update proof_k1 in place
    let proof_update = proof_update(kv1.key(), &proof, &kv2).unwrap();

    // Use the updated proof
    let verify_update = verify(&commitment3, &kv3, &proof_update);
    println!("Second verification: {:?}", verify_update);
}
