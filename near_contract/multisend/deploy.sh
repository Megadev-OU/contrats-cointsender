#!/bin/bash
./build.sh && ./test.sh

wait

echo 'y' | near delete mulitsend.$1 $1

wait

near create-account mulitsend.$1 --masterAccount $1 --initialBalance 100

near deploy mulitsend.$1 \
  --wasmFile ./target/wasm32-unknown-unknown/release/multisend.wasm \
  --initFunction 'initialize' \
  --initArgs '{
          "percentage": 10,
          "bank": "some_bank_account.testnet",
          "admin": "'$1'"
        }'

near view mulitsend.$1 get_bank_address '{}'
near view mulitsend.$1 get_percentage '{}'

printf "bank account BEFORE: "

near state some_bank_account.testnet |  sed -n "s/.*formattedAmount: '\([^\\]*\).*'/\1/p"

printf "account we are sending to BEFORE: "

near state otheraccount.testnet |  sed -n "s/.*formattedAmount: '\([^\\]*\).*'/\1/p"

printf "account that are sending BEFORE: "

near state $1 |  sed -n "s/.*formattedAmount: '\([^\\]*\).*'/\1/p"


near call mulitsend.$1 multi_send_from_attached_deposit_near '{"recipients": ["otheraccount.testnet"], "amounts": ["1000000000000000000000000"]}' --depositYocto 1200000000000000000000000 --accountId $1

printf "bank account AFTER: "

near state some_bank_account.testnet |  sed -n "s/.*formattedAmount: '\([^\\]*\).*'/\1/p"

printf "account we are sending to AFTER: "

near state otheraccount.testnet |  sed -n "s/.*formattedAmount: '\([^\\]*\).*'/\1/p"

printf "account that are sending AFTER: "

near state $1 |  sed -n "s/.*formattedAmount: '\([^\\]*\).*'/\1/p"
