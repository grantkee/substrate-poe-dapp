#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{pallet_prelude::*, storage::bounded_vec::BoundedVec};
	use frame_system::pallet_prelude::*;

	// The struct on which we build all of our Pallet logic.
	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/* Placeholder for defining custom types. */

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// This pallet emits events and must depend on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		/// Constrain the maximum bytes of a hash used for any proof.
		type MaxBytesInHash: Get<u32>;
	}

	// Pallets use events to inform users when important changes are made.
	// Event documentation should end with an array that provides descriptive 
	// names for parameters. docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event emitted when a proof is claimed. [who, claim]
		ClaimCreated(T::AccountId, BoundedVec<u8, T::MaxBytesInHash>),
		/// Event emitted when a claim is revocked by the owner. [who, claim]
		ClaimRevoked(T::AccountId, BoundedVec<u8, T::MaxBytesInHash>),
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Proof already claimed.
		ProofAlreadyClaimed,
		/// Proof does not exist.
		NoSuchProof,
		/// Proof is claimed by another account.
		NotProofOwner,
	}

	// use FRAME StorageMap trait for HashMap
	#[pallet::storage]
	/// Map each proof to its owner and block number
	/// for when the proof was made.
	pub(super) type Proofs<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		BoundedVec<u8, T::MaxBytesInHash>,
		(T::AccountId, T::BlockNumber),
		OptionQuery,
	>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(1_000)]
		pub fn create_claim(
			origin: OriginFor<T>,
			proof: BoundedVec<u8, T::MaxBytesInHash>,
		) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// Return an error if the extrinsic is not signed.
			let sender = ensure_signed(origin)?;

			// verify the specified proof is not yet claimed
			ensure!(!Proofs::<T>::contains_key(&proof), Error::<T>::ProofAlreadyClaimed);

			// get the block number from the FRAME System Pallet
			let current_block = <frame_system::Pallet<T>>::block_number();

			// add proof = sender:block_number
			Proofs::<T>::insert(&proof, (&sender, current_block));

			// emit claim created event
			Self::deposit_event(Event::ClaimCreated(sender, proof));

			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn revoke_claim(
			origin: OriginFor<T>,
			proof: BoundedVec<u8, T::MaxBytesInHash>,
		) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// Return an error if the extrinsic is not signed.
			let sender = ensure_signed(origin)?;

			// verify the specified proof is already claimed
			ensure!(Proofs::<T>::contains_key(&proof), Error::<T>::NoSuchProof);

			// get owner of the claim
			// key cannot be `None` owner, so this must always unwrap
			let (owner, _) = Proofs::<T>::get(&proof).expect("All proofs must have an owner!");

			// Verify that sender if the current call is also the claim owner
			ensure!(sender == owner, Error::<T>::NotProofOwner);

			// Remove claim from storage
			Proofs::<T>::remove(&proof);

			// emit claim revoked event
			Self::deposit_event(Event::ClaimRevoked(sender, proof));

			Ok(())
		}
	}
}
