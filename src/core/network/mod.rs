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

const PROTOCOL_ID: u64 = 1;//check for version control

const PLAYER_MOVE_SPEED: f32 = 1.0;


#[derive(Debug, Default, Serialize, Deserialize, Component, Resource)]
struct PlayerInput {
  up: bool,
  down: bool,
  left: bool,
  right: bool,
}

// CRAFT NETWORK PLUGIN

#[derive(Debug, Component)]
struct Player {
    id: u64,
}

#[derive(Debug, Default, Resource)]
struct Lobby {
    players: HashMap<u64, Entity>,
}

#[derive(Debug, Serialize, Deserialize, Component)]
enum ServerMessages {
    PlayerConnected { id: u64 },
    PlayerDisconnected { id: u64 },
}

fn new_renet_client() -> (RenetClient, NetcodeClientTransport) {
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

fn new_renet_server() -> (RenetServer, NetcodeServerTransport) {
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

pub struct BaseCraftPlugin;

impl Plugin for BaseCraftPlugin{

  fn build(&self, app: &mut App){

  }
  
}
