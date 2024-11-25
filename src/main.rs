use rust_kvac::insert::insert;
use rust_kvac::params::PUBLIC_PARAMS;
use rust_kvac::proof_update::proof_update;
use rust_kvac::verify::verify;
use unknown_order::BigNumber;

fn main() {
    // Access the public parameters
    let pp = &PUBLIC_PARAMS;

    let c = (pp.one.clone(), pp.g.clone());
    let (k, v) = ("test".to_string(), BigNumber::from(8));

    // Insert a key-value pair
    let (new_c, proof_k, upd) = insert(c.clone(), (k.clone(), v.clone()));

    // Verify the proof
    let ver1 = verify(new_c.clone(), upd.clone(), proof_k.clone());
    println!("Verification 1: {:?}", ver1);

    let (new_c2, _, upd2) = insert(new_c.clone(), (k.clone(), v.clone()));

    // Update the proof
    let updated_proof = proof_update(proof_k.clone(), (k.clone(), v.clone()));

    // check updated proof is equal to c
    println!("updated_proof == c ? {:?}", updated_proof == new_c);

    // verify the updated proof
    let ver2 = verify(new_c2.clone(), upd2.clone(), updated_proof);
    println!("Verification for proof update: {:?}", ver2);
}

// a = 2^256
