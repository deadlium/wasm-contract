use cosmwasm_std::{
    entry_point, Binary, Deps, DepsMut, Env, Event, MessageInfo, Response, StdError, StdResult,
};
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// Define state storage
pub const MESSAGE_COUNT: Item<u32> = Item::new("message_count"); // Tracks the number of messages
pub const MESSAGES: Map<u32, String> = Map::new("messages"); // Maps message ID to content

// Instantiate message
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

// Execute messages
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum ExecuteMsg {
    PostMessage { content: String }, // Add a new message
}

// Query messages
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum QueryMsg {
    GetMessage { id: u32 }, // Get a specific message by ID
}

// Query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MessageResponse {
    pub id: u32,
    pub content: String,
}

// Instantiate: Initializes the contract state
#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    // Initialize message count to 0
    MESSAGE_COUNT.save(deps.storage, &0)?;
    Ok(Response::new().add_attribute("method", "instantiate"))
}

// Execute: Handles actions like posting a message
#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::PostMessage { content } => post_message(deps, info, content),
    }
}

// Function to post a message
fn post_message(deps: DepsMut, info: MessageInfo, content: String) -> StdResult<Response> {
    if content.is_empty() {
        return Err(StdError::generic_err("Content cannot be empty"));
    }

    // Increment the message count
    let mut count = MESSAGE_COUNT.load(deps.storage)?;
    count += 1;
    MESSAGE_COUNT.save(deps.storage, &count)?;

    // Save the message
    MESSAGES.save(deps.storage, count, &content)?;

    // Emit an event
    let event = Event::new("post_message")
        .add_attribute("sender", info.sender.to_string())
        .add_attribute("message_id", count.to_string())
        .add_attribute("content", content.clone());

    Ok(Response::new()
        .add_attribute("method", "post_message")
        .add_attribute("message_id", count.to_string())
        .add_event(event))
}

// Query: Handles fetching messages
#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetMessage { id } => get_message(deps, id),
    }
}

// Function to get a message by ID
fn get_message(deps: Deps, id: u32) -> StdResult<Binary> {
    let content = MESSAGES
        .may_load(deps.storage, id)?
        .ok_or_else(|| StdError::not_found("Message"))?;
    let response = MessageResponse { id, content };
    cosmwasm_std::to_binary(&response)
}
