#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use frame_support::{
		sp_runtime::traits::Hash,
		traits::{ Randomness, Currency, tokens::ExistenceRequirement },
		transactional
	};
	use sp_io::hashing::blake2_128;

	#[cfg(feature = "std")]
	use serde::{Deserialize, Serialize};

	// ACTION #1: Write a Struct to hold Kitty information.

	// ACTION #2: Enum declaration for Gender.

	#[pallet::pallet]
	#[pallet::generate_store(trait Store)]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types it depends on.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// The Currency handler for the Kitties pallet.
		type Currency: Currency<Self::AccountId>;

		// ACTION #5: Specify the type for Randomness we want to specify for runtime.
	}

	// Errors.
	#[pallet::error]
	pub enum Error<T> {
		// TODO Part III
	}

	// Events.
	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		// TODO Part III
	}

	#[pallet::storage]
	#[pallet::getter(fn all_kitties_count)]
	pub(super) type AllKittiesCount<T: Config> = StorageValue<_, u64, ValueQuery>;

	// ACTION #6: Add Nonce storage item.

	// ACTION #9: Remaining storage items.

	// TODO Part IV: Our pallet's genesis configuration.

	#[pallet::call]
	impl<T: Config> Pallet<T> {

		// TODO Part III: create_kitty

		// TODO Part III: set_price

		// TODO Part III: transfer

		// TODO Part III: buy_kitty

		// TODO Part III: breed_kitty
	}

	// ACTION #4: helper function for Kitty struct

	impl<T: Config> Pallet<T> {
		// TODO Part III: helper functions for dispatchable functions

		// ACTION #7: increment_nonce helper

		// ACTION #8: random_hash helper

		// TODO: mint, transfer_from

	}
}