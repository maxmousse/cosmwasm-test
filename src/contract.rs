use cosmwasm_std::{coins, BankMsg, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

use crate::{
    donation::Donation,
    error::ContractError,
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
    project::Project,
    state::{DONATION_DENOM, FEE_COLLECTOR_ADDR, PROJECTS},
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
    // Note: Projects are identified by creator_addr and duplicate are not handled
    let projects: StdResult<Vec<Project>> = msg
        .projects
        .into_iter()
        .map(|(name, creator_addr)| {
            let creator_addr = deps.api.addr_validate(&creator_addr)?;
            Ok(Project::new(name, creator_addr))
        })
        .collect();

    // Init the contract state
    DONATION_DENOM.save(deps.storage, &msg.donation_denom)?;
    FEE_COLLECTOR_ADDR.save(deps.storage, &fee_collector_addr)?;
    PROJECTS.save(deps.storage, &projects?)?;

    Ok(Response::new())
}

pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!();
}

pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Donate {
            project_creator_addr,
        } => donate(deps, info, &project_creator_addr),
    }
}

fn donate(
    deps: DepsMut,
    info: MessageInfo,
    project_creator_addr: &String,
) -> Result<Response, ContractError> {
    // Validate the project_creator_addr
    let project_creator_addr = deps.api.addr_validate(&project_creator_addr)?;

    // Load the contract state
    let fee_collector_addr = FEE_COLLECTOR_ADDR.load(deps.storage)?;
    let denom = DONATION_DENOM.load(deps.storage)?;
    let mut projects = PROJECTS.load(deps.storage)?;

    // Get project index or return an error
    let project_index = projects
        .iter()
        .position(|project| project.creator_addr == &project_creator_addr)
        .ok_or(ContractError::ProjectNotFound {
            project: project_creator_addr.clone(),
        })?;

    // Get the donation
    let donator_addr = info.sender.clone();
    let donation = cw_utils::must_pay(&info, &denom)?.u128();

    // Calculate donation/fees
    let fee = if donation < 10000 {
        donation as f64 * 0.1
    } else {
        donation as f64 * 0.05
    } as u128;
    let donation = donation - fee;

    // Update project in contract state
    projects[project_index]
        .donations
        .push(Donation::new(donator_addr, donation));
    PROJECTS.save(deps.storage, &projects)?;

    // Transfer the donation to the project creator
    let donation_msg = BankMsg::Send {
        to_address: project_creator_addr.to_string(),
        amount: coins(donation, denom.clone()),
    };

    // Transfer the fees to the fee collector
    let fee_msg = BankMsg::Send {
        to_address: fee_collector_addr.to_string(),
        amount: coins(fee, denom),
    };

    let resp = Response::new()
        .add_message(donation_msg)
        .add_message(fee_msg);

    Ok(resp)
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

        let owner_addr = app.api().addr_make("owner");

        // Instantiate the contract
        let addr = app
            .instantiate_contract(
                code_id,
                owner_addr,
                &InstantiateMsg {
                    donation_denom: "uusd".to_owned(),
                    fee_collector_addr: "fee_collector_addr".to_owned(),
                    projects: vec![],
                    //     ("project1".to_owned(), "creator_addr_1".to_owned()),
                    //     ("project2".to_owned(), "creator_addr_2".to_owned()),
                    // ],
                },
                &[],
                "Contract",
                None,
            )
            .unwrap();

        // Check contract state
    }
}
