use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

use crate::{
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
    project::Project,
    state::{FEE_COLLECTOR_ADDR, PROJECTS},
};

pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    // Validate the fee_collector_addr
    let fee_collector_addr = deps.api.addr_validate(&msg.fee_collector_addr)?;

    // Validate the projects addresses
    // Note: do not handle projects with the same name
    let projects: StdResult<Vec<Project>> = msg
        .projects
        .into_iter()
        .map(|(name, creator_addr)| {
            let creator_addr = deps.api.addr_validate(&creator_addr)?;
            Ok(Project::new(name, creator_addr))
        })
        .collect();

    // Save the fee_collector_addr and projects
    FEE_COLLECTOR_ADDR.save(deps.storage, &fee_collector_addr)?;
    PROJECTS.save(deps.storage, &projects?)?;

    Ok(Response::new())
}

pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!();
}

pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    unimplemented!();
}

#[cfg(test)]
mod test {
    use crate::msg::{InstantiateMsg, QueryMsg};
    use cosmwasm_std::Addr;
    use cw_multi_test::{App, ContractWrapper, Executor};

    use super::*;

    #[test]
    fn should_instantiate() {
        let mut app = App::default();

        let code = ContractWrapper::new(execute, instantiate, query);
        let code_id = app.store_code(Box::new(code));

        // Instantiate the contract
        let addr = app
            .instantiate_contract(
                code_id,
                Addr::unchecked("owner"),
                &InstantiateMsg {
                    fee_collector_addr: "fee_collector_addr".to_string(),
                    projects: vec![
                        ("project1".to_string(), "creator_addr_1".to_string()),
                        ("project2".to_string(), "creator_addr_2".to_string()),
                    ],
                },
                &[],
                "Contract",
                None,
            )
            .unwrap();

        // Check contract state
    }
}
