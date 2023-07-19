mod pb;
use substreams::{log, store};
use substreams_solana::pb::sol as solana;
use substreams_ethereum::pb::eth::v2 as eth;
// use substreams_solana::pb::sf::solana as sol;
use chrono::NaiveDateTime;
use pb::acme;
use substreams::Hex;
use substreams_database_change::tables::Tables;
use substreams_database_change::pb::database::{table_change::Operation, DatabaseChanges};
use pb::acme::{SolanaBlockHeader, TransactionList, Transaction, Reward,RewardsList, TokenBalance, TokenBalanceList, UiTokenAmount };
use substreams_ethereum::pb::eth::v2::BigInt;

#[substreams::handlers::map]
fn map_block(block: solana::v1::Block) -> Result<SolanaBlockHeader, substreams::errors::Error> {

    let timestamp: u64 = block.block_time.unwrap_or_default().timestamp as u64;
    let block_height: u64 = block.block_height.unwrap_or_default().block_height as u64;
    let rewards: Vec<Reward> = block.rewards.iter().map(|reward| {
        Reward {
            pubkey: reward.pubkey.clone(),
            lamports: reward.lamports as u64,
            post_balance: reward.post_balance,
            reward_type: reward.reward_type as u32,
            commission: reward.commission.clone(),
        }
    }).collect();
    Ok(SolanaBlockHeader{
        hash: block.blockhash,
        number: block.slot,
        parent_hash: block.previous_blockhash,
        parent_number: block.parent_slot,
        timestamp: timestamp.to_string(),
        transaction_count:  block.transactions.len() as u64,
        rewards_list: Some(pb::acme::RewardsList { rewards_list: rewards }),
        block_height: block_height,
    })
}

#[substreams::handlers::map]
fn map_trx(block: solana::v1::Block) -> Result<TransactionList, substreams::errors::Error> {
    let transaction_details_list = block
        .transactions
        .clone().into_iter()
        .map(|trx| process_transaction_trace(trx.meta.unwrap_or_default(), block.clone()))
        .collect();

    Ok(TransactionList {
        transaction_details_list,
    })
}
fn process_transaction_trace(trx: solana::v1::TransactionStatusMeta, block: solana::v1::Block) -> Transaction {
    let token_balances: Vec<TokenBalance> = trx.post_token_balances.iter().map(|balance| {
        let ui_token_amount = balance.ui_token_amount.as_ref().map(|ui_token| UiTokenAmount {
            ui_amount: ui_token.ui_amount,
            decimals: ui_token.decimals,
            amount: ui_token.amount.clone(),
            ui_amount_string: ui_token.ui_amount_string.clone(),
        });

        TokenBalance {
            account_index: balance.account_index,
            mint: balance.mint.clone(),
            ui_token_amount: ui_token_amount,
            owner: balance.owner.clone(),
            program_id: balance.owner.clone(),
        }
    }).collect();
    let rewards: Vec<Reward> = trx.rewards.iter().map(|reward| {
        Reward {
            pubkey: reward.pubkey.clone(),
            lamports: reward.lamports as u64,
            post_balance: reward.post_balance,
            reward_type: reward.reward_type as u32,
            commission: reward.commission.clone(),
        }
    }).collect();
    let pre_balances: Vec<u64> = trx.pre_balances.into_iter().collect();
    let post_balances: Vec<u64> = trx.post_balances.into_iter().collect();
    let log_messages: Vec<String> = trx.log_messages.into_iter().collect();
    Transaction {
        block_hash: block.blockhash.clone(),
        timestamp: block.block_time.unwrap_or_default().timestamp.to_string(),
        block_number: block.slot,
        error: block.blockhash.clone().to_string(),
        fee: trx.fee,
        inner_instructions_none: trx.inner_instructions_none,
        log_messages_none: trx.log_messages_none,
        token_balance_list: Some(pb::acme::TokenBalanceList { token_balance_list: token_balances }),
        post_balances: post_balances,
        pre_balances: pre_balances,
        log_messages: log_messages,
        rewards_list: Some(pb::acme::RewardsList { rewards_list: rewards }),

    }
}


fn base_64_to_hex<T: std::convert::AsRef<[u8]>>(num:T) -> String {
    let num = hex::encode(&num);
    let num = num.to_string();
     format!("0x{}", &num)
}

fn option_bigint_to_number_string(bigint: Option<BigInt>) -> String {
    bigint
        .map(|num| {
            let bytes = num.bytes;
            let mut value: u128 = 0;

            for byte in bytes {
                value = (value << 8) + u128::from(byte);
            }

            value.to_string()
        })
        .unwrap_or_else(String::new)
}


