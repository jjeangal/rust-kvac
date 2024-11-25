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
}

// Lazy static variable to store your public parameters
pub static PUBLIC_PARAMS: Lazy<PublicParameters> = Lazy::new(|| {
    let result = keygen(1024);
    let ((_group, _g, _hash_fn), (_one, _g2)) = result;

    PublicParameters {
        // a: _a,
        // b: _b,
        group: _group,
        g: _g,
        hash_function: _hash_fn,
        one: _one,
        g2: _g2,
    }
});
