#![cfg_attr(no(feature="std"),"no_std")]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_support::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_support::Config>::Event>;
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config>{
        ClaimCreated{who: T::AccountId, claim: T::Hash},
        ClaimRevoked{who: T::AccountId, claim: T::Hash},
    }

    #[pallet::error]
    pub enum Error<T> {
        AlreadyClaimed,
        NoSuchClaim,
        NotClaimOwner,
    }
    
    #[pallet::storage]
}