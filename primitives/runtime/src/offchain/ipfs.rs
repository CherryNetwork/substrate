// This file is part of Cherry Networks.

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! A high-level helpers for making IPFS requests from within Offchain Workers.

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

/// /stats/repo response
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

pub enum IpfsRequest {
	BitswapStats,
	RepoStats,
	Cat(Vec<u8>),
	Pin(Vec<u8>),
}

pub enum IpfsResponse {
	BitswapStats(BitswapStatsResponse),
	RepoStats(RepoStatsResponse),
	Cat(Vec<u8>),
	Pin,
}

pub fn ipfs_request(request: IpfsRequest) -> Result<IpfsResponse, HttpError> {
	let timeout = timestamp().add(Duration::from_millis(3000));

	match request {
		IpfsRequest::BitswapStats => {
			let url = "http://127.0.0.1:5001/api/v0/stats/bitswap";
			let request = http::Request::get(url).method(http::Method::Post);
			let pending = request.deadline(timeout).send()?;
			let response =
				pending.try_wait(timeout).map_err(|_| HttpError::DeadlineReached)?.unwrap();
			let json_response = serde_json::from_str(
				sp_std::str::from_utf8(&response.body().collect::<Vec<u8>>())
					.map_err(|_| log::error!("Can't deser json response."))
					.unwrap(),
			)
			.unwrap();

			Ok(IpfsResponse::BitswapStats(json_response))
		},

		IpfsRequest::RepoStats => {
			// TODO: Not sure about the address here. - @charmitro
			let url = "http://127.0.0.1:5001/api/v0/stats/repo";

			let request = http::Request::get(url).method(http::Method::Post);

			let pending = request.deadline(timeout).send()?;

			let resopnse =
				pending.try_wait(timeout).map_err(|_| HttpError::DeadlineReached)?.unwrap();

			let rsp: RepoStatsResponse = serde_json::from_str(
				sp_std::str::from_utf8(&resopnse.body().collect::<Vec<u8>>())
					.map_err(|_| log::error!("Can't deser json response."))
					.unwrap(),
			)
			.unwrap();

			Ok(IpfsResponse::RepoStats(rsp))
		},
		IpfsRequest::Cat(cid) => {
			let mut address: scale_info::prelude::string::String =
				"http://127.0.0.1:5001/api/v0/cat?arg=".to_owned();
			let url: scale_info::prelude::string::String =
				scale_info::prelude::string::String::from_utf8(cid).unwrap();

			address.push_str(&url.to_owned());

			let request = http::Request::get(address.as_str()).method(http::Method::Post);
			let pending = request.deadline(timeout).send()?;
			let response =
				pending.try_wait(timeout).map_err(|_| HttpError::DeadlineReached)?.unwrap();

			Ok(IpfsResponse::Cat(response.body().collect::<Vec<u8>>()))
		},
		IpfsRequest::Pin(_cid) => todo!(),
	}
}
