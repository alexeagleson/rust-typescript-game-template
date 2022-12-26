use crate::{
    api::{ClientMessage, Key, LogMessage, PlayerDetails, PlayerPosition, ServerMessage, UserId},
    database::DatabaseLock,
    game::world::WorldLock,
};
use ae_direction::{Cardinal, Direction};
use ae_position::Delta;
use warp::ws::Message;

use super::connections::ConnectionsLock;

pub async fn handle_message(
    id: UserId,
    msg: Message,
    connections: &ConnectionsLock,
    db: &DatabaseLock,
    world: &mut WorldLock,
) {
    // Skip any non-Text messages...
    let msg = if let Ok(s) = msg.to_str() {
        s
    } else {
        return;
    };

    // Log the socket message from the client
    // println!("{}", msg);

    let request = serde_json::from_str::<ClientMessage>(msg);

    match request {
        Ok(request) => match request {
            ClientMessage::Initialize => {
                // Add player to the game
                world.write().await.new_player(id, format!("Player {}", id));

                let world = world.write().await;

                let mut positions: Vec<PlayerPosition> = Vec::with_capacity(world.players.len());

                for (player_id, player) in world.players.iter() {
                    positions.push(PlayerPosition {
                        id: *player_id,
                        pos: player.read().await.pos.clone(),
                    });
                }

                drop(world);

                let player_positions_serialized: String =
                    serde_json::to_string(&ServerMessage::AllPlayerPositions(positions))
                        .expect("Serialize should work");

                for (&_uid, sender) in connections.read().await.0.iter() {
                    println!("sending add player to {}", _uid);
                    if let Err(_disconnected) =
                        sender.send(Message::text(&player_positions_serialized))
                    {
                        // The tx is disconnected, our `user_disconnected` code
                        // should be happening in another task, nothing more to
                        // do here.
                    }
                }
            }
            ClientMessage::TileHover(pos) => {
                let world = world.write().await;

                let mut hovered_player_name: Option<String> = None;

                for (_, player) in world.players.iter() {
                    let player = player.read().await;
                    if player.pos == pos {
                        hovered_player_name = Some(player.name.clone());
                        break;
                    }
                }

                let player_hover_data = ServerMessage::TileHover(
                    hovered_player_name.map(|name| PlayerDetails { name }),
                );
                let player_hover_data_serialized =
                    serde_json::to_string(&player_hover_data).expect("Should be able to serialize");

                for (&uid, sender) in connections.read().await.0.iter() {
                    if uid == id {
                        // Only send hover data to the player who requested it
                        sender
                            .send(Message::text(&player_hover_data_serialized))
                            .ok();

                        break;
                    }
                }
            }
            ClientMessage::TileClick(pos) => {
                println!("Player {} clicked something", id);
                let world = world.write().await;

                let mut clicked_player_id: Option<UserId> = None;

                for (_, player) in world.players.iter() {
                    let player = player.read().await;
                    if player.pos == pos {
                        clicked_player_id = Some(player.id);
                        break;
                    }
                }

                if let Some(clicked_player_id) = clicked_player_id {
                    let log_message = ServerMessage::TileClick(LogMessage(format!(
                        "Player {} clicked Player {}",
                        id, clicked_player_id
                    )));

                    let log_message_serialized: String =
                        serde_json::to_string(&log_message).expect("Should work");

                    for (&_uid, sender) in connections.read().await.0.iter() {
                        sender.send(Message::text(&log_message_serialized)).ok();
                    }
                }
            }

            ClientMessage::Keypress(key) => {
                let key_string = key.to_string();
                let db = db.read().await;

                // Log the move in the database regardkess of whether it succeeds because why not
                sqlx::query!("INSERT INTO moves (direction) VALUES (?)", key_string)
                    .execute(&db.0)
                    .await
                    .unwrap();

                let moves = sqlx::query!("SELECT id FROM moves")
                    .fetch_all(&db.0)
                    .await
                    .unwrap();

                let move_count: ServerMessage = ServerMessage::MoveCount(moves.len() as i32);
                let move_count_serialized: String =
                    serde_json::to_string(&move_count).expect("that should work");

                let world = world.write().await;

                let my_player =
                    world.players.iter().find_map(
                        |(&uid, player)| {
                            if uid == id {
                                Some(player)
                            } else {
                                None
                            }
                        },
                    );

                if let Some(my_player) = my_player {
                    let mut my_player = my_player.write().await;

                    let new_pos = match key {
                        Key::Up => my_player
                            .pos
                            .add_delta(&Delta::from(Direction::Cardinal(Cardinal::North))),
                        Key::Down => my_player
                            .pos
                            .add_delta(&Delta::from(Direction::Cardinal(Cardinal::South))),
                        Key::Left => my_player
                            .pos
                            .add_delta(&Delta::from(Direction::Cardinal(Cardinal::West))),
                        Key::Right => my_player
                            .pos
                            .add_delta(&Delta::from(Direction::Cardinal(Cardinal::East))),
                    };

                    if world.map.valid_position(&new_pos) {
                        my_player.pos = new_pos;
                    }

                    let player_position_update: ServerMessage =
                        ServerMessage::PlayerPosition(PlayerPosition {
                            pos: my_player.pos.clone(),
                            id: my_player.id,
                        });

                    drop(my_player);

                    let player_position_update_serialized: String =
                        serde_json::to_string(&player_position_update)
                            .expect("whoops failed to serialize");

                    for (&_uid, sender) in connections.read().await.0.iter() {
                        sender
                            .send(Message::text(&player_position_update_serialized))
                            .ok();

                        sender.send(Message::text(&move_count_serialized)).ok();
                    }
                }
            }
        },
        Err(_) => panic!("Received an unsupported message {}", msg),
    }
}
