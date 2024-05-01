use cosmwasm_test::{execute, instantiate, msg::InstantiateMsg, query};
use cw_multi_test::{App, ContractWrapper, Executor};

#[test]
fn contract_should_work() {
    let mut app = App::default();

    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));

    let owner_addr = app.api().addr_make("owner");

    // Instantiate the contract
    // NOTE: for some reasons, contract instantiation fails and I did not
    // found a solution to fix it
    let addr = app
        .instantiate_contract(
            code_id,
            owner_addr,
            &InstantiateMsg {
                donation_denom: "uusd".to_owned(),
                fee_collector_addr: "fee_collector_addr".to_owned(),
                projects: vec![
                    ("project1".to_owned(), "creator_addr_1".to_owned()),
                    ("project2".to_owned(), "creator_addr_2".to_owned()),
                ],
            },
            &[],
            "Contract",
            None,
        )
        .unwrap();

    // After instantiation, the contract state should be as follows:
    // project 1 should have no donations
    // project 2 should have no donations
    // querying user donations should return 0 donation
    // user should have its starting balance
    // fee collector should have no balance

    // User donates to non-existent project
    // Should fail with error
    // project 1 should have no donations
    // project 2 should have no donations
    // querying user donations should return 0 donation
    // user should have its starting balance
    // fee collector should have no balance

    // User donates to project 1
    // After first donation, the contract state should be as follows:
    // project 1 should have 1 donation
    // project 2 should have no donations
    // user should have its balance minus the donation amount
    // fee collector should have collected the fee

    // User donates to project 2
    // After second donation, the contract state should be as follows:
    // project 1 should have 1 donation
    // project 2 should have 1 donation
    // user should have its balance minus the donation amount
    // fee collector should have collected the fee
}
