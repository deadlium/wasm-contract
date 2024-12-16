use cosmwasm_std::{entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, StdError};
use cw_storage_plus::{Item, Map};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use serde_json_wasm::{to_vec};

pub const BOARD_COUNT: Item<u32> = Item::new("board_count"); // Tracks the number of messages

// Storage for querying by contract address
pub const BOARD_BY_CONTRACT: Map<String, BoardDetail> = Map::new("board_by_contract");

// Storage for querying by wallet address
pub const BOARDS_BY_CREATOR: Map<String, Vec<String>> = Map::new("boards_by_creator");

// Struct for BoardDetail
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BoardDetail {
    pub board_id: u32,
    pub creator_address: String,
    pub contract_address: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum ExecuteMsg {
    CreateBoard {
        creator_contract_address: String,
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum QueryMsg {
    GetBoardByContractAddress {
        contract_address: String
    },
    GetBoardByCreatorAddress {
        creator_address: String
    }
}

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    BOARD_COUNT.save(deps.storage, &0)?;
    Ok(Response::new().add_attribute("method", "instantiate"))
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::CreateBoard { creator_contract_address } => create_board(deps, _env, _info, creator_contract_address),
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetBoardByContractAddress { contract_address } => {
            query_board_by_contract(deps, contract_address)
        },
        QueryMsg::GetBoardByCreatorAddress {creator_address} => {
            query_boards_by_creator(deps, creator_address)
        },
    }
}

pub fn create_board(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    contract_address: String,
) -> StdResult<Response> {
    if contract_address.is_empty() {
        return Err(StdError::generic_err("Contract address cannot be empty"));
    }

    let mut count = BOARD_COUNT.load(deps.storage)?;
    count += 1;
    BOARD_COUNT.save(deps.storage, &count)?;

    // Create the board detail
    let board_detail = BoardDetail {
        board_id: count,
        creator_address: info.sender.to_string(),
        contract_address: contract_address.clone(),
    };

    // Save to `BOARD_BY_CONTRACT`
    BOARD_BY_CONTRACT.save(deps.storage, contract_address.clone(), &board_detail)?;

    // Update `BOARDS_BY_CREATOR`
    let creator_address = info.sender.to_string();
    let mut boards = BOARDS_BY_CREATOR
        .may_load(deps.storage, creator_address.clone())?
        .unwrap_or_default();

    boards.push(contract_address.clone());
    BOARDS_BY_CREATOR.save(deps.storage, creator_address.clone(), &boards)?;

    Ok(Response::new()
        .add_attribute("action", "add_board")
        .add_attribute("board_id", count.to_string())
        .add_attribute("creator_address", info.sender)
        .add_attribute("contract_address", contract_address))
}

pub fn query_board_by_contract(
    deps: Deps,
    contract_address: String,
) -> StdResult<Binary> {
    let board = BOARD_BY_CONTRACT
        .may_load(deps.storage, contract_address)?
        .ok_or_else(|| StdError::generic_err("No board found for the provided contract address"))?;

    let serialized = to_vec(&board).map_err(|e| StdError::serialize_err("BoardDetail", e))?;
    Ok(Binary::from(serialized))
}

pub fn query_boards_by_creator(
    deps: Deps,
    creator_address: String,
) -> StdResult<Binary> {
    let contract_addresses = BOARDS_BY_CREATOR
        .may_load(deps.storage, creator_address.clone())?
        .ok_or_else(|| StdError::generic_err("No boards found for the provided creator address"))?;

    // Map contract addresses to their corresponding board details
    let boards: Vec<BoardDetail> = contract_addresses
        .iter()
        .filter_map(|contract_address| {
            BOARD_BY_CONTRACT.may_load(deps.storage, contract_address.clone()).ok().flatten()
        })
        .collect();

    if boards.is_empty() {
        return Err(StdError::generic_err("No boards found for the provided creator address"));
    }

    let serialized = to_vec(&boards).map_err(|e| StdError::serialize_err("BoardDetail", e))?;
    Ok(Binary::from(serialized))
}


