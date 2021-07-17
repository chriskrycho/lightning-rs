use std::collections::HashMap;
use std::hash::Hash;

use serde::{Deserialize, Serialize};

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Deserialize, Serialize)]
pub struct Id(uuid::Uuid);

#[derive(Debug, Deserialize, Serialize)]
pub struct Collection {
    pub id: Id,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CollectionPages(HashMap<Id, crate::page::Id>);
