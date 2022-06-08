use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{CanonicalAddr, Storage};
use cosmwasm_storage::{
    bucket, bucket_read, singleton, singleton_read, Bucket, ReadonlyBucket, ReadonlySingleton,
    Singleton,
};

pub static CONFIG_KEY: &[u8] = b"config";
pub const BUCKET_USER_CREDITS: &[u8] = b"usercredits";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub owner: CanonicalAddr,
}

pub fn config<S: Storage>(storage: &mut S) -> Singleton<S, State> {
    singleton(storage, CONFIG_KEY)
}

pub fn config_read<S: Storage>(storage: &S) -> ReadonlySingleton<S, State> {
    singleton_read(storage, CONFIG_KEY)
}

pub fn credits_storage<S: Storage>(store: &mut S) -> Bucket<S, u64> {
    bucket(BUCKET_USER_CREDITS, store)
}

pub fn credits_storage_read<S: Storage>(store: &S) -> ReadonlyBucket<S, u64> {
    bucket_read(BUCKET_USER_CREDITS, store)
}
