use crate::ocnode::Ocnode;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct StoredOctree {
    pub name: String,
    pub active_nodes: Vec<Ocnode>,
}
