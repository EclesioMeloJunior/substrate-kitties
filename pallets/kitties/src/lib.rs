#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{
		sp_runtime::traits::{Hash, Zero},
		dispatch::{DispatchResultWithPostInfo, DispatchResult},
		traits::{Currency, ExistenceRequirement, Randomness, IsType},
		pallet_prelude::*,
	};
	use frame_system::pallet_prelude::*;
	use sp_io::hashing::blake2_128;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// Configure the pallet specifying the parameters and types it depends on
	#[pallet::config]
	pub trait Config: frame_system::Config {
		// Because this pallet emit events, it depends on the runtime's
		// definition of an Event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		
		// The Currency handler fot the Kitties pallet.
		type Currency: Currency<Self::AccountId>;
	}

	#[pallet::error]
	pub enum Error<T> {}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {}

	#[pallet::storage]
	#[pallet::getter(fn kitty_cnt)]
	pub(super) type KittyCnt<T: Config> = StorageValue<_, u64, ValueQuery>;

	#[pallet::call]
	impl <T: Config> Pallet<T> {}

	impl <T: Config> Pallet<T> {}
}