JUNCTION="./artifacts/junctiond"
CHAIN_ID="junction"
KEY_NAME="okok"
KEYRING_BACKEND="--keyring-backend test"
WASM="artifacts/cosm_board.wasm"
CONTRACT_ADDRESS="air17p9rzwnnfxcjp32un9ug7yhhzgtkhvl9jfksztgw5uh69wac2pgs8g70ff"

$JUNCTION tx wasm store $WASM \
  --from $KEY_NAME \
  --chain-id $CHAIN_ID \
  $KEYRING_BACKEND \
  --gas auto \
  --gas-adjustment 1.3 \
  --gas-prices 0.025amf \
  -y

sleep 4

$JUNCTION tx wasm instantiate 6 '{}' \
  --from $KEY_NAME \
  --label "Counter Contract" \
  --chain-id $CHAIN_ID \
  $KEYRING_BACKEND \
  --no-admin \
  --gas auto \
  --gas-adjustment 1.3 \
  --gas-prices 0.025amf \
  -y

#sleep 5s
#
#$JUNCTION tx wasm execute $CONTRACT_ADDRESS '{"PostMessage": {"content":"okok"}}' \
#  --from $KEY_NAME \
#  --chain-id $CHAIN_ID \
#  $KEYRING_BACKEND \
#  --gas auto \
#  --gas-adjustment 1.3 \
#  --gas-prices 0.025amf \
#  -y
#
#sleep 5s
#
#$JUNCTION query wasm contract-state smart $CONTRACT_ADDRESS '{"GetMessage": {"id" : 2}}' \
#  --chain-id $CHAIN_ID \
#  --node tcp://0.0.0.0:26657

#$JUNCTION query tx 2EEDAFBBA19DCA4D0FBEB2BFD6DB7A67295EED83220425180ABD99269A00691F --chain-id $CHAIN_ID
