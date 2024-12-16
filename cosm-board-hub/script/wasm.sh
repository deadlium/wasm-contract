JUNCTION="./artifacts/junctiond"
CHAIN_ID="junction"
KEY_NAME="okok"
KEYRING_BACKEND="--keyring-backend test"
WASM="artifacts/cosm_board_hub.wasm"
CONTRACT_ADDRESS="air1ghd753shjuwexxywmgs4xz7x2q732vcnkm6h2pyv9s6ah3hylvrqm7nkfg"

$JUNCTION tx wasm store $WASM \
  --from $KEY_NAME \
  --chain-id $CHAIN_ID \
  $KEYRING_BACKEND \
  --gas auto \
  --gas-adjustment 1.3 \
  --gas-prices 0.025amf \
  -y

$JUNCTION tx wasm instantiate 4 '{}' \
  --from $KEY_NAME \
  --label "COSM BOARD HUB CONTRACT" \
  --chain-id $CHAIN_ID \
  $KEYRING_BACKEND \
  --no-admin \
  --gas auto \
  --gas-adjustment 1.3 \
  --gas-prices 0.025amf \
  -y

$JUNCTION tx wasm execute $CONTRACT_ADDRESS '{"CreateBoard": {"creator_contract_address":"air1pvrwmjuusn9wh34j7y520g8gumuy9xtl3gvprlljfdpwju3x7ucs40xrky"}}' \
  --from $KEY_NAME \
  --chain-id $CHAIN_ID \
  $KEYRING_BACKEND \
  --gas auto \
  --gas-adjustment 1.3 \
  --gas-prices 0.025amf \
  -y

$JUNCTION query wasm contract-state smart $CONTRACT_ADDRESS '{"GetBoardByCreatorAddress": {"creator_address" : "air1t9gzrdayfrka06e3h2k4kq4kl6wgc8mrkege7f"}}' \
  --chain-id $CHAIN_ID \
  --node tcp://0.0.0.0:26657

$JUNCTION query wasm contract-state smart $CONTRACT_ADDRESS '{"GetBoardByContractAddress": {"contract_address": "air1eyfccmjm6732k7wp4p6gdjwhxjwsvje44j0hfx8nkgrm8fs7vqfsqfvswe"}}' \
  --chain-id $CHAIN_ID \
  --node tcp://0.0.0.0:26657
