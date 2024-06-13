use cosmwasm_test::{execute, instantiate, msg::InstantiateMsg, query};
use cw_multi_test::{
    no_init, App, AppBuilder, ContractWrapper, Executor, IntoBech32, MockApiBech32,
};

#[test]
fn contract_should_work() {
    let mut app = App::default();

    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));

    let owner_addr = app.api().addr_make("owner");

    // Instantiate the contract
    // ORIGINAL NOTE: for some reason, contract instantiation fails and I haven't found a solution to fix it

    // Please see: "DD notes" in README.md

    let _ = app
        .instantiate_contract(
            code_id,
            owner_addr,
            &InstantiateMsg {
                donation_denom: "uusd".to_owned(),
                fee_collector_addr: app.api().addr_make("fee_collector_addr").to_string(),
                projects: vec![
                    (
                        "project1".to_string(),
                        app.api().addr_make("creator_addr_1").to_string(),
                    ),
                    (
                        "project2".to_string(),
                        app.api().addr_make("creator_addr_2").to_string(),
                    ),
                ],
            },
            &[],
            "Contract",
            None,
        )
        .unwrap();

    // After instantiation, the contract state should be as follows:
    // project 1 should have no donations
    // project 2 should have no donations
    // querying user donations should return 0 donation
    // user should have its starting balance
    // fee collector should have no balance

    // User donates to non-existent project
    // Should fail with error
    // project 1 should have no donations
    // project 2 should have no donations
    // querying user donations should return 0 donation
    // user should have its starting balance
    // fee collector should have no balance

    // User donates to project 1
    // After first donation, the contract state should be as follows:
    // project 1 should have 1 donation
    // project 2 should have no donations
    // user should have its balance minus the donation amount
    // fee collector should have collected the fee

    // User donates to project 2
    // After second donation, the contract state should be as follows:
    // project 1 should have 1 donation
    // project 2 should have 1 donation
    // user should have its balance minus the donation amount
    // fee collector should have collected the fee
}

//TODO: this is an example of using custom address prefix, just remove te code below
#[test]
fn contract_should_work_custom_address_prefix() {
    let mut app = AppBuilder::default()
        .with_api(MockApiBech32::new("donator"))
        .build(no_init);

    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));

    // it's always better to use `addr_make` function, but it is also possible
    // create an address this way (when you know the custom prefix used in your chain):
    let owner_addr = "owner".into_bech32_with_prefix("donator");
    assert_eq!(
        "donator1fsgzj6t7udv8zhf6zj32mkqhcjcpv52yph5qsdcl0qt94jgdckqsx5vyr3",
        owner_addr.as_str()
    );

    let _ = app
        .instantiate_contract(
            code_id,
            owner_addr,
            &InstantiateMsg {
                donation_denom: "uusd".to_owned(),
                fee_collector_addr: app.api().addr_make("fee_collector_addr").to_string(),
                projects: vec![
                    (
                        "project1".to_string(),
                        app.api().addr_make("creator_addr_1").to_string(),
                    ),
                    (
                        "project2".to_string(),
                        app.api().addr_make("creator_addr_2").to_string(),
                    ),
                ],
            },
            &[],
            "Contract",
            None,
        )
        .unwrap();
}
