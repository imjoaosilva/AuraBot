use super::db::models::repository::Repository;
use serenity::prelude::TypeMapKey;

pub struct ClientData;

impl TypeMapKey for ClientData {
    type Value = Repository;
}
