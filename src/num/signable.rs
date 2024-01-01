//! Trait to handle signability
pub trait Signable {
    const SIGNED: bool;
    fn is_signed(&self) -> bool {
        Self::SIGNED
    }
}
