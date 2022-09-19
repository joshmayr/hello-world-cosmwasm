#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, GetCountResponse, InstantiateMsg, QueryMsg, GetMessageResponse};
use crate::state::{State, Message, STATE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:hello-world-cosmwasm";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        count: msg.count,
        owner: info.sender.clone(),
        messages: vec![],
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("count", msg.count.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::SendMessage { message } => try_send_message(deps, info, message, env),
    }
}

pub fn try_send_message(deps: DepsMut, info: MessageInfo, message: String, env: Env) -> Result<Response, ContractError> {
    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        state.count += 1;
        state.messages.push(Message{
            sender: info.sender,
            message: message,
            timestamp: env.block.time.seconds() * 1000,
        });
        Ok(state)
    })?;
    Ok(Response::new().add_attribute("method", "send_message"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetCount {} => to_binary(&query_count(deps)?),
        QueryMsg::GetMessage { index } => to_binary(&query_message(deps, index)?),
    }
}

fn query_count(deps: Deps) -> StdResult<GetCountResponse> {
    let state = STATE.load(deps.storage)?;
    Ok(GetCountResponse { count: state.count })
}

fn query_message(deps: Deps, index: u64) -> StdResult<GetMessageResponse> {
    let state = STATE.load(deps.storage)?;
    match state.messages.get(index as usize) {
        None => Ok(GetMessageResponse { message: String::from("Invalid Index") }),
        Some(message_to_return) => Ok(GetMessageResponse { message: message_to_return.message.to_string() }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg { count: 17 };
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: GetCountResponse = from_binary(&res).unwrap();
        assert_eq!(17, value.count);
    }

    #[test]
    fn send_message() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg { count: 0 };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // Calling GetMessage on an invalid index returns the string "Invalid Index"
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetMessage { index: 0 }).unwrap();
        let value: GetMessageResponse = from_binary(&res).unwrap();
        assert_eq!(String::from("Invalid Index"), value.message);

        // Execute SendMessage("Hello, World!")
        let info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::SendMessage { message: (String::from("Hello, World!")) };
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // should increase counter by 1
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: GetCountResponse = from_binary(&res).unwrap();
        assert_eq!(1, value.count);

        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetMessage { index: 0 }).unwrap();
        let value: GetMessageResponse = from_binary(&res).unwrap();
        assert_eq!(String::from("Hello, World!"), value.message);
    }
}
