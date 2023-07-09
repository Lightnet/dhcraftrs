/*
  Project Name: dhcraftrs
  License: CC BY-SA
  Created by: Lightnet
  Information: Note there are other licenses.
 */

// network
// server
// client

use bevy::prelude::*;

use bevy_renet::{
  renet::{
      transport::{ClientAuthentication, ServerAuthentication, ServerConfig},
      ConnectionConfig, DefaultChannel, RenetClient, RenetServer, ServerEvent,
  },
  transport::{NetcodeClientPlugin, NetcodeServerPlugin},
  RenetClientPlugin, RenetServerPlugin,
};
use renet::transport::{NetcodeClientTransport, NetcodeServerTransport, NetcodeTransportError};

use std::time::SystemTime;
use std::{collections::HashMap, net::UdpSocket};

use serde::{Deserialize, Serialize};

use super::api::AppState;

const PROTOCOL_ID: u64 = 1;//check for version control

const PLAYER_MOVE_SPEED: f32 = 1.0;

#[derive(Debug, Default, Serialize, Deserialize, Component, Resource)]
pub struct PlayerInput {
  up: bool,
  down: bool,
  left: bool,
  right: bool,
}

// CRAFT NETWORK PLUGIN

#[derive(Debug, Component)]
pub struct Player {
    id: u64,
}

#[derive(Debug, Default, Resource)]
pub struct Lobby {
    players: HashMap<u64, Entity>,
}

#[derive(Debug, Serialize, Deserialize, Component)]
pub enum ServerMessages {
    PlayerConnected { id: u64 },
    PlayerDisconnected { id: u64 },
}

pub fn new_renet_client() -> (RenetClient, NetcodeClientTransport) {
  let client = RenetClient::new(ConnectionConfig::default());

  let server_addr = "127.0.0.1:5000".parse().unwrap();
  let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
  let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
  let client_id = current_time.as_millis() as u64;
  let authentication = ClientAuthentication::Unsecure {
    client_id,
    protocol_id: PROTOCOL_ID,
    server_addr,
    user_data: None,
  };

  let transport = NetcodeClientTransport::new(current_time, authentication, socket).unwrap();

  (client, transport)
}

pub fn new_renet_server() -> (RenetServer, NetcodeServerTransport) {
  let server = RenetServer::new(ConnectionConfig::default());

  let public_addr = "127.0.0.1:5000".parse().unwrap();
  let socket = UdpSocket::bind(public_addr).unwrap();
  let server_config = ServerConfig {
    max_clients: 64,
    protocol_id: PROTOCOL_ID,
    public_addr,
    authentication: ServerAuthentication::Unsecure,
  };
  let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();

  let transport = NetcodeServerTransport::new(current_time, server_config, socket).unwrap();

  (server, transport)
}

pub struct NetworkCraftPlugin;

impl Plugin for NetworkCraftPlugin{

  fn build(&self, app: &mut App){
    app.init_resource::<Lobby>();

    //app.add_system(system_enter_server.in_schedule(OnEnter(AppState::SERVER))); //not working
    //app.add_system(system_enter_server.in_schedule(OnEnter(AppState::SERVER))); //not working app call does not have in system prevent miss used.

    // SERVER
    app.add_plugin(RenetServerPlugin);
    app.add_plugin(NetcodeServerPlugin);
    app.add_system(setup_system_server.in_schedule(OnEnter(AppState::SERVER)));
    app.add_systems((server_update_system, server_sync_players, move_players_system).in_set(OnUpdate(AppState::SERVER)));

    //client
    app.add_plugin(RenetClientPlugin);
    app.add_plugin(NetcodeClientPlugin);
    //app.init_resource::<PlayerInput>();

    app.add_system(setup_system_client.in_schedule(OnEnter(AppState::CLIENT)));
    app.add_systems(
      (player_input, client_send_input, client_sync_players)
          .distributive_run_if(bevy_renet::transport::client_connected)
          .in_base_set(CoreSet::Update),
    );

    app.add_system(setup.in_schedule(OnEnter(AppState::SERVER)));
    app.add_system(setup.in_schedule(OnEnter(AppState::CLIENT)));

  }
  
}

//pub fn system_enter_server(
  //mut commands: Commands,
  //mut app:App,
//){
  //println!("SERVER INIT...");
//}

pub fn setup_system_server(
  mut commands: Commands,
){
  println!("INIT SERVER HOST SET UP");
  let (server, transport) = new_renet_server();
  commands.insert_resource(server);
  commands.insert_resource(transport);
}

pub fn setup_server(mut app:App){
  app.add_plugin(RenetServerPlugin);
  app.add_plugin(NetcodeServerPlugin);
  let (server, transport) = new_renet_server();
  app.insert_resource(server);
  app.insert_resource(transport);

  app.add_systems((server_update_system, server_sync_players, move_players_system));
}

//pub fn system_enter_client(
  //mut commands: Commands,
//){
  //println!("SERVER INIT...");
//}

pub fn setup_system_client(
  mut commands: Commands,
){
  let (client, transport) = new_renet_client();
  commands.init_resource::<PlayerInput>();
  commands.insert_resource(client);
  commands.insert_resource(transport);
}

pub fn setup_client(mut app:App){
  app.add_plugin(RenetClientPlugin);
  app.add_plugin(NetcodeClientPlugin);
  app.init_resource::<PlayerInput>();
  let (client, transport) = new_renet_client();
  app.insert_resource(client);
  app.insert_resource(transport);

  app.add_systems(
      (player_input, client_send_input, client_sync_players)
          .distributive_run_if(bevy_renet::transport::client_connected)
          .in_base_set(CoreSet::Update),
  );
}

pub fn server_update_system(
  mut server_events: EventReader<ServerEvent>,
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  mut lobby: ResMut<Lobby>,
  mut server: ResMut<RenetServer>,
) {
  for event in server_events.iter() {
      match event {
          ServerEvent::ClientConnected { client_id } => {
              println!("Player {} connected.", client_id);
              // Spawn player cube
              let player_entity = commands
                  .spawn(PbrBundle {
                      mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                      material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                      transform: Transform::from_xyz(0.0, 0.5, 0.0),
                      ..Default::default()
                  })
                  .insert(PlayerInput::default())
                  .insert(Player { id: *client_id })
                  .id();

              // We could send an InitState with all the players id and positions for the client
              // but this is easier to do.
              for &player_id in lobby.players.keys() {
                  let message = bincode::serialize(&ServerMessages::PlayerConnected { id: player_id }).unwrap();
                  server.send_message(*client_id, DefaultChannel::ReliableOrdered, message);
              }

              lobby.players.insert(*client_id, player_entity);

              let message = bincode::serialize(&ServerMessages::PlayerConnected { id: *client_id }).unwrap();
              server.broadcast_message(DefaultChannel::ReliableOrdered, message);
          }
          ServerEvent::ClientDisconnected { client_id, reason } => {
              println!("Player {} disconnected: {}", client_id, reason);
              if let Some(player_entity) = lobby.players.remove(client_id) {
                  commands.entity(player_entity).despawn();
              }

              let message = bincode::serialize(&ServerMessages::PlayerDisconnected { id: *client_id }).unwrap();
              server.broadcast_message(DefaultChannel::ReliableOrdered, message);
          }
      }
  }

  for client_id in server.clients_id().into_iter() {
      while let Some(message) = server.receive_message(client_id, DefaultChannel::ReliableOrdered) {
          let player_input: PlayerInput = bincode::deserialize(&message).unwrap();
          if let Some(player_entity) = lobby.players.get(&client_id) {
              commands.entity(*player_entity).insert(player_input);
          }
      }
  }
}

pub fn server_sync_players(mut server: ResMut<RenetServer>, query: Query<(&Transform, &Player)>) {
  let mut players: HashMap<u64, [f32; 3]> = HashMap::new();
  for (transform, player) in query.iter() {
      players.insert(player.id, transform.translation.into());
  }

  let sync_message = bincode::serialize(&players).unwrap();
  server.broadcast_message(DefaultChannel::Unreliable, sync_message);
}

pub fn client_sync_players(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  mut client: ResMut<RenetClient>,
  mut lobby: ResMut<Lobby>,
) {
  while let Some(message) = client.receive_message(DefaultChannel::ReliableOrdered) {
      let server_message = bincode::deserialize(&message).unwrap();
      match server_message {
          ServerMessages::PlayerConnected { id } => {
              println!("Player {} connected.", id);
              let player_entity = commands
                  .spawn(PbrBundle {
                      mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                      material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                      transform: Transform::from_xyz(0.0, 0.5, 0.0),
                      ..Default::default()
                  })
                  .id();

              lobby.players.insert(id, player_entity);
          }
          ServerMessages::PlayerDisconnected { id } => {
              println!("Player {} disconnected.", id);
              if let Some(player_entity) = lobby.players.remove(&id) {
                  commands.entity(player_entity).despawn();
              }
          }
      }
  }

  while let Some(message) = client.receive_message(DefaultChannel::Unreliable) {
      let players: HashMap<u64, [f32; 3]> = bincode::deserialize(&message).unwrap();
      for (player_id, translation) in players.iter() {
          if let Some(player_entity) = lobby.players.get(player_id) {
              let transform = Transform {
                  translation: (*translation).into(),
                  ..Default::default()
              };
              commands.entity(*player_entity).insert(transform);
          }
      }
  }
}

/// set up a simple 3D scene
pub fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
  // plane
  commands.spawn(PbrBundle {
      mesh: meshes.add(shape::Plane::from_size(5.0).into()),
      material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
      ..Default::default()
  });
  // light
  commands.spawn(PointLightBundle {
      point_light: PointLight {
          intensity: 1500.0,
          shadows_enabled: true,
          ..Default::default()
      },
      transform: Transform::from_xyz(4.0, 8.0, 4.0),
      ..Default::default()
  });
  // camera
  commands.spawn(Camera3dBundle {
      transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
      ..Default::default()
  });
}

pub fn player_input(keyboard_input: Res<Input<KeyCode>>, mut player_input: ResMut<PlayerInput>) {
  player_input.left = keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left);
  player_input.right = keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right);
  player_input.up = keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up);
  player_input.down = keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down);
}

pub fn client_send_input(player_input: Res<PlayerInput>, mut client: ResMut<RenetClient>) {
  let input_message = bincode::serialize(&*player_input).unwrap();

  client.send_message(DefaultChannel::ReliableOrdered, input_message);
}

pub fn move_players_system(mut query: Query<(&mut Transform, &PlayerInput)>, time: Res<Time>) {
  for (mut transform, input) in query.iter_mut() {
      let x = (input.right as i8 - input.left as i8) as f32;
      let y = (input.down as i8 - input.up as i8) as f32;
      transform.translation.x += x * PLAYER_MOVE_SPEED * time.delta().as_secs_f32();
      transform.translation.z += y * PLAYER_MOVE_SPEED * time.delta().as_secs_f32();
  }
}

// If any error is found we just panic
pub fn panic_on_error_system(mut renet_error: EventReader<NetcodeTransportError>) {
  for e in renet_error.iter() {
      panic!("{}", e);
  }
}