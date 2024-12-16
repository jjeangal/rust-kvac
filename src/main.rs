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

    // Insert the first key-value pair
    let (commitment2, proof_k1, kv1) = insert(commitment.clone(), kv1.clone());

    // Verify the first insert
    let verify_insert = verify(commitment2.clone(), kv1.clone(), proof_k1.clone());
    println!("First insert verification: {:?}", verify_insert);

    // Update the first key-value pair
    let (commitment3, kv2) = update(commitment2.clone(), kv2.clone());

    let proof_upd: Proof = proof_update(kv1.key, proof_k1, kv2.clone()).unwrap();

    // Verify the second insert
    let verify_update = verify(commitment3.clone(), kv2.clone(), proof_upd.clone());
    println!("Second verification: {:?}", verify_update);
}
