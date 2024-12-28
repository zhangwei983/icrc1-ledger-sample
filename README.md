# Sample for ICRC-1 Ledger

Demonstrate how to deploy and interact with ICRC-1 ledger.

You can find the ledger Wasm and candid files at [ledger-suite-icrc-2024-11-28](https://github.com/dfinity/ic/releases/tag/ledger-suite-icrc-2024-11-28).

## How to interact

### Deploy the canisters

1. dfx new identity minter
1. dfx identity get-principal --identity minter
1. Replace the `tdrdy-ztedg-ftfrj-mwmqh-wjl3j-pty4c-j63lp-xfvtt-7jxvp-4ialz-3ae` principal with your minter principal in the [dfx.json](dfx.json) file.

1. dfx start --clean --background
1. dfx deploy

1. dfx canister call icrc1-ledger-sample set_icrc1_ledger "(principal \"$(dfx canister id icrc1-ledger)\")"
1. dfx canister call icrc1-ledger-sample get_icrc1_ledger

### Mint tokens

1. dfx canister call icrc1-ledger icrc1_transfer "(record {to=record {owner=principal \"$(dfx identity get-principal)\"; subaccount=null}; fee=null; memo=null; from_subaccount=null; created_at_time=null; amount=10000})" --identity minter
1. dfx canister call icrc1-ledger icrc1_balance_of "(record {owner=principal \"$(dfx identity get-principal)\"; subaccount=null})"

1. dfx canister call icrc1-ledger icrc1_transfer "(record {to=record {owner=principal \"$(dfx canister id icrc1-ledger-sample)\"; subaccount=null}; fee=null; memo=null; from_subaccount=null; created_at_time=null; amount=10000})" --identity minter
1. dfx canister call icrc1-ledger icrc1_balance_of "(record {owner=principal \"$(dfx canister id icrc1-ledger-sample)\"; subaccount=null})"

### Transfer

1. dfx canister call icrc1-ledger-sample transfer "(record {to=record {owner=principal \"$(dfx identity get-principal)\"; subaccount=null}; amount=100})"
1. dfx canister call icrc1-ledger icrc1_balance_of "(record {owner=principal \"$(dfx identity get-principal)\"; subaccount=null})"

### Transfer from

1. dfx canister call icrc1-ledger icrc2_approve "(record {amount=100; spender=record {owner=principal \"$(dfx canister id icrc1-ledger-sample)\"}})"
  The icrc_approve call takes fee.
1. dfx canister call icrc1-ledger-sample transfer_from "(record {to=record {owner=principal \"$(dfx canister id icrc1-ledger-sample)\"; subaccount=null}; from=record {owner=principal \"$(dfx identity get-principal)\"; subaccount=null}; amount=99})"
  The transfer_from call takes fee.

1. dfx canister call icrc1-ledger icrc1_balance_of "(record {owner=principal \"$(dfx canister id icrc1-ledger-sample)\"; subaccount=null})"
1. dfx canister call icrc1-ledger icrc1_balance_of "(record {owner=principal \"$(dfx identity get-principal)\"; subaccount=null})"
