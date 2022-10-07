#![cfg_attr(no(feature = "std"), "no_std")]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

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

	#[pallet::error]
	pub enum Error<T> {
		AlreadyClaimed,
		NoSuchClaim,
		NotClaimOwner,
	}

	#[pallet::storage]
	pub(super) type Claims<T: Config> = StorageMap<
		_,
		Blake2_12concat,
		T::Hash,
		(T::AccountId, T::BlockNumber)
	>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
        pub fn create_claim(origin: OriginFor<T: AccountId>, claim:T::Hash) -> DispatchResult {

            // ensure that the user to create the claim is signed
            // if not signed then return error
            let sender = ensure_signed(origin)?;

            // double check that the specified claim does not already
            // exist, else return the Already claimed error enum
            ensure!(!Claims::<T>::contains_key(&claim), Error::<T>::AlreadyClaimed);

            // get the block number from the framesystem pallet
            let current_block = <frame_system::Pallet<T>>::block_number();

            // insert the new claim into storage and emit an event 
            Claims::<T>::insert(&claim, (&sender, current_block));
            Self::deposit_event(Event::ClaimCreated{who: sender, claim});
            Ok(());
        }
    }
}