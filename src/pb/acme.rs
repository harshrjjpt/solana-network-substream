// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BigInt {
    #[prost(bytes="vec", tag="1")]
    pub bytes: ::prost::alloc::vec::Vec<u8>,
}
/// ## Block Details ##
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SolanaBlockHeader {
    #[prost(string, tag="1")]
    pub hash: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub number: u64,
    #[prost(string, tag="3")]
    pub parent_hash: ::prost::alloc::string::String,
    #[prost(uint64, tag="4")]
    pub parent_number: u64,
    #[prost(string, tag="5")]
    pub timestamp: ::prost::alloc::string::String,
    #[prost(uint64, tag="6")]
    pub transaction_count: u64,
    #[prost(uint64, tag="7")]
    pub block_height: u64,
    #[prost(message, optional, tag="8")]
    pub rewards_list: ::core::option::Option<RewardsList>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RewardsList {
    #[prost(message, repeated, tag="1")]
    pub rewards_list: ::prost::alloc::vec::Vec<Reward>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Reward {
    #[prost(string, tag="1")]
    pub pubkey: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub lamports: u64,
    #[prost(uint64, tag="3")]
    pub post_balance: u64,
    #[prost(uint32, tag="4")]
    pub reward_type: u32,
    #[prost(string, tag="5")]
    pub commission: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transaction {
    #[prost(string, tag="1")]
    pub block_hash: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub timestamp: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub block_number: u64,
    #[prost(uint64, tag="4")]
    pub fee: u64,
    #[prost(string, tag="5")]
    pub error: ::prost::alloc::string::String,
    #[prost(bool, tag="6")]
    pub inner_instructions_none: bool,
    #[prost(bool, tag="7")]
    pub log_messages_none: bool,
    #[prost(message, optional, tag="8")]
    pub token_balance_list: ::core::option::Option<TokenBalanceList>,
    #[prost(uint64, repeated, tag="9")]
    pub post_balances: ::prost::alloc::vec::Vec<u64>,
    /// CustomList post_token_balances =9;
    #[prost(uint64, repeated, tag="10")]
    pub pre_balances: ::prost::alloc::vec::Vec<u64>,
    #[prost(string, repeated, tag="11")]
    pub log_messages: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// CustomList pre_token_balances =11;
    #[prost(message, optional, tag="12")]
    pub rewards_list: ::core::option::Option<RewardsList>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TokenBalanceList {
    #[prost(message, repeated, tag="1")]
    pub token_balance_list: ::prost::alloc::vec::Vec<TokenBalance>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TokenBalance {
    #[prost(uint32, tag="1")]
    pub account_index: u32,
    #[prost(string, tag="2")]
    pub mint: ::prost::alloc::string::String,
    #[prost(message, optional, tag="4")]
    pub ui_token_amount: ::core::option::Option<UiTokenAmount>,
    #[prost(string, tag="5")]
    pub owner: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub program_id: ::prost::alloc::string::String,
}
// message UiTokenAmountList {
//    repeated UiTokenAmount Ui_token_amount_list = 1;
// }

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UiTokenAmount {
    #[prost(double, tag="1")]
    pub ui_amount: f64,
    #[prost(uint32, tag="2")]
    pub decimals: u32,
    #[prost(string, tag="3")]
    pub amount: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub ui_amount_string: ::prost::alloc::string::String,
}
/// ## Transaction List ##
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionList {
    #[prost(message, repeated, tag="1")]
    pub transaction_details_list: ::prost::alloc::vec::Vec<Transaction>,
}
// @@protoc_insertion_point(module)
