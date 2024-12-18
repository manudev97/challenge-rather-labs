// Code generated by the multiversx-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Upgrade:                              1
// Endpoints:                           12
// Async Callback (empty):               1
// Total number of exported functions:  15

#![no_std]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    staking_contract
    (
        init => init
        upgrade => upgrade
        stake => stake
        unstake => unstake
        claim_rewards => claim_rewards
        getPendingRewards => pending_rewards
        update_global_state => update_global_state
        getStakingPosition => staking_position
        getStakedAddresses => staked_addresses
        getPendingRewardsMapper => pending_rewards_mapper
        getRewardIndex => reward_index
        getTotalStaked => total_staked
        getGlobalRewardIndex => global_reward_index
        getLastUpdateTimestamp => last_update_timestamp
    )
}

multiversx_sc_wasm_adapter::async_callback_empty! {}
