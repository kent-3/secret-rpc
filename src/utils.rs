//! Utility methods for the Tendermint RPC crate.

use nanorand::{BufferedRng, Rng, WyRand};

use crate::prelude::*;

/// Produce a string containing a UUID.
///
/// Panics if random number generation fails.
pub fn uuid_str() -> String {
    let mut bytes = [0u8; 16];
    let mut rng = BufferedRng::new(WyRand::new());
    rng.fill(&mut bytes);
    let uuid = uuid::Builder::from_random_bytes(bytes).into_uuid();
    uuid.to_string()
}
