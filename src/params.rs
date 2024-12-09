use crate::keygen::keygen;
use once_cell::sync::Lazy;
use std::sync::Arc;
use unknown_order::*;

/// Represents a commitment with two components.
#[derive(Debug, Clone, PartialEq)]
pub struct Commitment {
    c1: BigNumber,
    c2: BigNumber,
}

impl Commitment {
    /// Creates a new `Commitment`.
    ///
    /// # Arguments
    ///
    /// * `c1` - The first component of the commitment.
    /// * `c2` - The second component of the commitment.
    ///
    /// # Returns
    ///
    /// A new `Commitment` instance.
    pub fn new(c1: BigNumber, c2: BigNumber) -> Self {
        Self { c1, c2 }
    }

    /// Returns the first component of the commitment.
    pub fn c1(&self) -> &BigNumber {
        &self.c1
    }

    /// Returns the second component of the commitment.
    pub fn c2(&self) -> &BigNumber {
        &self.c2
    }
}

/// Represents a proof with three components.
#[derive(Debug, Clone, PartialEq)]
pub struct Proof {
    pub first: Commitment,
    pub second: (BigNumber, BigNumber, BigNumber),
    pub u_k: BigNumber,
}

impl Proof {
    /// Creates a new `Proof`.
    ///
    /// # Arguments
    ///
    /// * `first` - The first component of the proof.
    /// * `second` - The second component of the proof.
    /// * `u_k` - The third component of the proof.
    ///
    /// # Returns
    ///
    /// A new `Proof` instance.
    pub fn new(
        first: Commitment,
        second: (BigNumber, BigNumber, BigNumber),
        u_k: BigNumber,
    ) -> Self {
        Self { first, second, u_k }
    }

    /// Returns the first component of the proof.
    pub fn first(&self) -> &Commitment {
        &self.first
    }

    /// Returns the second component of the proof.
    pub fn second(&self) -> &(BigNumber, BigNumber, BigNumber) {
        &self.second
    }

    /// Returns the third component of the proof.
    pub fn u_k(&self) -> &BigNumber {
        &self.u_k
    }
}

/// Represents an operation with a key-value pair.
#[derive(Debug, Clone, PartialEq)]
pub struct KeyValue {
    pub key: String,
    pub value: BigNumber,
}

impl KeyValue {
    /// Creates a new `Operation`.
    ///
    /// # Arguments
    ///
    /// * `operation` - The type of operation (Insert or Update).
    /// * `key` - The key as a string.
    /// * `value` - The value as a `BigNumber`.
    ///
    /// # Returns
    ///
    /// A new `Operation` instance.
    pub fn new(key: String, value: BigNumber) -> Self {
        Self { key, value }
    }

    /// Returns the key of the operation.
    pub fn key(&self) -> &String {
        &self.key
    }

    /// Returns the value of the operation.
    pub fn value(&self) -> &BigNumber {
        &self.value
    }
}

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
