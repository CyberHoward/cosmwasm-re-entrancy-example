# CosmWasm re-entrancy Exploit Example

> The exploit example is located in `contracts/liquidity_hub/vault-network/exploit_contract`.

A vault in `contracts/liquidity_hub/vault-network/vault` allows users to get a flashloan for a fee. 

The user provide a callback for the flashloan which is sandwiched between the vault's own messages. The execution order is: 

- Call vault for flashloan
- Vault sends tokens, executes the callback and **inserts its own callback**.
- The calling contract executes whatever he wishes with the received funds
- the vault callback executes.

The vault contract can ensure that all lent-out funds are returned by the lender by extending the `Response` with a callback [`vault::CallbackMsg::AfterTrade`]. This callback is ensured to be the last call in the execution order. 

The edge-case that was overlooked was the case in which the lender performs a deposit with his lent funds. This edge-case is shown to be exploitable in `tests/exploit.rs`. 

## Run
cd into `contracts/liquidity_hub/vault-network/exploit_contract` and run `cargo test`. The exploit test can be viewed in `tests/exploit.rs`.