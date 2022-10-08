// allow the program to compile in native rust binary
// and also Wasm, all should have the no-std attribute
#![cfg_attr(not(feature = "std"), no_std)]

// import the pallet library so as to be able
// to use the related methods from the crate
pub use pallet::*;

// start the pallet modules and import the
// frame support macros for help
#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	// define the pallet class to hold
	// an underlying struct for our implementation
	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// define the config trait which every pallet
	// needs to define the parameters & types in its function
	#[pallet::config]
	pub trait Config: frame_system::Config {
		// define the events that would be captured
		// for each function and parameter type
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	// define the storage runtime pallet macro
	// this should hold what we want to store
	#[pallet::storage]
	pub(super) type Claims<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::Hash,
		(T::AccountId, T::BlockNumber)
	>;

	// define the events dispatcher macro, this should
	// throw certain events based on the action taken
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		ClaimCreated {
			who: T::AccountId,
			claim: T::Hash,
		},
		ClaimRevoked {
			who: T::AccountId,
			claim: T::Hash,
		},
	}

	// define the macros for the runtime errors
	// should be an enum for errors
	#[pallet::error]
	pub enum Error<T> {
		AlreadyClaimed,
		NoSuchClaim,
		NotClaimOwner,
	}

	// define the pallet call macro which
	// would hold the core functions and function
	// definition of the proof of existence.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		// users have to pay a certain gas fee
		// in order to perform an extrinsic(off chain)
		// activity in our Blockchain
		#[pallet::weight(0)]
		pub fn create_claims(origin: OriginFor<T>, claim: T::Hash) -> DispatchResult {
			// check if the sender of the digital file signature
			// is a signed user else return false.
			let sender = ensure_signed(origin)?;

			// also ensure that the hash being passed is not
			// already stored in our blockchain database
			ensure!(!Claims::<T>::contains_key(&claim), Error::<T>::AlreadyClaimed);

			// since at this point we have verified that the claim
			// does not exist [assuming the logic above does not throw an error]
			// then we insert the new claim into our Blockchain store
			let current_block = <frame_system::Pallet<T>>::block_number();
			Claims::<T>::insert(&claim, (&sender, current_block));

			// Dispatch an event telling the system that a claim has been created
			// this should be recorded as a state of transition in our block
			Self::deposit_event(Event::ClaimCreated {
				who: sender,
				claim,
			});
            Ok(())
		}
	}
}