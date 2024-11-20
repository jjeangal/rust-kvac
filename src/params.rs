use crate::keygen::keygen;
use once_cell::sync::Lazy;
use std::sync::Arc;
use unknown_order::*;

// Define your PublicParameters struct
pub struct PublicParameters {
    // pub a: BigNumber,
    // pub b: BigNumber,
    pub group: Group,
    pub g: BigNumber,
    pub hash_function: Arc<dyn Fn(&str) -> BigNumber + Send + Sync>,
    pub one: BigNumber,
    pub g2: BigNumber,
    // pub v: Vec<BigNumber>,
    // pub k: Vec<BigNumber>,
}

// Lazy static variable to store your public parameters
pub static PUBLIC_PARAMS: Lazy<PublicParameters> = Lazy::new(|| {
    let result = keygen(124 as usize);
    let ((_group, _g, _hash_fn), (_one, _g2)) = result;

    // // Set V = [0, a) as a vector of BigNumber
    // let a_bytes = _a.to_bytes();
    // let a_int = u64::from_be_bytes(
    //     a_bytes[..8]
    //         .try_into()
    //         .expect("BigNumber to u64 conversion failed"),
    // );

    // let v: Vec<BigNumber> = (0..a_int).map(|x| BigNumber::from(x)).collect();

    // Set K = {0, 1}*
    //let k = vec![BigNumber::from(0), BigNumber::from(1)];

    PublicParameters {
        // a: _a,
        // b: _b,
        group: _group,
        g: _g,
        hash_function: _hash_fn,
        one: _one,
        g2: _g2,
        // v,
        // k,
    }
});
