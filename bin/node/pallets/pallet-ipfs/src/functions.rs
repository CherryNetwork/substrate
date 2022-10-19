use frame_system::offchain::{SendSignedTransaction, Signer};
use sp_runtime::offchain::{
	ipfs::{ipfs_request, IpfsRequest},
	ipfs_types::{
		BlockRMResponse, BootstrapAddResponse, CatResponse, PeersResponse, PinResponse,
		UnPinResponse,
	},
	HttpError,
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
			log::info!("🗃️ ❌IPFS: {} entries in the data queue", len);
		}

		for cmd in data_queue.into_iter() {
			match cmd {
				// TODO(@charmitro): Add should just confirm that the cid exists as it
				// is supposed to be added from the cherry-ipfs-service
				// and add it to the account.
				DataCommand::AddBytes(m_addr, cid, size, account, recursive) => {
					let signer = Signer::<T, T::AuthorityId>::all_accounts();

					if !signer.can_sign() {
						log::error!(
									"🗃️ ❌ IPFS: No local accounts available. Consider adding one via `author_insertKey` RPC call.",
									);
					}

					let results =
						signer.send_signed_transaction(|_account| Call::submit_ipfs_add_results {
							admin: account.clone(),
							cid: cid.clone(),
							size,
						});

					for (_, res) in &results {
						match res {
							Ok(()) => {
								log::info!("🗃️ IPFS: Submitted IPFS Add results.");
							},
							Err(e) => {
								log::error!("🗃️ ❌ IPFS: Failed to submit IPSF Add results: {:?}", e)
							},
						}
					}
				},

				DataCommand::CatBytes(_m_address, cid, _account_id) => {
					match ipfs_request::<CatResponse>(IpfsRequest::Cat(cid)) {
						Ok(rsp) => log::info!("{:?}", rsp),
						Err(err) => log::error!("{:?}", err),
					}
				},

				DataCommand::InsertPin(_m_address, cid, _account_id, _recursive) =>
					match ipfs_request::<PinResponse>(IpfsRequest::Pin(cid.clone())) {
						Ok(_rsp) => {
							let signer = Signer::<T, T::AuthorityId>::all_accounts();

							if !signer.can_sign() {
								log::error!(
									"🗃️ ❌ IPFS: No local accounts available. Consider adding one via `author_insertKey` RPC call.",
									);
							}

							let results = signer.send_signed_transaction(|_account| {
								Call::submit_ipfs_pin_results { cid: cid.clone() }
							});

							for (_, res) in &results {
								match res {
									Ok(()) => {
										log::info!("🗃️ IPFS: Submitted IPFS Pin results.");
									},
									Err(e) => log::error!(
										"🗃️ ❌ IPFS: Failed to submit IPSF Pin results: {:?}",
										e
									),
								}
							}
						},
						Err(err) => log::error!("{:?}", err),
					},

				DataCommand::RemovePin(_m_addr, cid, _account_id, _recursive) =>
					match ipfs_request::<UnPinResponse>(IpfsRequest::UnPin(cid.clone())) {
						Ok(rsp) => {
							let signer = Signer::<T, T::AuthorityId>::all_accounts();

							if !signer.can_sign() {
								log::error!(
									"🗃️ ❌ IPFS: No local accounts available. Consider adding one via `author_insertKey` RPC call.",
									);
							}

							let results = signer.send_signed_transaction(|_account| {
								Call::submit_ipfs_unpin_results { cid: cid.clone() }
							});

							for (_, res) in &results {
								match res {
									Ok(()) => {
										log::info!("🗃️ IPFS: Submitted IPFS Unpin results.");
									},
									Err(e) => log::error!(
										"🗃️ ❌ IPFS: Failed to submit IPSF Unpin results: {:?}",
										e
									),
								}
							}
						},
						Err(err) => log::error!("{:?}", err),
					},

				DataCommand::RemoveBlock(_m_addr, cid, _account_id) => {
					match ipfs_request::<BlockRMResponse>(IpfsRequest::BlockRM(cid)) {
						Ok(rsp) => log::info!("{:?}", rsp),
						Err(err) => log::error!("{:?}", err),
					}
				},

				DataCommand::BootstrapAdd(m_address, _account_id) => {
					match ipfs_request::<BootstrapAddResponse>(IpfsRequest::BootstrapAdd(
						m_address.0,
					)) {
						Ok(resp) => log::info!("{:?}", resp),
						Err(err) => log::error!("{:?}", err),
					}
				},

				_ => {},
			}
		}

		Ok(())
	}

	pub fn print_metadata() -> Result<PeersResponse, HttpError> {
		match ipfs_request::<PeersResponse>(IpfsRequest::Peers) {
			Ok(resp) => Ok(resp),
			Err(err) => Err(err),
		}
	}
}
