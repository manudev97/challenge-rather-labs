#![no_std]

multiversx_sc::imports!();

#[multiversx_sc::contract]
pub trait StakingContract {
    #[init]
    fn init(&self) {
        self.total_staked().set(BigUint::zero());
        self.global_reward_index().set(BigUint::zero());
        self.last_update_timestamp().set(self.blockchain().get_block_timestamp());
    }

    #[upgrade]
    fn upgrade(&self) {
        
    }

    #[payable("EGLD")]
    #[endpoint]
    fn stake(&self) {
        let payment_amount = self.call_value().egld_value().clone_value();
        require!(payment_amount > 0, "Your stake must be greater than 0");

        let caller = self.blockchain().get_caller();
        self.update_global_state();

        // Recalcular recompensas pendientes del usuario
        let current_rewards = self.pending_rewards(&caller);
        self.pending_rewards_mapper(&caller)
            .set(&current_rewards);

        // Actualizar la posición de staking del usuario
        let user_stake = self.staking_position(&caller).get();
        self.staking_position(&caller).set(&(user_stake + &payment_amount));
        self.reward_index(&caller)
            .set(&self.global_reward_index().get());

        // Actualizar el total staked
        let total_staked = self.total_staked().get();
        self.total_staked().set(&(total_staked + payment_amount));
    }

    #[endpoint]
    fn unstake(&self, opt_unstake_amount: OptionalValue<BigUint>) {
        let caller = self.blockchain().get_caller();
        self.update_global_state();

        let unstake_amount = match opt_unstake_amount {
            OptionalValue::Some(amt) => amt,
            OptionalValue::None => self.staking_position(&caller).get(),
        };

        let user_stake = self.staking_position(&caller).get();
        require!(
            unstake_amount > 0 && unstake_amount <= user_stake,
            "Invalid unstake amount"
        );

        // Recalcular recompensas pendientes del usuario
        let current_rewards = self.pending_rewards(&caller);
        self.pending_rewards_mapper(&caller)
            .set(&current_rewards);

        // Actualizar la posición de staking del usuario
        self.staking_position(&caller)
            .set(&(user_stake - &unstake_amount));
        self.reward_index(&caller)
            .set(&self.global_reward_index().get());

        // Actualizar el total staked
        let total_staked = self.total_staked().get();
        self.total_staked().set(&(total_staked - &unstake_amount));

        // Transferir EGLD de vuelta al usuario
        self.send().direct_egld(&caller, &unstake_amount);
    }

    #[endpoint]
    fn claim_rewards(&self) {
        let caller = self.blockchain().get_caller();
        self.update_global_state();

        // Obtener recompensas pendientes y reiniciarlas
        let rewards = self.pending_rewards(&caller);
        require!(rewards > 0, "No rewards available to claim");

        self.pending_rewards_mapper(&caller).set(BigUint::zero());
        self.reward_index(&caller)
            .set(&self.global_reward_index().get());

        // Transferir recompensas al usuario
        self.send().direct_egld(&caller, &rewards);
    }

    #[view(getPendingRewards)]
    fn pending_rewards(&self, caller: &ManagedAddress) -> BigUint {
        let user_stake = self.staking_position(caller).get();
        if user_stake == 0 {
            return BigUint::zero();
        }

        let current_index = self.global_reward_index().get();
        let user_index = self.reward_index(caller).get();
        let accrued_rewards = user_stake * (current_index - user_index);
        accrued_rewards + self.pending_rewards_mapper(caller).get()
    }

    #[only_owner]
    #[endpoint]
    fn update_global_state(&self) {
        let current_timestamp = self.blockchain().get_block_timestamp();
        let last_timestamp = self.last_update_timestamp().get();
        if current_timestamp == last_timestamp {
            return;
        }

        let time_elapsed = current_timestamp - last_timestamp;
        let total_staked = self.total_staked().get();

        if total_staked > 0 {
            let reward_rate = BigUint::from(3u64); // 0.0003 EGLD por segundo (ajustado a escala)
            let rewards_generated = reward_rate * time_elapsed;
            let reward_per_share = rewards_generated / total_staked;

            self.global_reward_index()
                .update(|index| *index += reward_per_share);
        }

        self.last_update_timestamp().set(current_timestamp);
    }

    // Mappers de almacenamiento
    #[view(getStakingPosition)]
    #[storage_mapper("stakingPosition")]
    fn staking_position(&self, addr: &ManagedAddress) -> SingleValueMapper<BigUint>;

    #[view(getStakedAddresses)]
    #[storage_mapper("stakedAddresses")]
    fn staked_addresses(&self) -> UnorderedSetMapper<ManagedAddress>;

    #[view(getPendingRewardsMapper)]
    #[storage_mapper("pendingRewards")]
    fn pending_rewards_mapper(&self, addr: &ManagedAddress) -> SingleValueMapper<BigUint>;

    #[view(getRewardIndex)]
    #[storage_mapper("rewardIndex")]
    fn reward_index(&self, addr: &ManagedAddress) -> SingleValueMapper<BigUint>;

    #[view(getTotalStaked)]
    #[storage_mapper("totalStaked")]
    fn total_staked(&self) -> SingleValueMapper<BigUint>;

    #[view(getGlobalRewardIndex)]
    #[storage_mapper("globalRewardIndex")]
    fn global_reward_index(&self) -> SingleValueMapper<BigUint>;

    #[view(getLastUpdateTimestamp)]
    #[storage_mapper("lastUpdateTimestamp")]
    fn last_update_timestamp(&self) -> SingleValueMapper<u64>;
}
