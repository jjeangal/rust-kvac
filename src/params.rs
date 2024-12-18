use crate::keygen::keygen;
use once_cell::sync::Lazy;
use std::fmt;
use std::sync::Arc;
use unknown_order::*;

// Add this helper function at the top of the file
pub fn format_bignumber(bn: &BigNumber) -> String {
    let s = format!("{:?}", bn);
    if s.len() > 20 {
        format!("{}...{}", &s[..10], &s[s.len() - 10..])
    } else {
        s
    }
}

/// Represents a commitment (C1, C2)
#[derive(Debug, Clone, PartialEq)]
pub struct Commitment {
    pub c1: BigNumber, // C1
    pub c2: BigNumber, // C2
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

// Implement Display for Commitment
impl fmt::Display for Commitment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Commitment {{\n    c1: {}\n    c2: {}\n}}",
            format_bignumber(&self.c1),
            format_bignumber(&self.c2)
        )
    }
}

/// Represents a proof Λk = ((Λk,1,Λk,2),(Λk,3,Λk,4,Λk,5), uk)
#[derive(Debug, Clone, PartialEq)]
pub struct Proof {
    pub lambda_k: Commitment,                          // (Λk,1,Λk,2)
    pub lambda_aux: (BigNumber, BigNumber, BigNumber), // (Λk,3,Λk,4,Λk,5)
    pub uk: BigNumber,                                 // uk
}

impl Proof {
    /// Creates a new `Proof`.
    ///
    /// # Arguments
    ///
    /// * `lambda_k` - The first component of the proof.
    /// * `lambda_aux` - The second component of the proof.
    /// * `uk` - The third component of the proof.
    ///
    /// # Returns
    ///
    /// A new `Proof` instance.
    pub fn new(
        lambda_k: Commitment,
        lambda_aux: (BigNumber, BigNumber, BigNumber),
        uk: BigNumber,
    ) -> Self {
        Self {
            lambda_k,
            lambda_aux,
            uk,
        }
    }

    /// Returns the first component of the proof.
    pub fn lambda_k(&self) -> &Commitment {
        &self.lambda_k
    }

    /// Returns the second component of the proof.
    pub fn lambda_aux(&self) -> &(BigNumber, BigNumber, BigNumber) {
        &self.lambda_aux
    }

    /// Returns the third component of the proof.
    pub fn uk(&self) -> &BigNumber {
        &self.uk
    }
}

// Implement Display for Proof
impl fmt::Display for Proof {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Proof {{\n    lambda_k: {}\n    lambda_aux: (\n        {}\n        {}\n        {}\n    )\n    uk: {}\n}}",
            self.lambda_k,
            format_bignumber(&self.lambda_aux.0),  // Λk,3
            format_bignumber(&self.lambda_aux.1),  // Λk,4
            format_bignumber(&self.lambda_aux.2),  // Λk,5
            format_bignumber(&self.uk)
        )
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

// Implement Display for KeyValue
impl fmt::Display for KeyValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "KeyValue {{\n    key: \"{}\"\n    value: {}\n}}",
            self.key,
            format_bignumber(&self.value)
        )
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
