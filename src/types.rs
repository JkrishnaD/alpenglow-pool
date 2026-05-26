use {
    serde::{Deserialize, Serialize},
    solana_bls_signatures::Signature as BLSSignature,
    wincode::{SchemaRead, SchemaWrite},
};

pub type Slot = u64;
pub type Stake = u64;

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    PartialEq,
    Ord,
    Serialize,
    Deserialize,
    PartialOrd,
    SchemaRead,
    SchemaWrite,
)]
pub struct Hash([u8; 32]);

pub type PoolId = (Slot, VoteType);
