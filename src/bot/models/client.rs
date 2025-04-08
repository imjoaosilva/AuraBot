use serenity::prelude::TypeMapKey;
use super::db::models::repository::Repository;

pub struct ClientData;

impl TypeMapKey for ClientData {
    type Value = Repository;
}