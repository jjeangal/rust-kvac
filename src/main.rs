use rust_kvac::insert::insert;
use rust_kvac::params::*;
use rust_kvac::proof_update::proof_update;
use rust_kvac::update::update;
use rust_kvac::verify::verify;
use unknown_order::BigNumber;

fn main() {
    // Access the public parameters
    let pp = &PUBLIC_PARAMS;

    let commitment = Commitment::new(pp.one.clone(), pp.g.clone());
    let key = "test".to_string();
    let value = BigNumber::from(8);
    let value2 = BigNumber::from(10);

    //#########################################################
    // Test inserts
    //#########################################################

    // Insert a key-value pair
    let (commitment2, proof_k, op1) = insert(commitment, (key.clone(), value.clone()));

    // Verify the proof
    let verify_insert_1 = verify(
        commitment2.clone(),
        (op1.key().clone(), op1.value().clone()),
        proof_k.clone(),
    );
    println!("First insert verification: {:?}", verify_insert_1);

    // Update the key-value pair
    let (new_c2, op2) = update(commitment2.clone(), (key.clone(), value2.clone()));

    // Update the proof
    let updated_proof = proof_update(key.clone(), proof_k, op2.clone()).unwrap_or_else(|e| {
        println!("Error updating proof: {:?}", e);
        std::process::exit(1);
    });

    // Verify the updated proof
    let verify_insert_2 = verify(
        new_c2.clone(),
        (op2.key().clone(), op2.value().clone()),
        updated_proof.clone(),
    );
    println!("Second insert verification: {:?}", verify_insert_2);
}
