JUNCTION="./artifacts/junctiond"
CHAIN_ID="junction"
KEY_NAME="okok"
KEYRING_BACKEND="--keyring-backend test"
WASM="artifacts/number_counter.wasm"
CONTRACT_ADDRESS="air14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9snx9gzt"

$JUNCTION tx wasm store $WASM \
  --from $KEY_NAME \
  --chain-id $CHAIN_ID \
  $KEYRING_BACKEND \
  --gas auto \
  --gas-adjustment 1.3 \
  --gas-prices 0.025amf \
  -y

sleep 5s

$JUNCTION tx wasm instantiate 1 '{}' \
  --from $KEY_NAME \
  --label "Counter Contract" \
  --chain-id $CHAIN_ID \
  $KEYRING_BACKEND \
  --no-admin \
  --gas auto \
  --gas-adjustment 1.3 \
  --gas-prices 0.025amf \
  -y

sleep 5s

$JUNCTION tx wasm execute $CONTRACT_ADDRESS '{"Increment": {}}' \
  --from $KEY_NAME \
  --chain-id $CHAIN_ID \
  $KEYRING_BACKEND \
  --gas auto \
  --gas-adjustment 1.3 \
  --gas-prices 0.025amf \
  -y

sleep 5s

$JUNCTION query wasm contract-state smart $CONTRACT_ADDRESS '{"GetCount": {}}' \
  --chain-id $CHAIN_ID \
  --node tcp://0.0.0.0:26657

