use super::connections::ConnectionsLock;
use crate::{
    api::{ServerMessage, UserId},
    game::world::WorldLock,
};
use warp::ws::Message;

pub async fn handle_disconnect(id: UserId, connections: &ConnectionsLock, world: &WorldLock) {
    eprintln!("good bye user: {}", id);

    let remove_player = ServerMessage::RemovedPlayer(id);
    let remove_player_serialized: String =
        serde_json::to_string(&remove_player).expect("Serialize should work");

    for (&_uid, sender) in connections.write().await.0.iter() {
        // if my_id != uid {
        if let Err(_disconnected) = sender.send(Message::text(&remove_player_serialized)) {}
    }

    // Remove player's connection from the list of active connections
    connections.write().await.0.remove(&id);

    // Remove player from the game world
    world.write().await.players.remove(&id);
}
