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

    pub fn revoke_claim(origin: OriginFor<T:AccountId>, claim: T:Hash){
        // verify that the user is signed
        let sender = ensure_signed(origin)?;

        // get the claims from storage if exists
        // else throw an error of no such claim
        let (owner, _ ) = Claims::<T>::get(&claim).ok_or(Error::<T>::NoSuchClaim)?;

        // ensure that the sender is also the owner
        ensure!(sender == owner, Error::<T>::NotClaimOwner);

        // remove the claim
        Claims::<T>::remove(&claim);

        // emit an event showing that a claim was removed.
        Self::deposit_event(Event::ClaimRevoked(who: sender, claim));
        Ok(())
    }
}