use super::connections::ConnectionsLock;
use crate::{
    api::{ServerMessage, UserId},
    game::world::WorldLock,
};
use log::info;
use warp::ws::Message;

pub async fn handle_disconnect(id: UserId, connections: &ConnectionsLock, world: &WorldLock) {
    info!("User disconnected: {}", id);

    let remove_player = ServerMessage::RemovedPlayer(id);
    let remove_player_serialized: String =
        serde_json::to_string(&remove_player).expect("Serialize should work");

    for (&_uid, sender) in connections.write().await.0.iter() {
        sender.send(Message::text(&remove_player_serialized)).ok();
    }

    // Remove player's connection from the list of active connections
    connections.write().await.0.remove(&id);
    info!("Removing player connection: {}", id);

    // Remove player from the game world
    world.write().await.players.remove(&id);
    info!("Removing player from world: {}", id);
}
