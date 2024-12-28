use candid::{CandidType, Principal};
use ic_cdk::api::is_controller;
use ic_cdk::caller;
use icrc_ledger_types::icrc1::account::Account;
use icrc_ledger_types::icrc1::transfer::{BlockIndex, NumTokens, TransferArg, TransferError};
use icrc_ledger_types::icrc2::transfer_from::{TransferFromArgs, TransferFromError};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;

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

#[derive(Debug)]
pub struct State {
    icrc1_ledger_id: Principal,
    initialized: bool,
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State {
        icrc1_ledger_id: Principal::anonymous(),
        initialized: false
    });
}

#[ic_cdk::update]
fn set_icrc1_ledger(ledger_id: Principal) {
    if ledger_id == Principal::anonymous() {
        panic!("Cannot set an anonymous principal");
    }

    if !is_controller(&caller()) {
        panic!("Only the controllers can set the ICRC-1 ledger.");
    }

    STATE.with(|state| {
        let mut s = state.borrow_mut();
        s.icrc1_ledger_id = ledger_id;
        s.initialized = true;
    });
}

#[ic_cdk::query]
fn get_icrc1_ledger() -> Principal {
    STATE.with(|state| state.borrow().icrc1_ledger_id)
}

#[ic_cdk::update]
async fn transfer(args: TransferArguments) -> Result<BlockIndex, TransferError> {
    if !STATE.with(|state| state.borrow().initialized) {
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
        STATE.with(|state| state.borrow().icrc1_ledger_id),
        "icrc1_transfer",
        (transfer_arg,),
    )
    .await
    .unwrap()
    .0
}

#[ic_cdk::update]
async fn transfer_from(args: TransferFromArguments) -> Result<BlockIndex, TransferFromError> {
    if !STATE.with(|state| state.borrow().initialized) {
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
        STATE.with(|state| state.borrow().icrc1_ledger_id),
        "icrc2_transfer_from",
        (transfer_arg,),
    )
    .await
    .unwrap()
    .0
}

ic_cdk::export_candid!();
