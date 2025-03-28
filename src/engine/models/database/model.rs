use crate::engine::models::collection::model::Collection;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Database {
    pub collections: Vec<Collection>,
}
