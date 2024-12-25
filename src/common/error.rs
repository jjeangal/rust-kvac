#[derive(Debug)]
pub enum ProofError {
    InvalidProof(String),
    NonCoprimePair,
}

#[derive(Debug)]
pub enum CryptoError {
    InvalidGroup(String),
    InvalidJacobiSymbol,
    InvalidJacobiInput(String),
}
