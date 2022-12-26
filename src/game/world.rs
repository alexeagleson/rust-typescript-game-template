use crate::api::UserId;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

use super::{map::Map, player::Player};

#[derive(Debug)]
pub struct World {
    pub players: HashMap<UserId, RwLock<Player>>,
    pub map: Map,
}

impl Default for World {
    fn default() -> Self {
        Self {
            players: Default::default(),
            map: Default::default(),
        }
    }
}

impl World {
    pub fn new_player(&mut self, id: UserId, name: String) {
        self.players.insert(id, RwLock::new(Player::new(id, name)));
    }
}

pub type WorldLock = Arc<RwLock<World>>;
