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
use serde_json;
use sp_core::offchain::{Duration, HttpError};
use sp_io::offchain::timestamp;
use sp_std::{borrow::ToOwned, vec::Vec};

use super::ipfs_types::{
	BitswapStatsResponse, BlockRMResponse, BootstrapAddResponse, BootstrapRMResponse, CatResponse,
	PeerIdConfigResponse, PinResponse, RepoStatsResponse, UnPinResponse,
};

/// Type of implemented IPFS Requests
pub enum IpfsRequest {
	/// Returns the selected PeerID IPFS Configuration
	PeerIdConfig,
	/// Returns the selected PeerID IPFS Storage Configuration
	StorageConfig,
	/// Returns all connected Peers
	Peers,
	/// Returns the selected PeerID IPFS Bitswap stats
	BitswapStats,
	/// Returns the selected PeerID IPFS Repo stats
	RepoStats,
	/// Cat an IPFS Object based on the CID
	Cat(Vec<u8>),
	/// Pin an IPFS Object based on the CID
	Pin(Vec<u8>),
	/// Unpin an IPFS Object based on the CID
	UnPin(Vec<u8>),
	/// Remove Block a.k.a. file
	BlockRM(Vec<u8>),
	/// Add bootstrap Peer to network
	BootstrapAdd(Vec<u8>),
	/// Remove bootstrap Peer from network
	BootstrapRM(Vec<u8>),
}

/// Type of implemented IPFS Responses
pub enum IpfsResponse {
	/// Returns the selected PeerID IPFS Configuration
	PeerIdConfig(PeerIdConfigResponse),
	/// Returns the selected PeerID IPFS Storage Configuration
	StorageConfig,
	/// Returns all connected Peers
	Peers,
	/// Returns the selected PeerID IPFS Bitswap stats
	BitswapStats(BitswapStatsResponse),
	/// Returns the selected PeerID IPFS Repo stats
	RepoStats(RepoStatsResponse),
	/// Cat an IPFS Object based on the CID
	Cat(CatResponse),
	/// Pin an IPFS Object based on the CID
	Pin(PinResponse),
	/// Unpin an IPFS Object based on the CID
	UnPin(UnPinResponse),
	/// Remove Block a.k.a. file
	BlockRM(BlockRMResponse),
	/// Add bootstrap Peer to network
	BootstrapAdd(BootstrapAddResponse),
	/// Remove bootstrap Peer from network
	BootstrapRM(BootstrapRMResponse),
}

// Using the return type as a Generic. We can't use the request parameter
// as a generic due to the different URLs that we call. - @charmitro
/// Performs an IPFS requests based on enums IpfsRequest
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
			let response =
				pending.try_wait(timeout).map_err(|_| HttpError::DeadlineReached)?.unwrap();

			let json_response: T = serde_json::from_str(
				sp_std::str::from_utf8(&response.body().collect::<Vec<u8>>())
					.map_err(|_| log::error!("Can't deser json response."))
					.unwrap(),
			)
			.unwrap();

			Ok(json_response)
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

			log::info!(
				"{:?}",
				sp_std::str::from_utf8(&response.body().collect::<Vec<u8>>()).unwrap()
			);
			let json_response: T = serde_json::from_str(
				sp_std::str::from_utf8(&response.body().collect::<Vec<u8>>())
					.map_err(|_| log::error!("Can't deser json response."))
					.unwrap(),
			)
			.unwrap();

			Ok(json_response)
		},

		IpfsRequest::Pin(cid) => {
			let mut address: scale_info::prelude::string::String =
				"http://127.0.0.1:5001/api/v0/pin/add?arg=".to_owned();
			let url: scale_info::prelude::string::String =
				scale_info::prelude::string::String::from_utf8(cid).unwrap();
			address.push_str(&url.to_owned());
			address.push_str(&"&recursive=true");

			let request = http::Request::get(address.as_str()).method(http::Method::Post);
			let pending = request.deadline(timeout).send()?;
			let response =
				pending.try_wait(timeout).map_err(|_| HttpError::DeadlineReached)?.unwrap();

			let json_response: T = serde_json::from_str(
				sp_std::str::from_utf8(&response.body().collect::<Vec<u8>>())
					.map_err(|_| log::error!("Can't deser json response."))
					.unwrap(),
			)
			.unwrap();

			Ok(json_response)
		},

		IpfsRequest::UnPin(cid) => {
			let mut address: scale_info::prelude::string::String =
				"http://127.0.0.1:5001/api/v0/pin/rm?arg=".to_owned();
			let url: scale_info::prelude::string::String =
				scale_info::prelude::string::String::from_utf8(cid).unwrap();
			address.push_str(&url.to_owned());
			address.push_str(&"&recursive=true");

			let request = http::Request::get(address.as_str()).method(http::Method::Post);
			let pending = request.deadline(timeout).send()?;
			let response =
				pending.try_wait(timeout).map_err(|_| HttpError::DeadlineReached)?.unwrap();

			let json_response: T = serde_json::from_str(
				sp_std::str::from_utf8(&response.body().collect::<Vec<u8>>())
					.map_err(|_| log::error!("Can't deser json response."))
					.unwrap(),
			)
			.unwrap();

			Ok(json_response)
		},

		IpfsRequest::BlockRM(cid) => {
			let mut address: scale_info::prelude::string::String =
				"http://127.0.0.1:5001/api/v0/pin/rm?arg=".to_owned();
			let url: scale_info::prelude::string::String =
				scale_info::prelude::string::String::from_utf8(cid).unwrap();
			address.push_str(&url.to_owned());
			address.push_str(&"&recursive=true");

			let request = http::Request::get(address.as_str()).method(http::Method::Post);
			let pending = request.deadline(timeout).send()?;
			let response =
				pending.try_wait(timeout).map_err(|_| HttpError::DeadlineReached)?.unwrap();

			let json_response: T = serde_json::from_str(
				sp_std::str::from_utf8(&response.body().collect::<Vec<u8>>())
					.map_err(|_| log::error!("Can't deser json response."))
					.unwrap(),
			)
			.unwrap();

			Ok(json_response)
		},

		IpfsRequest::BootstrapAdd(multiaddr_peer) => {
			let mut address: scale_info::prelude::string::String =
				"http://127.0.0.1:5001/api/v0/bootstrap/add?arg=".to_owned();
			let url: scale_info::prelude::string::String =
				scale_info::prelude::string::String::from_utf8(multiaddr_peer).unwrap();
			address.push_str(&url.to_owned());
			address.push_str(&"&recursive=true");

			let request = http::Request::get(address.as_str()).method(http::Method::Post);
			let pending = request.deadline(timeout).send()?;
			let response =
				pending.try_wait(timeout).map_err(|_| HttpError::DeadlineReached)?.unwrap();

			let json_response: T = serde_json::from_str(
				sp_std::str::from_utf8(&response.body().collect::<Vec<u8>>())
					.map_err(|_| log::error!("Can't deser json response."))
					.unwrap(),
			)
			.unwrap();

			Ok(json_response)
		},

		IpfsRequest::BootstrapRM(multiaddr_peer) => {
			let mut address: scale_info::prelude::string::String =
				"http://127.0.0.1:5001/api/v0/bootstrap/rm?arg=".to_owned();
			let url: scale_info::prelude::string::String =
				scale_info::prelude::string::String::from_utf8(multiaddr_peer).unwrap();
			address.push_str(&url.to_owned());
			address.push_str(&"&recursive=true");

			let request = http::Request::get(address.as_str()).method(http::Method::Post);
			let pending = request.deadline(timeout).send()?;
			let response =
				pending.try_wait(timeout).map_err(|_| HttpError::DeadlineReached)?.unwrap();

			let json_response: T = serde_json::from_str(
				sp_std::str::from_utf8(&response.body().collect::<Vec<u8>>())
					.map_err(|_| log::error!("Can't deser json response."))
					.unwrap(),
			)
			.unwrap();

			Ok(json_response)
		},

		IpfsRequest::Peers => {
			let address: scale_info::prelude::string::String =
				"http://127.0.0.1:5001/api/v0/swarm/peers".to_owned();

			let request = http::Request::get(address.as_str()).method(http::Method::Post);
			let pending = request.deadline(timeout).send()?;
			let response =
				pending.try_wait(timeout).map_err(|_| HttpError::DeadlineReached)?.unwrap();

			let json_response: T = serde_json::from_str(
				sp_std::str::from_utf8(&response.body().collect::<Vec<u8>>())
					.map_err(|_| log::error!("Can't deser json response."))
					.unwrap(),
			)
			.unwrap();

			Ok(json_response)
		},

		IpfsRequest::PeerIdConfig => {
			let address: scale_info::prelude::string::String =
				"http://127.0.0.1:5001/api/v0/config?arg=Identity.PeerID".to_owned();

			let request = http::Request::get(address.as_str()).method(http::Method::Post);
			let pending = request.deadline(timeout).send()?;
			let response =
				pending.try_wait(timeout).map_err(|_| HttpError::DeadlineReached)?.unwrap();

			let json_response: T = serde_json::from_str(
				sp_std::str::from_utf8(&response.body().collect::<Vec<u8>>())
					.map_err(|_| log::error!("Can't deser json response."))
					.unwrap(),
			)
			.unwrap();

			Ok(json_response)
		},

		IpfsRequest::StorageConfig => {
			let address: scale_info::prelude::string::String =
				"http://127.0.0.1:5001/api/v0/config?arg=Datastore.StorageMax".to_owned();

			let request = http::Request::get(address.as_str()).method(http::Method::Post);
			let pending = request.deadline(timeout).send()?;
			let response =
				pending.try_wait(timeout).map_err(|_| HttpError::DeadlineReached)?.unwrap();

			let json_response: T = serde_json::from_str(
				sp_std::str::from_utf8(&response.body().collect::<Vec<u8>>())
					.map_err(|_| log::error!("Can't deser json response."))
					.unwrap(),
			)
			.unwrap();

			Ok(json_response)
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
