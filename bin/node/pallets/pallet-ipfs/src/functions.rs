use sp_core::offchain::Duration;
use sp_io::offchain::timestamp;
use sp_runtime::offchain::{
	http,
	ipfs::{self, IpfsRequest, IpfsResponse},
};

use super::*;

impl<T: Config> Pallet<T> {
	pub fn retrieve_bytes(_public_key: Bytes, _signature: Bytes, message: Bytes) -> Bytes {
		let message_vec: Vec<u8> = message.to_vec();
		if let Some(data) =
			sp_io::offchain::local_storage_get(StorageKind::PERSISTENT, &message_vec)
		{
			Bytes(data.clone())
		} else {
			Bytes(Vec::new())
		}
	}

	pub fn determine_account_ownership_layer(
		cid: &Vec<u8>,
		acct: &T::AccountId,
	) -> Result<OwnershipLayer, Error<T>> {
		match Self::ipfs_asset(cid) {
			Some(ipfs) =>
				if let Some(layer) = ipfs.owners.get_key_value(acct) {
					Ok(layer.1.clone())
				} else {
					Err(<Error<T>>::AccNotExist)
				},
			None => Err(<Error<T>>::IpfsNotExist),
		}
	}

	// Leaving this here as a reference. - @charmitro
	pub fn ipfs_repo_stats() {
		match ipfs::ipfs_request(IpfsRequest::RepoStats) {
			Ok(rsp) => {
				match rsp {
					IpfsResponse::RepoStats(peos) => log::info!("{:?}", peos),
					_ => {},
				};
			},
			Err(_) => todo!(),
		}
	}

	// pub fn ipfs_request(
	// 	req: IpfsRequest,
	// 	deadline: impl Into<Option<Timestamp>>,
	// ) -> Result<IpfsResponse, Error<T>> {
	// 	let ipfs_request =
	// 		ipfs::PendingRequest::new(req).map_err(|_| Error::<T>::CantCreateRequest)?;

	// 	log::info!("{:?}", ipfs_request.request);

	// 	ipfs_request
	// 		.try_wait(deadline)
	// 		.map_err(|_| Error::<T>::RequestTimeout)?
	// 		.map(|r| r.response)
	// 		.map_err(|e| {
	// 			if let ipfs::Error::IoError(err) = e {
	// 				log::error!("IPFS Request failed: {}", sp_std::str::from_utf8(&err).unwrap());
	// 			} else {
	// 				log::error!("IPFS Request failed: {:?}", e);
	// 			}
	// 			Error::<T>::RequestFailed
	// 		})
	// }

	pub fn handle_data_requests() -> Result<(), Error<T>> {
		let data_queue = DataQueue::<T>::get();
		let len = data_queue.len();

		if len != 0 {
			log::info!("IPFS: {} entries in the data queue", len);
		}

		for cmd in data_queue.into_iter() {
			match cmd {
				DataCommand::CatBytes(_m_address, cid, _account_id) => {
					match ipfs::ipfs_request(IpfsRequest::Cat(cid)) {
						Ok(rsp) => {
							match rsp {
								IpfsResponse::Cat(body) => log::info!("{:?}", body),
								_ => {},
							};
						},
						Err(_) => todo!(),
					}
				},
				_ => {},
			}
		}
		Ok(())
	}
}
