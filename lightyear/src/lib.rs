/*!
Lightyear is a networking library for Bevy.
It is designed for server-authoritative multiplayer games; and aims to be both feature-complete and easy-to-use.

You can find more information in the [book](https://cbournhonesque.github.io/lightyear/book/)!
*/
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(clippy::type_complexity)]
#![allow(rustdoc::private_intra_doc_links)]

// re-exports (mostly used in the derive macro crate or for internal purposes)
#[doc(hidden)]
pub mod _reexport {
    pub use enum_delegate;
    pub use enum_dispatch::enum_dispatch;
    pub use lightyear_macros::{
        component_protocol_internal, message_protocol_internal, ChannelInternal, MessageInternal,
    };
    pub use paste::paste;

    pub use crate::channel::builder::TickBufferChannel;
    pub use crate::channel::builder::{
        EntityActionsChannel, EntityUpdatesChannel, InputChannel, PingChannel,
    };
    pub use crate::inputs::input_buffer::InputMessage;
    pub use crate::protocol::component::{
        ComponentBehaviour, ComponentKindBehaviour, ComponentProtocol, ComponentProtocolKind,
        IntoKind,
    };
    pub use crate::protocol::message::{MessageBehaviour, MessageKind, MessageProtocol};
    pub use crate::protocol::{BitSerializable, EventContext};
    pub use crate::serialize::reader::ReadBuffer;
    pub use crate::serialize::wordbuffer::reader::ReadWordBuffer;
    pub use crate::serialize::wordbuffer::writer::WriteWordBuffer;
    pub use crate::serialize::writer::WriteBuffer;
    pub use crate::shared::replication::components::{ShouldBeInterpolated, ShouldBePredicted};
    pub use crate::shared::replication::ReplicationSend;
    pub use crate::shared::tick_manager::TickManager;
    pub use crate::shared::time_manager::{TimeManager, WrappedTime};
    pub use crate::utils::ready_buffer::ReadyBuffer;
    pub use crate::utils::sequence_buffer::SequenceBuffer;
}

/// Prelude containing commonly used types
pub mod prelude {
    pub use lightyear_macros::{component_protocol, message_protocol, Channel, Message};

    pub use crate::channel::builder::TickBufferChannel;
    pub use crate::channel::builder::{
        Channel, ChannelBuilder, ChannelContainer, ChannelDirection, ChannelMode, ChannelSettings,
        DefaultUnorderedUnreliableChannel, ReliableSettings,
    };
    pub use crate::inputs::UserInput;
    pub use crate::netcode::{generate_key, ClientId, Key};
    pub use crate::packet::message::Message;
    pub use crate::protocol::channel::{ChannelKind, ChannelRegistry};
    pub use crate::protocol::Protocol;
    pub use crate::protocolize;
    pub use crate::shared::config::SharedConfig;
    pub use crate::shared::log::LogConfig;
    pub use crate::shared::ping::manager::PingConfig;
    pub use crate::shared::plugin::SharedPlugin;
    pub use crate::shared::replication::components::{NetworkTarget, Replicate};
    pub use crate::shared::replication::resources::ReplicationData;
    pub use crate::shared::sets::{FixedUpdateSet, MainSet, ReplicationSet};
    pub use crate::shared::tick_manager::{Tick, TickConfig};
    pub use crate::transport::conditioner::LinkConditionerConfig;
    pub use crate::transport::io::{Io, IoConfig, TransportConfig};
    pub use crate::transport::udp::UdpSocket;
    pub use crate::utils::named::{Named, TypeNamed};

    pub mod client {
        pub use crate::client::components::{ComponentSyncMode, Confirmed, SyncComponent};
        pub use crate::client::config::ClientConfig;
        pub use crate::client::config::NetcodeConfig;
        pub use crate::client::events::{
            ComponentInsertEvent, ComponentRemoveEvent, ComponentUpdateEvent, ConnectEvent,
            DisconnectEvent, EntityDespawnEvent, EntitySpawnEvent, InputEvent, MessageEvent,
        };
        pub use crate::client::input::{InputConfig, InputSystemSet};
        pub use crate::client::interpolation::interpolation_history::ConfirmedHistory;
        pub use crate::client::interpolation::plugin::{InterpolationConfig, InterpolationDelay};
        pub use crate::client::interpolation::{InterpolateStatus, Interpolated, LerpMode};
        pub use crate::client::plugin::{ClientPlugin, PluginConfig};
        pub use crate::client::prediction::plugin::PredictionConfig;
        pub use crate::client::prediction::predicted_history::{ComponentState, PredictionHistory};
        pub use crate::client::prediction::{Predicted, PredictionCommandsExt};
        pub use crate::client::resource::{Authentication, Client};
        pub use crate::client::sync::SyncConfig;
    }
    pub mod server {
        pub use crate::server::config::NetcodeConfig;
        pub use crate::server::config::ServerConfig;
        pub use crate::server::events::{
            ComponentInsertEvent, ComponentRemoveEvent, ComponentUpdateEvent, ConnectEvent,
            DisconnectEvent, EntityDespawnEvent, EntitySpawnEvent, InputEvent, MessageEvent,
        };
        pub use crate::server::plugin::{PluginConfig, ServerPlugin};
        pub use crate::server::resource::Server;
    }
}

pub mod channel;

pub mod client;

pub mod connection;

pub mod inputs;

pub mod netcode;

pub mod packet;

pub mod protocol;

pub mod serialize;

pub mod server;

pub mod shared;

#[cfg(test)]
pub(crate) mod tests;

/// Provides an abstraction over an unreliable transport
pub mod transport;

/// Extra utilities
pub mod utils;
