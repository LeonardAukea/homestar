//! [gossipsub] and [Floodsub] initializers for PubSub across connected peers.

use anyhow::Result;
use libp2p::{
    floodsub::Floodsub,
    gossipsub::{self, ConfigBuilder, Message, MessageAuthenticity, MessageId, ValidationMode},
    identity::Keypair,
    PeerId,
};
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    time::Duration,
};

/// Setup direct [Floodsub] protocol with a given [PeerId].
pub fn new_floodsub(peer_id: PeerId) -> Floodsub {
    Floodsub::new(peer_id)
}

/// Setup [gossipsub] mesh protocol with default configuration.
pub fn new_gossipsub(keypair: Keypair) -> Result<gossipsub::Behaviour> {
    // To content-address message, we can take the hash of message and use it as an ID.
    let message_id_fn = |message: &Message| {
        let mut s = DefaultHasher::new();
        message.data.hash(&mut s);
        MessageId::from(s.finish().to_string())
    };

    // TODO: Make configurable
    let gossipsub_config = ConfigBuilder::default()
        .heartbeat_interval(Duration::from_secs(10))
        // This sets the kind of message validation. The default is Strict (enforce message signing).
        .validation_mode(ValidationMode::Strict)
        .mesh_n_low(1)
        .mesh_outbound_min(1)
        .mesh_n(2)
        // Content-address messages. No two messages of the same content will be propagated.
        .message_id_fn(message_id_fn)
        .build()
        .map_err(anyhow::Error::msg)?;

    gossipsub::Behaviour::new(MessageAuthenticity::Signed(keypair), gossipsub_config)
        .map_err(anyhow::Error::msg)
}