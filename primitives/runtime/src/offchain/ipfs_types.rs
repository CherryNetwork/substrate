use crate::offchain::http;
use serde::{Deserialize, Serialize};
use serde_json;
use sp_core::offchain::{Duration, HttpError};
use sp_io::offchain::timestamp;
use sp_std::{borrow::ToOwned, vec::Vec};

/// /stats/bitswap response
#[derive(Serialize, Deserialize, Debug)]
pub struct BitswapStatsResponse {
	#[serde(alias = "BlocksReceived")]
	blocks_received: u64,
	#[serde(alias = "BlocksSent")]
	blocks_sent: u64,
	#[serde(alias = "DataReceived")]
	data_received: u64,
	#[serde(alias = "DataSent")]
	data_sent: u64,
	#[serde(alias = "DupBlksReceived")]
	dup_blks_received: u64,
	#[serde(alias = "DupDataReceived")]
	dup_data_received: u64,
	#[serde(alias = "MessagesReceived")]
	messages_received: u64,
	#[serde(alias = "Peers")]
	peers: serde_json::Value,
	#[serde(alias = "ProvideBufLen")]
	provide_buf_len: u64,
	#[serde(alias = "Wantlist")]
	wantlist: serde_json::Value,
}

/// /api/v0/stats/repo response
#[derive(Serialize, Deserialize, Debug)]
pub struct RepoStatsResponse {
	#[serde(alias = "NumObjects")]
	num_objects: u64,
	#[serde(with = "serde_bytes")]
	#[serde(alias = "RepoPath")]
	repo_path: Vec<u8>,
	#[serde(alias = "RepoSize")]
	repo_size: u64,
	#[serde(alias = "StorageMax")]
	storage_max: u64,
	#[serde(with = "serde_bytes")]
	#[serde(alias = "Version")]
	version: Vec<u8>,
}

/// /api/v0/cat response
pub type CatResponse = Vec<u8>;

/// /api/v0/pin/add response
#[derive(Serialize, Deserialize, Debug)]
pub struct PinResponse {
	#[serde(alias = "Pins")]
	pins: serde_json::Value,
	#[serde(alias = "Progress")]
	progress: u64,
}

/// /api/v0/pin/rm response
#[derive(Serialize, Deserialize, Debug)]
pub struct UnPinResponse {
	#[serde(alias = "Pins")]
	pins: serde_json::Value,
}

/// /api/v0/pin/rm response
#[derive(Serialize, Deserialize, Debug)]
pub struct BlockRMResponse {
	#[serde(with = "serde_bytes")]
	#[serde(alias = "Error")]
	error: Vec<u8>,
	#[serde(with = "serde_bytes")]
	#[serde(alias = "Hash")]
	hash: Vec<u8>,
}

/// /api/v0/pin/rm response
#[derive(Serialize, Deserialize, Debug)]
pub struct BootstrapAddResponse {
	#[serde(alias = "Peers")]
	peers: serde_json::Value,
}

/// /api/v0/pin/rm response
#[derive(Serialize, Deserialize, Debug)]
pub struct BootstrapRMResponse {
	#[serde(alias = "Peers")]
	peers: serde_json::Value,
}
