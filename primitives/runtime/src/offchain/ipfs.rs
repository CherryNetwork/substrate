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
use sp_core::offchain::Duration;
use sp_io::offchain::timestamp;
use sp_std::vec::Vec;

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
	RepoStats,
	Cat,
	Pin(Vec<u8>),
}

pub fn start_ipfs_request(request: IpfsRequest) {
	match request {
		IpfsRequest::RepoStats => {
			let request = http::Request::get("http://127.0.0.1:5001/api/v0/stats/repo")
				.method(http::Method::Post);

			let timeout = timestamp().add(Duration::from_millis(3000));
			let pending =
				request.deadline(timeout).send().map_err(|_| http::Error::IoError).unwrap();

			let resopnse = pending
				.try_wait(timeout)
				.map_err(|_| http::Error::DeadlineReached)
				.unwrap()
				.unwrap();

			let rsp: RepoStatsResponse = serde_json::from_str(
				sp_std::str::from_utf8(&resopnse.body().collect::<Vec<u8>>())
					.map_err(|_| log::error!("Can't deser json response."))
					.unwrap(),
			)
			.unwrap();

			log::info!("{:?}\n", rsp);
		},
		IpfsRequest::Cat => todo!(),
		IpfsRequest::Pin(cid) => todo!(),
	}
}
