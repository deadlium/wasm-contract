use cosmwasm_std::{
    entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, StdError,
};
use cw_storage_plus::Item;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use serde_json_wasm::to_vec;

// State storage for the counter
pub const COUNTER: Item<u32> = Item::new("counter");

// Instantiate message: Used for contract initialization
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

// Execute message: Defines actions the contract can perform
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum ExecuteMsg {
    Increment {}, // Increment the counter
}

// Query message: Used to fetch data without changing state
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum QueryMsg {
    GetCount {}, // Query the current counter value
}

// Response for QueryMsg::GetCount
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CountResponse {
    pub count: u32,
}

// Instantiate: Initializes the contract state
#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    // Initialize the counter to 0
    COUNTER.save(deps.storage, &0)?;
    Ok(Response::new().add_attribute("method", "instantiate"))
}

// Execute: Handles actions that modify the contract's state
#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::Increment {} => {
            let mut counter = COUNTER.load(deps.storage)?;
            counter += 1; // Increment the counter
            COUNTER.save(deps.storage, &counter)?; // Save the updated value

            Ok(Response::new()
                .add_attribute("method", "increment")
                .add_attribute("new_count", counter.to_string()))
        }
    }
}

// Query: Handles queries to read the contract's state
#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetCount {} => {
            let count = COUNTER.load(deps.storage)?;
            let response = CountResponse { count };

            // Serialize the response using serde_json_wasm
            let serialized = to_vec(&response).map_err(|e| StdError::serialize_err("CountResponse", e))?;
            Ok(Binary::from(serialized))
        }
    }
}
