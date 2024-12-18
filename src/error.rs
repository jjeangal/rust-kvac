#[derive(Debug)]
pub enum ProofError {
    InvalidProof(String),
    NonCoprimePair,
}
