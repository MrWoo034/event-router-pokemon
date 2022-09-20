use ic_cdk::export::candid;
use candid::CandidType;
use ic_types::Principal;
use serde::{Serialize, Deserialize};

pub const PRINCIPAL_ID: &str = "rrkah-fqaaa-aaaaa-aaaaq-cai";

#[derive(Serialize, Deserialize, Clone, Debug, CandidType)]
pub struct Event<T>
where T: Serialize + CandidType {
    pub consumer_address: Principal,
    pub consumer_method: String,
    pub consume_at_timestamp: Option<u64>,
    pub payload: T,
    pub source_timestamp: u64,
}