use rust_kvac::insert::insert;
use rust_kvac::params::PUBLIC_PARAMS;
use rust_kvac::proof_update::proof_update;
use rust_kvac::verify::verify;
use unknown_order::BigNumber;

// a = 2^256

fn main() {
    // Access the public parameters
    let pp = &PUBLIC_PARAMS;

    let c = (pp.one.clone(), pp.g.clone());
    let (k, v) = ("test".to_string(), BigNumber::from(8));
    let (k2, v2) = ("test".to_string(), BigNumber::from(10));
    let (k3, v3) = ("test".to_string(), BigNumber::from(12));

    //#########################################################
    // Test inserts
    //#########################################################

    // Insert a key-value pair
    let (new_c, proof_k, upd) = insert(c.clone(), (k.clone(), v.clone()));

    // Verify the proof
    let verify_insert_1 = verify(new_c.clone(), upd.clone(), proof_k.clone());
    println!("First insert verification: {:?}", verify_insert_1);

    // Insert another key-value pair
    let (new_c2, proof_k2, upd2) = insert(new_c.clone(), (k2.clone(), v2.clone()));

    // Verify the proof
    let verify_insert_2 = verify(new_c2.clone(), upd2.clone(), proof_k2.clone());
    println!("Second insert verification: {:?}", verify_insert_2);

    let (new_c3, _, upd3) = insert(new_c2.clone(), (k3.clone(), v3.clone()));

    //#########################################################
    // Test proof updates
    //#########################################################

    // Update the proof
    let updated_proof = proof_update(proof_k.clone(), (k.clone(), v.clone()));

    // check updated proof is equal to c
    println!("'updated_proof' == 'c' ? {:?}", updated_proof == new_c);

    // verify the updated proof
    let verify_update_1 = verify(new_c2.clone(), upd2.clone(), updated_proof);
    println!("First proof update verification: {:?}", verify_update_1);

    //#########################################################

    // Update the proof
    let updated_proof2 = proof_update(proof_k2.clone(), (k2.clone(), v2.clone()));

    // check updated proof is equal to c
    println!("'updated_proof2' == 'c2' ? {:?}", updated_proof2 == new_c2);

    // verify the updated proof
    let verify_update_2 = verify(new_c3.clone(), upd3.clone(), updated_proof2);
    println!("Second proof update verification: {:?}", verify_update_2);
}
