# Cosmwasm Test

This repository contains a simple Cosmwasm smart contract that implements a donation platform.

## Features

- Contract is instantiated with a list of projects and a fee collector wallet address
- Contract allows users to donate CW20 tokens to a project.
- A project has a name and the wallet address of its creator
- A project keeps track of the donations received from each user
- Donations history can be queried by user wallet address
- Donations are sent to the project creator's wallet address (minus fees)
- Contract deducts 10% fee if the donation is less than 10,000 CW20 tokens, 5% otherwise. Fees are sent to the fee collector wallet

## DD notes

During instantiation (and almost during any operation in contract) addresses are validated, like this:

```text
let fee_collector_addr = deps.api.addr_validate( & msg.fee_collector_addr) ?;
```

It means, that the human-readable format provided as a string is checked, and (if correct) then converted into `Addr`.

**This is correct.**

In tests, we need to provide addresses that will not fail the validation process inside contract.
The only function, that guarantees that the address will be valid in tests is `addr_make`, used like this:

```text
let owner_addr = app.api().addr_make("owner");
```

Because the most of the chains use Bech32 for the human-readable address format,
we have provided this format also in **MultiTest**.

There is another interesting option, where you can create an address in tests like this,
you can even select your favourite prefix:

```text
assert_eq!(
    "addr1".into_bech32().as_str(),
    "cosmwasm14ch5q26mhx3jk5cxl88t278nper264ce5fa7agjr4cw0yfjj7c6q56drym",
);
assert_eq!(
    "addr1".into_bech32_with_prefix("juno").as_str(),
    "juno14ch5q26mhx3jk5cxl88t278nper264ce5fa7agjr4cw0yfjj7c6q96gycm",
);
```

If you would like to change a prefix used for your chain in tests, you can do it like this:

```text
let mut app = AppBuilder::default()
    .with_api(MockApiBech32::new("donator"))
    .build(no_init);
```

Default prefix for chain tested this way:

```text
let mut app = App::default();
```

is **`cosmwasm`**. You can easily verify it this way:

```text
let mut app = App::default ();
let owner_addr = "owner".into_bech32_with_prefix("donator");
assert_eq!("donator1fsgzj6t7udv8zhf6zj32mkqhcjcpv52yph5qsdcl0qt94jgdckqsx5vyr3", owner_addr.as_str());
```
