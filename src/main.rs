use rust_kvac::insert::insert;
use rust_kvac::params::PUBLIC_PARAMS;
use rust_kvac::proof_update::proof_update;
use rust_kvac::verify::verify;
use unknown_order::BigNumber;

fn main() {
    // Access the public parameters
    let pp = &PUBLIC_PARAMS;

    // Insert a key-value pair
    let c = (pp.one.clone(), pp.g.clone());
    let (k, v) = ("test".to_string(), BigNumber::from(8));

    let (new_c, proof_k, upd) = insert(c, (k, v));

    // Update the proof
    let st = "maybe".to_string();
    let new_proof_k = proof_update(st, proof_k, upd.clone());

    // Verify the proof
    let ver = verify(new_c, upd, new_proof_k);
    println!("Verification: {:?}", ver);
}

// a = 2^256
