use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub(crate) up_streamer_config: UpStreamerConfig,
    pub(crate) host_config: HostConfig,
    pub(crate) usubscription_config: USubscriptionConfig,
    pub(crate) zenoh_transport_config: ZenohTransportConfig,
    pub(crate) someip_config: SomeipConfig,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct UpStreamerConfig {
    pub(crate) message_queue_size: u16,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct HostConfig {
    pub(crate) transport: HostTransport,
    pub(crate) authority: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct USubscriptionConfig {
    pub(crate) file_path: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct ZenohTransportConfig {
    pub(crate) config_file: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct SomeipConfig {
    pub(crate) authority: String,
    pub(crate) config_file: PathBuf,
    pub(crate) default_someip_application_id_for_someip_subscriptions: u16,
    pub(crate) enabled: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum HostTransport {
    Zenoh,
}
