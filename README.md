# Challenge | Rather Labs

# Note
Contract address: erd1qqqqqqqqqqqqqpgqd6sq3xt8tkxn2kk3s3yuultxeagcwvjwqx2qx4m7u4
- https://devnet-explorer.multiversx.com/accounts/erd1qqqqqqqqqqqqqpgqd6sq3xt8tkxn2kk3s3yuultxeagcwvjwqx2qx4m7u4

### Deploying the contract on devnet
```sh
mxpy --verbose contract deploy --bytecode=./output/staking-contract.wasm \
    --recall-nonce --pem=~/MyTestWallets/TestKey.pem \
    --gas-limit=10000000 \
    --send --outfile="deploy-devnet.interaction.json" --wait-result \
    --proxy=https://devnet-gateway.multiversx.com --chain=D
```

### Call the stake function
```sh
mxpy --verbose contract call erd1qqqqqqqqqqqqqpgqd6sq3xt8tkxn2kk3s3yuultxeagcwvjwqx2qx4m7u4 \
    --proxy=https://devnet-gateway.multiversx.com --chain=D \
    --send --recall-nonce --pem=~/MyTestWallets/TestKey.pem \
    --gas-limit=10000000 \
    --value=2000000000000000000 \
    --function="stake"
```

### Querying the view functions

```sh
mxpy --verbose contract query erd1qqqqqqqqqqqqqpgqd6sq3xt8tkxn2kk3s3yuultxeagcwvjwqx2qx4m7u4 \
    --proxy=https://devnet-gateway.multiversx.com \
    --function="getStakingPosition" \
    --arguments ${USER_ADDRESS}

mxpy --verbose contract query erd1qqqqqqqqqqqqqpgqd6sq3xt8tkxn2kk3s3yuultxeagcwvjwqx2qx4m7u4 \
    --proxy=https://devnet-gateway.multiversx.com \
    --function="getStakedAddresses"
```

### Upgrading smart contracts
```sh
mxpy --verbose contract upgrade erd1qqqqqqqqqqqqqpgqd6sq3xt8tkxn2kk3s3yuultxeagcwvjwqx2qx4m7u4 --recall-nonce \
    --bytecode=./output/staking-contract.wasm \
    --recall-nonce --pem=~/MyTestWallets/TestKey.pem \
    --gas-limit=20000000 \
    --send --outfile="upgrade-devnet.interaction.json" \
    --proxy=https://devnet-gateway.multiversx.com --chain=D
```


### Unstaking our devnet tokens
```sh
mxpy --verbose contract call erd1qqqqqqqqqqqqqpgqd6sq3xt8tkxn2kk3s3yuultxeagcwvjwqx2qx4m7u4 \
    --proxy=https://devnet-gateway.multiversx.com --chain=D \
    --send --recall-nonce --pem=~/MyTestWallets/TestKey.pem \
    --gas-limit=10000000 \
    --function="unstake" \
    --arguments 500000000000000000
```

### Update Global Status (Internal to Contract)
```sh
mxpy --verbose contract call erd1qqqqqqqqqqqqqpgqd6sq3xt8tkxn2kk3s3yuultxeagcwvjwqx2qx4m7u4 \
    --proxy=https://devnet-gateway.multiversx.com --chain=D \
    --send --recall-nonce --pem=~/MyTestWallets/TestKey.pem \
    --gas-limit=5000000 \
    --function="update_global_state"
```

### View Pending Rewards
```sh
mxpy --verbose contract query erd1qqqqqqqqqqqqqpgqd6sq3xt8tkxn2kk3s3yuultxeagcwvjwqx2qx4m7u4 \
    --proxy=https://devnet-gateway.multiversx.com \
    --function="getPendingRewards" \
    --arguments ${USER_ADDRESS}
```

### See Total Participated
```sh
mxpy --verbose contract query erd1qqqqqqqqqqqqqpgqd6sq3xt8tkxn2kk3s3yuultxeagcwvjwqx2qx4m7u4 \
    --proxy=https://devnet-gateway.multiversx.com \
    --function="getTotalStaked"
```

### Claim Rewards
```sh
mxpy --verbose contract call erd1qqqqqqqqqqqqqpgqd6sq3xt8tkxn2kk3s3yuultxeagcwvjwqx2qx4m7u4 \
    --proxy=https://devnet-gateway.multiversx.com --chain=D \
    --send --recall-nonce --pem=~/MyTestWallets/TestKey.pem \
    --gas-limit=7000000 \
    --function="claim_rewards"
```