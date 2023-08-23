extern crate num_bigint;
extern crate num_traits;

use num_bigint::BigInt;
use num_traits::{ToPrimitive, Zero};

#[derive(Clone, Debug)]
pub struct GasOutputs {
    base_fee_burn: BigInt,
    over_estimation_burn: BigInt,
    miner_penalty: BigInt,
    miner_tip: BigInt,
    refund: BigInt,
    gas_refund: i64,
    gas_burned: i64,
}

fn compute_gas_overestimation_burn(gas_used: i64, gas_limit: i64) -> (i64, i64) {
    if gas_used == 0 {
        return (0, gas_limit);
    }
    let over = gas_limit - (11 * gas_used) / 10;
    if over < 0 {
        return (gas_limit - gas_used, 0);
    }
    let over = std::cmp::min(over, gas_used);
    let gas_to_burn =
        (BigInt::from(gas_limit - gas_used) * BigInt::from(over)) / BigInt::from(gas_used);
    (
        gas_limit - gas_used - gas_to_burn.to_i64().unwrap(),
        gas_to_burn.to_i64().unwrap(),
    )
}

pub fn compute_gas_outputs(
    gas_used: i64,
    gas_limit: i64,
    base_fee: &BigInt,
    fee_cap: &BigInt,
    gas_premium: &BigInt,
    charge_network_fee: bool,
) -> GasOutputs {
    let gas_used_big = BigInt::from(gas_used);
    let mut outputs = GasOutputs {
        base_fee_burn: BigInt::zero(),
        over_estimation_burn: BigInt::zero(),
        miner_penalty: BigInt::zero(),
        miner_tip: BigInt::zero(),
        refund: BigInt::zero(),
        gas_refund: 0,
        gas_burned: 0,
    };

    let mut base_fee_to_pay = base_fee.clone();
    if &base_fee_to_pay > fee_cap {
        base_fee_to_pay = fee_cap.clone();
        outputs.miner_penalty = (base_fee - fee_cap) * &gas_used_big;
    }

    if charge_network_fee {
        outputs.base_fee_burn = &base_fee_to_pay * &gas_used_big;
    }

    let mut miner_tip = gas_premium.clone();
    if &(&base_fee_to_pay + &miner_tip) > fee_cap {
        miner_tip = fee_cap - &base_fee_to_pay;
    }
    outputs.miner_tip = &miner_tip * BigInt::from(gas_limit);

    let (gas_refund, gas_burned) = compute_gas_overestimation_burn(gas_used, gas_limit);
    outputs.gas_refund = gas_refund;
    outputs.gas_burned = gas_burned;

    if outputs.gas_burned != 0 {
        let gas_burned_big = BigInt::from(outputs.gas_burned);
        outputs.over_estimation_burn = &base_fee_to_pay * &gas_burned_big;
        let miner_penalty = (base_fee - &base_fee_to_pay) * &gas_burned_big;
        outputs.miner_penalty += miner_penalty;
    }

    let required_funds = BigInt::from(gas_limit) * fee_cap;
    let refund = &required_funds
        - &outputs.base_fee_burn
        - &outputs.miner_tip
        - &outputs.over_estimation_burn;
    outputs.refund = refund;

    outputs
}
