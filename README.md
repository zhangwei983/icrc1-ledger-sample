# Sample for ICRC-1 Ledger

Demonstrate how to deploy and interact with ICRC-1 ledger.

You can find the ledger Wasm and candid files under [icrc1-ledger](./src/icrc1-ledger) at [ledger-suite-icrc-2024-11-28](https://github.com/dfinity/ic/releases/tag/ledger-suite-icrc-2024-11-28).

## How to interact

### Deploy the canisters

1. Create an identity as the minter
   ```bash
   dfx new identity minter
   dfx identity get-principal --identity minter
   ```

1. Replace the `tdrdy-ztedg-ftfrj-mwmqh-wjl3j-pty4c-j63lp-xfvtt-7jxvp-4ialz-3ae` principal with your minter principal in the [dfx.json](dfx.json) file.
   And you can modify the `icrc1-ledger` init arguments as well.

1. Deploy the canisters
   ```bash
   dfx start --clean --background
   dfx deploy
   ```

1. Set the ICRC-1 ledger
   ```bash
   dfx canister call icrc1-ledger-sample set_icrc1_ledger "(principal \"$(dfx canister id icrc1-ledger)\")"
   dfx canister call icrc1-ledger-sample get_icrc1_ledger
   ```

### Mint tokens

1. Mint tokens to an your default identity.
   ```bash
   dfx canister call icrc1-ledger icrc1_transfer "(record {to=record {owner=principal \"$(dfx identity get-principal)\"; subaccount=null}; fee=null; memo=null; from_subaccount=null; created_at_time=null; amount=10000})" --identity minter
   dfx canister call icrc1-ledger icrc1_balance_of "(record {owner=principal \"$(dfx identity get-principal)\"; subaccount=null})"
   ```

1. Mint tokens to the `icrc1-ledger-sample` canister.
   ```
   dfx canister call icrc1-ledger icrc1_transfer "(record {to=record {owner=principal \"$(dfx canister id icrc1-ledger-sample)\"; subaccount=null}; fee=null; memo=null; from_subaccount=null; created_at_time=null; amount=10000})" --identity minter
   dfx canister call icrc1-ledger icrc1_balance_of "(record {owner=principal \"$(dfx canister id icrc1-ledger-sample)\"; subaccount=null})"
   ```

### Transfer

1. Tranfer the tokens from the `icrc1-ledger-sample` canister to your default identity.
   ```bash
   dfx canister call icrc1-ledger-sample transfer "(record {to=record {owner=principal \"$(dfx identity get-principal)\"; subaccount=null}; amount=100})"
   ```

1. Verify the transfer
   ```bash
   dfx canister call icrc1-ledger icrc1_balance_of "(record {owner=principal \"$(dfx canister id icrc1-ledger-sample)\"; subaccount=null})"
   dfx canister call icrc1-ledger icrc1_balance_of "(record {owner=principal \"$(dfx identity get-principal)\"; subaccount=null})"
   ```

### Transfer from

1. Call the `icrc2_approve` to approve the `icrc1-ledger-sample` canister as the spender which can transfer on behalf of your default identity.
   ```bash
   dfx canister call icrc1-ledger icrc2_approve "(record {amount=100; spender=record {owner=principal \"$(dfx canister id icrc1-ledger-sample)\"}})"
   ```
   Note: The `icrc_approve` call on the ICRC-1 ledger takes transfer fee.

1. Call the `transfer_from` of the `icrc1-ledger-sample` canister to transfer on behalf of your default identity.

   ```bash
   dfx canister call icrc1-ledger-sample transfer_from "(record {to=record {owner=principal \"$(dfx canister id icrc1-ledger-sample)\"; subaccount=null}; from=record {owner=principal \"$(dfx identity get-principal)\"; subaccount=null}; amount=99})"
   ```
  Note: The `transfer_from` call on the ICRC-1 ledger takes transfer fee.

1. Verify the transfer
   ```bash
   dfx canister call icrc1-ledger icrc1_balance_of "(record {owner=principal \"$(dfx canister id icrc1-ledger-sample)\"; subaccount=null})"
   dfx canister call icrc1-ledger icrc1_balance_of "(record {owner=principal \"$(dfx identity get-principal)\"; subaccount=null})"
   ```
