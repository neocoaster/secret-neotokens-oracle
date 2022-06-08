# COMPILE AND OPTIMIZE RUST CODE:
`cargo wasm`

`docker run --rm -v "$(pwd)":/contract --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry enigmampc/secret-contract-optimizer`

# START BLOCKCHAIN:

`docker run -it --rm -p 26657:26657 -p 26656:26656 -p 1337:1337 -p 9091:9091 -v $(pwd):/root/code --name secretdev enigmampc/secret-network-sw-dev`

# CONNECT TO NODE:

`docker exec -it secretdev /bin/bash`

# INSIDE NODE:

`cd code`

`secretd tx compute store contract.wasm.gz --from a --gas 1000000 -y --keyring-backend test`

`INIT="{}"`

`CODE_ID=1`

`secretd tx compute instantiate $CODE_ID "$INIT" --from a --label "neotokens-oracle" -y --keyring-backend test`

`secretd query compute list-contract-by-code 1`

# RUN TRANSACTIONS

`secretd tx compute execute $CONTRACT '{"add": {"credits":5, "address":"secret1dyndv273y0d0w2tq4gqtx5mjjhuzl5cf6hfqe6"}}' --from a --keyring-backend test`

`secretd tx compute execute $CONTRACT '{"get_credits": {}}' --from a --keyring-backend test`
