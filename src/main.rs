use near_sdk::test_utils::{accounts, VMContextBuilder};
use near_sdk::{testing_env, MockedBlockchain};
use clap::{load_yaml, App};

mod math;
use math::*;
mod admin_fee;
use admin_fee::AdminFees;

fn main(){
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    if let Some(matches) = matches.subcommand_matches("calc_d") {
        let amp_string: String = matches.value_of("amp").unwrap().into();
        let amp = amp_string.parse::<u128>().unwrap();
        let c_amounts_str = matches.value_of("c_amounts").unwrap();
        let c_amounts_str_vec = c_amounts_str.split(",").collect::<Vec<&str>>();
        let c_amounts = c_amounts_str_vec.into_iter().map(|item| item.parse::<u128>().unwrap()).collect::<Vec<u128>>();
        let ss = StableSwap::new( amp, amp, 0, 0, 0);
        let d = ss.compute_d(&c_amounts).unwrap();
        println!("calc_d : {}", d);
    }else if let Some(matches) = matches.subcommand_matches("calc_y") {
        let amp_string: String = matches.value_of("amp").unwrap().into();
        let amp = amp_string.parse::<u128>().unwrap();
        let x_c_amount_string: String = matches.value_of("x_c_amount").unwrap().into();
        let x_c_amount = x_c_amount_string.parse::<u128>().unwrap();
        let current_c_amounts_str = matches.value_of("current_c_amounts").unwrap();
        let current_c_amounts_str_vec = current_c_amounts_str.split(",").collect::<Vec<&str>>();
        let current_c_amounts = current_c_amounts_str_vec.into_iter().map(|item| item.parse::<u128>().unwrap()).collect::<Vec<u128>>();
        let index_x_string: String = matches.value_of("index_x").unwrap().into();
        let index_x = index_x_string.parse::<u128>().unwrap() as usize;
        let index_y_string: String = matches.value_of("index_y").unwrap().into();
        let index_y = index_y_string.parse::<u128>().unwrap() as usize;

        let ss = StableSwap::new( amp, amp, 0, 0, 0);
        let y = ss.compute_y(x_c_amount, &current_c_amounts, index_x, index_y).unwrap();
        println!("compute_y : {}", y);
    }else if let Some(matches) = matches.subcommand_matches("calc_add_liquidity") {
        let amp_string: String = matches.value_of("amp").unwrap().into();
        let amp = amp_string.parse::<u128>().unwrap();
        let deposit_c_amounts_str = matches.value_of("deposit_c_amounts").unwrap();
        let deposit_c_amounts_str_vec = deposit_c_amounts_str.split(",").collect::<Vec<&str>>();
        let deposit_c_amounts = deposit_c_amounts_str_vec.into_iter().map(|item| item.parse::<u128>().unwrap()).collect::<Vec<u128>>();
        let old_c_amounts_str = matches.value_of("old_c_amounts").unwrap();
        let old_c_amounts_str_vec = old_c_amounts_str.split(",").collect::<Vec<&str>>();
        let old_c_amounts = old_c_amounts_str_vec.into_iter().map(|item| item.parse::<u128>().unwrap()).collect::<Vec<u128>>();
        let pool_token_supply_string: String = matches.value_of("pool_token_supply").unwrap().into();
        let pool_token_supply = pool_token_supply_string.parse::<u128>().unwrap();
        let total_fee_string: String = matches.value_of("total_fee").unwrap().into();
        let total_fee = total_fee_string.parse::<u32>().unwrap();
        let exchange_fee_string: String = matches.value_of("exchange_fee").unwrap().into();
        let exchange_fee = exchange_fee_string.parse::<u32>().unwrap();

        let mut context = VMContextBuilder::new();
        testing_env!(context.predecessor_account_id(accounts(0)).build());
        let ss = StableSwap::new( amp, amp, 0, 0, 0);
        let (minted, fee_part) = ss.compute_lp_amount_for_deposit(&deposit_c_amounts, &old_c_amounts, pool_token_supply, &Fees::new(total_fee, &AdminFees::new(exchange_fee))).unwrap();
        println!("minted : {}", minted);
        println!("fee_part : {}", fee_part);
    }else if let Some(matches) = matches.subcommand_matches("calc_remove_liquidity") {
        let shares_string: String = matches.value_of("shares").unwrap().into();
        let shares = shares_string.parse::<u128>().unwrap();
        let c_amounts_str = matches.value_of("c_amounts").unwrap();
        let c_amounts_str_vec = c_amounts_str.split(",").collect::<Vec<&str>>();
        let c_amounts = c_amounts_str_vec.into_iter().map(|item| item.parse::<u128>().unwrap()).collect::<Vec<u128>>();
        let pool_token_supply_string: String = matches.value_of("pool_token_supply").unwrap().into();
        let pool_token_supply = pool_token_supply_string.parse::<u128>().unwrap();

        let mut result = vec![0u128; c_amounts.len()];
        for i in 0..c_amounts.len() {
            result[i] = U256::from(c_amounts[i])
                .checked_mul(shares.into())
                .unwrap()
                .checked_div(pool_token_supply.into())
                .unwrap()
                .as_u128();
        }

        println!("calc_remove_liquidity : {:?}", result);
    }else if let Some(matches) = matches.subcommand_matches("calc_remove_liquidity_by_tokens") {
        let amp_string: String = matches.value_of("amp").unwrap().into();
        let amp = amp_string.parse::<u128>().unwrap();
        let removed_c_amounts_str = matches.value_of("removed_c_amounts").unwrap();
        let removed_c_amounts_str_vec = removed_c_amounts_str.split(",").collect::<Vec<&str>>();
        let removed_c_amounts = removed_c_amounts_str_vec.into_iter().map(|item| item.parse::<u128>().unwrap()).collect::<Vec<u128>>();
        let old_c_amounts_str = matches.value_of("old_c_amounts").unwrap();
        let old_c_amounts_str_vec = old_c_amounts_str.split(",").collect::<Vec<&str>>();
        let old_c_amounts = old_c_amounts_str_vec.into_iter().map(|item| item.parse::<u128>().unwrap()).collect::<Vec<u128>>();
        let pool_token_supply_string: String = matches.value_of("pool_token_supply").unwrap().into();
        let pool_token_supply = pool_token_supply_string.parse::<u128>().unwrap();
        let total_fee_string: String = matches.value_of("total_fee").unwrap().into();
        let total_fee = total_fee_string.parse::<u32>().unwrap();
        let exchange_fee_string: String = matches.value_of("exchange_fee").unwrap().into();
        let exchange_fee = exchange_fee_string.parse::<u32>().unwrap();

        let mut context = VMContextBuilder::new();
        testing_env!(context.predecessor_account_id(accounts(0)).build());
        let ss = StableSwap::new( amp, amp, 0, 0, 0);
        let (burned, fee_part) = ss.compute_lp_amount_for_withdraw(&removed_c_amounts, &old_c_amounts, pool_token_supply, &Fees::new(total_fee, &AdminFees::new(exchange_fee))).unwrap();
        println!("burned : {}", burned);
        println!("fee_part : {}", fee_part);
    }else if let Some(matches) = matches.subcommand_matches("calc_swap") {
        let amp_string: String = matches.value_of("amp").unwrap().into();
        let amp = amp_string.parse::<u128>().unwrap();
        let in_token_idx_string: String = matches.value_of("in_token_idx").unwrap().into();
        let in_token_idx = in_token_idx_string.parse::<u128>().unwrap() as usize;
        let in_c_amount_string: String = matches.value_of("in_c_amount").unwrap().into();
        let in_c_amount = in_c_amount_string.parse::<u128>().unwrap();
        let out_token_idx_string: String = matches.value_of("out_token_idx").unwrap().into();
        let out_token_idx = out_token_idx_string.parse::<u128>().unwrap() as usize;
        let old_c_amounts_str = matches.value_of("old_c_amounts").unwrap();
        let old_c_amounts_str_vec = old_c_amounts_str.split(",").collect::<Vec<&str>>();
        let old_c_amounts = old_c_amounts_str_vec.into_iter().map(|item| item.parse::<u128>().unwrap()).collect::<Vec<u128>>();
        let total_fee_string: String = matches.value_of("total_fee").unwrap().into();
        let total_fee = total_fee_string.parse::<u32>().unwrap();
        let exchange_fee_string: String = matches.value_of("exchange_fee").unwrap().into();
        let exchange_fee = exchange_fee_string.parse::<u32>().unwrap();

        let mut context = VMContextBuilder::new();
        testing_env!(context.predecessor_account_id(accounts(0)).build());
        let ss = StableSwap::new( amp, amp, 0, 0, 0);
        let res = ss.swap_to(in_token_idx, in_c_amount, out_token_idx, &old_c_amounts, &Fees::new(total_fee, &AdminFees::new(exchange_fee))).unwrap();
        println!("swap_out : {}", res.amount_swapped);
        println!("fee_part : {}", res.fee);
    }
}