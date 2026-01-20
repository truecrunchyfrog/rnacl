use rand::{RngCore, rngs::ThreadRng};

pub fn random_node_id(rng: Option<ThreadRng>) -> String {
    let mut bytes = [0u8; 7];
    rng.unwrap_or_else(rand::rng).fill_bytes(&mut bytes);
    hex::encode(bytes)
}
