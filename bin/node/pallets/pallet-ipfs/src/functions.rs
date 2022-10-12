use sp_runtime::offchain::{
	ipfs::{ipfs_request, IpfsRequest},
	ipfs_types::{BlockRMResponse, CatResponse, PinResponse, UnPinResponse},
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

	pub fn handle_data_requests() -> Result<(), Error<T>> {
		let data_queue = DataQueue::<T>::get();
		let len = data_queue.len();

		if len != 0 {
			log::info!("IPFS: {} entries in the data queue", len);
		}

		for cmd in data_queue.into_iter() {
			match cmd {
				DataCommand::CatBytes(_m_address, cid, _account_id) => {
					match ipfs_request::<CatResponse>(IpfsRequest::Cat(cid)) {
						Ok(rsp) => log::info!("{:?}", rsp),
						Err(err) => log::error!("{:?}", err),
					}
				},
				DataCommand::InsertPin(_m_address, cid, _account_id, _recursive) =>
					match ipfs_request::<PinResponse>(IpfsRequest::Cat(cid)) {
						Ok(rsp) => log::info!("{:?}", rsp),
						Err(err) => log::error!("{:?}", err),
					},
				DataCommand::RemovePin(_m_addr, cid, _account_id, _recursive) =>
					match ipfs_request::<UnPinResponse>(IpfsRequest::UnPin(cid)) {
						Ok(rsp) => log::info!("{:?}", rsp),
						Err(err) => log::error!("{:?}", err),
					},
				DataCommand::RemoveBlock(_m_addr, cid, _account_id) => {
					match ipfs_request::<BlockRMResponse>(IpfsRequest::BlockRM(cid)) {
						Ok(rsp) => log::info!("{:?}", rsp),
						Err(err) => log::error!("{:?}", err),
					}
				},
				_ => {},
			}
		}
		Ok(())
	}
}
