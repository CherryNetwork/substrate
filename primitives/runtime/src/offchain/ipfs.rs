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

/// /stats/repo response
pub type CatResponse = Vec<u8>;

/// /stats/repo response
#[derive(Serialize, Deserialize, Debug)]
pub struct PinResponse {
	#[serde(alias = "Pins")]
	pins: serde_json::Value,
	#[serde(alias = "Progress")]
	progress: u64,
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
	Cat(CatResponse),
	Pin(PinResponse),
}

// Using the return type as a Generic. We can't use the request parameter
// as a generic due to the different URLs that we call. - @charmitro
pub fn ipfs_request<T>(request: IpfsRequest) -> Result<T, HttpError>
where
	T: for<'de> serde::Deserialize<'de>,
{
	let timeout = timestamp().add(Duration::from_millis(3000));

	match request {
		IpfsRequest::BitswapStats => {
			let url = "http://127.0.0.1:5001/api/v0/stats/bitswap";
			let request = http::Request::get(url).method(http::Method::Post);
			let pending = request.deadline(timeout).send()?;
			let response =
				pending.try_wait(timeout).map_err(|_| HttpError::DeadlineReached)?.unwrap();
			let json_response: T = serde_json::from_str::<T>(
				sp_std::str::from_utf8(&response.body().to_owned().collect::<Vec<u8>>())
					.map_err(|_| log::error!("Can't deser json response."))
					.unwrap(),
			)
			.unwrap();

			Ok(json_response)
		},

		IpfsRequest::RepoStats => {
			// TODO: Not sure about the address here. - @charmitro
			let url = "http://127.0.0.1:5001/api/v0/stats/repo";

			let request = http::Request::get(url).method(http::Method::Post);

			let pending = request.deadline(timeout).send()?;

			let resopnse =
				pending.try_wait(timeout).map_err(|_| HttpError::DeadlineReached)?.unwrap();

			let rsp: T = serde_json::from_str(
				sp_std::str::from_utf8(&resopnse.body().collect::<Vec<u8>>())
					.map_err(|_| log::error!("Can't deser json response."))
					.unwrap(),
			)
			.unwrap();

			Ok(rsp)
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

			let rsp: T = serde_json::from_str(
				sp_std::str::from_utf8(&response.body().collect::<Vec<u8>>())
					.map_err(|_| log::error!("Can't deser json response."))
					.unwrap(),
			)
			.unwrap();

			Ok(rsp)
		},

		IpfsRequest::Pin(cid) => {
			let mut address: scale_info::prelude::string::String =
				"http://127.0.0.1:5001/api/v0/cat?arg=".to_owned();
			let url: scale_info::prelude::string::String =
				scale_info::prelude::string::String::from_utf8(cid).unwrap();
			address.push_str(&url.to_owned());
			address.push_str(&"&recursive=true");

			let request = http::Request::get(address.as_str()).method(http::Method::Post);
			let pending = request.deadline(timeout).send()?;

			let response =
				pending.try_wait(timeout).map_err(|_| HttpError::DeadlineReached)?.unwrap();

			let rsp: T = serde_json::from_str(
				sp_std::str::from_utf8(&response.body().collect::<Vec<u8>>())
					.map_err(|_| log::error!("Can't deser json response."))
					.unwrap(),
			)
			.unwrap();

			Ok(rsp)
		},
	}
}

// TODO: add IPFS tests - @charmitro
#[cfg(test)]
mod tests {
	use super::*;
	use sp_core::offchain::{testing, OffchainWorkerExt};
	use sp_io::TestExternalities;

	#[test]
	fn should_send_a_basic_request_and_get_response() {
		let (offchain, state) = testing::TestOffchainExt::new();
		let mut t = TestExternalities::default();
		t.register_extension(OffchainWorkerExt::new(offchain));

		t.execute_with(|| {
			let request: Request = Request::get("http://localhost:1234");
			let pending = request.add_header("X-Auth", "hunter2").send().unwrap();
			// make sure it's sent correctly
			state.write().fulfill_pending_request(
				0,
				testing::PendingRequest {
					method: "GET".into(),
					uri: "http://localhost:1234".into(),
					headers: vec![("X-Auth".into(), "hunter2".into())],
					sent: true,
					..Default::default()
				},
				b"1234".to_vec(),
				None,
			);

			// wait
			let mut response = pending.wait().unwrap();

			// then check the response
			let mut headers = response.headers().into_iter();
			assert_eq!(headers.current(), None);
			assert_eq!(headers.next(), false);
			assert_eq!(headers.current(), None);

			let body = response.body();
			assert_eq!(body.clone().collect::<Vec<_>>(), b"1234".to_vec());
			assert_eq!(body.error(), &None);
		})
	}
}
