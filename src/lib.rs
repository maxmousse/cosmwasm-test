use cosmwasm_std::{entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use error::ContractError;
use msg::{ExecuteMsg, InstantiateMsg};

mod contract;
mod donation;
mod error;
pub mod msg;
mod project;
mod state;

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    contract::instantiate(deps, env, info, msg)
}

#[entry_point]
pub fn query(deps: Deps, env: Env, msg: msg::QueryMsg) -> Result<Binary, ContractError> {
    contract::query(deps, env, msg)
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    contract::execute(deps, _env, info, msg)
}
