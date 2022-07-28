// This is where you define what your contract does

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cosmwasm_std::SystemResult::Ok;
use cw2::set_contract_version;
use schemars::_serde_json::Result;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Config, CONFIG, Poll};


const CONTRACT_NAME: &str = "crates.io:zero-to-hero";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;


//This will error, if the user gives an invalid address
    let validated_admin_address = deps.api.addr_validate(&msg.admin_address)?;

    let config = Config {
        pub_admin_address: validated_admin_address // Set to the validated address
       };

       CONFIG.save(deps.storage, &config)?;

    // Result<Respomse>
    Ok(Response::new().add_attribute("action", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreatePoll { question } => execute_create_poll(deps, env, info, question)
   }
}

use crate::state::POLLS;
fn execute_create_poll(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    question: String,
) -> Result<Response, ContractError> {
    // Does the map have a key of this value
    if POLLS.has(deps.storage, question.clone()) {
        // If it does, we want to error!
        return Err(ContractError::CustomError {
            val: "Key already taken!".to_string(),
        });
    }

    let poll = Poll {
        question: question.clone(),
        yes_votes: 0,
        no_votes: 0,
    };

    POLLS.save(deps.storage, question, &poll)?;

    Ok(Response::new().add_attribute("action", "create_poll"))
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{attr, Response};
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use crate::contract::{instantiate, execute};
    use crate::msg::{InstantiateMsg, ExecuteMsg};

    #[test]
    fn test_itantiate() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info( "addr1", &[]
        );
        let msg = InstantiateMsg {
            admin_address: "addr1".to_string()
        };
        let resp = instantiate(deps.as_mut(), env, info, msg).unwrap;
        assert_eq!(resp.attributes, vec![
            attr("action", "instantiate")
        ]);
    }
    
    #[test]
    fn test_create_poll() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info( "addr1", &[]
        );
        let msg = InstantiateMsg {
            admin_address: "addr1".to_string()
        };

        // Before you execute a contract you need to instantiate it
        let _resp = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap;

        let msg = ExecuteMsg::CreatePoll { 
            question: "Do you love Spark IBC?".to_string() 
    };
   
    let resp = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    assert_eq!(resp.attributes, vec![
        attr("action", "create_poll")
    ]);


    }
}