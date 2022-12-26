use ae_position::Position;

use crate::api::UserId;

#[derive(Debug)]
pub struct Player {
    pub pos: Position,
    pub id: UserId,
    pub name: String,
}

impl Player {
    pub fn new(id: UserId, name: String) -> Self {
        Self {
            pos: Position { x: 0, y: 0 },
            id,
            name,
        }
    }
}
