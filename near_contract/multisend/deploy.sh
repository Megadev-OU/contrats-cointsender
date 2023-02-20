#!/bin/bash
./build.sh && ./test.sh

wait

echo 'y' | near delete mulitsend.$1 $1

wait

near create-account mulitsend.$1 --masterAccount $1 --initialBalance 3

near deploy mulitsend.$1 \
  --wasmFile ./target/wasm32-unknown-unknown/release/multisend.wasm \
  --initFunction 'initialize' \
  --initArgs '{
          "percentage": 10,
          "bank": "some_bank_account.near",
          "admin": "'$1'"
        }'

near view mulitsend.$1 get_bank_address '{}'
near view mulitsend.$1 get_percentage '{}'