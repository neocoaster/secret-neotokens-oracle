# COMPILE AND OPTIMIZE RUST CODE:
`cargo wasm`

`docker run --rm -v "$(pwd)":/contract --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry enigmampc/secret-contract-optimizer`

# START BLOCKCHAIN:

`docker run -d -it -p 9091:9091 -p 26657:26657 -p 13170:1317 -p 5000:5000 -v $(pwd):/root/code --name localsecret ghcr.io/scrtlabs/localsecret`

# CONNECT TO NODE:

`docker exec -it localsecret /bin/bash`

# INSIDE NODE:

`cd code`

`secretd tx compute store contract.wasm.gz --from a --gas 1000000 -y --keyring-backend test`

`INIT="{}"`

`CODE_ID=1`

`secretd tx compute instantiate $CODE_ID "$INIT" --from a --label "neotokens-oracle" -y --keyring-backend test`

`secretd query compute list-contract-by-code 1`

# TO GET THE CONTRACT'S HASH (ignore 0x from output)

`secretd q compute contract-hash <address>`

# RUN TRANSACTIONS

`secretd tx compute execute $CONTRACT '{"add": {"credits":5, "address":"secret1fc3fzy78ttp0lwuujw7e52rhspxn8uj52zfyne"}}' --from a --keyring-backend test`

`secretd query compute query $CONTRACT '{"get_credits": { "address": "secret1fc3fzy78ttp0lwuujw7e52rhspxn8uj52zfyne" }}'`
