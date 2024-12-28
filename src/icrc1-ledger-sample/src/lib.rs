use candid::{CandidType, Decode, Encode, Principal};
use ic_cdk::api::is_controller;
use ic_cdk::caller;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{storable::Bound, DefaultMemoryImpl, StableCell, Storable};
use icrc_ledger_types::icrc1::account::Account;
use icrc_ledger_types::icrc1::transfer::{BlockIndex, NumTokens, TransferArg, TransferError};
use icrc_ledger_types::icrc2::transfer_from::{TransferFromArgs, TransferFromError};
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, cell::RefCell};

#[derive(CandidType, Deserialize, Serialize)]
pub struct TransferArguments {
    to: Account,
    amount: NumTokens,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct TransferFromArguments {
    from: Account,
    to: Account,
    amount: NumTokens,
}

const MAX_VALUE_SIZE: u32 = 31; // Principal(30) + bool(1)

#[derive(CandidType, Clone, Deserialize, Serialize)]
pub struct State {
    icrc1_ledger_id: Principal,
    initialized: bool,
}

impl State {
    pub fn new() -> Self {
        Self {
            icrc1_ledger_id: Principal::anonymous(),
            initialized: false,
        }
    }
}

impl Storable for State {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: MAX_VALUE_SIZE,
        is_fixed_size: true,
    };
}

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static STATE: RefCell<StableCell<State, VirtualMemory<DefaultMemoryImpl>>> = RefCell::new(
        StableCell::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),
            State::new()
        ).unwrap()
    );
}

#[ic_cdk::update]
fn set_icrc1_ledger(ledger_id: Principal) {
    if ledger_id == Principal::anonymous() {
        panic!("Cannot set an anonymous principal");
    }

    if !is_controller(&caller()) {
        panic!("Only the controllers can set the ICRC-1 ledger.");
    }

    STATE.with_borrow_mut(|state| {
        let mut temp_state = state.get().clone();
        temp_state.icrc1_ledger_id = ledger_id;
        temp_state.initialized = true;
        state.set(temp_state).unwrap();
    });
}

#[ic_cdk::query]
fn get_icrc1_ledger() -> Principal {
    STATE.with(|state| state.borrow().get().icrc1_ledger_id)
}

#[ic_cdk::update]
async fn transfer(args: TransferArguments) -> Result<BlockIndex, TransferError> {
    if !STATE.with(|state| state.borrow().get().initialized) {
        panic!("Please call 'set_icrc1_ledger' to initialize the IRCR-1 ledger first.");
    }

    if !is_controller(&caller()) {
        panic!("Only the controllers can call 'transfer' function.");
    }

    let transfer_arg: TransferArg = TransferArg {
        from_subaccount: None, // Subaccount with 0.
        to: args.to,
        amount: args.amount,
        fee: None, // default fee
        memo: None,
        created_at_time: None,
    };

    ic_cdk::call::<(TransferArg,), (Result<BlockIndex, TransferError>,)>(
        STATE.with(|state| state.borrow().get().icrc1_ledger_id),
        "icrc1_transfer",
        (transfer_arg,),
    )
    .await
    .unwrap()
    .0
}

#[ic_cdk::update]
async fn transfer_from(args: TransferFromArguments) -> Result<BlockIndex, TransferFromError> {
    if !STATE.with(|state| state.borrow().get().initialized) {
        panic!("Please call 'set_icrc1_ledger' to initialize the IRCR-1 ledger first.");
    }

    if !is_controller(&caller()) {
        panic!("Only the controllers can call 'transfer_from' function.");
    }

    let transfer_arg: TransferFromArgs = TransferFromArgs {
        spender_subaccount: None, // Subaccount with 0.
        from: args.from,
        to: args.to,
        amount: args.amount,
        fee: None, // default fee
        memo: None,
        created_at_time: None,
    };

    ic_cdk::call::<(TransferFromArgs,), (Result<BlockIndex, TransferFromError>,)>(
        STATE.with(|state| state.borrow().get().icrc1_ledger_id),
        "icrc2_transfer_from",
        (transfer_arg,),
    )
    .await
    .unwrap()
    .0
}

ic_cdk::export_candid!();
