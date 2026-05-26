use crate::vote::VoteType;

pub type Slot = u64;
pub type Stake = u64;

pub type PoolId = (Slot, VoteType);
