use serenity::prelude::TypeMapKey;
use super::repository::Repository;

pub struct ClientData;

impl TypeMapKey for ClientData {
    type Value = Repository;
}