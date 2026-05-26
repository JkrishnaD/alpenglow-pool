use {
    crate::types::Slot,
    serde::{Deserialize, Serialize},
    solana_hash::Hash,
    wincode::{SchemaRead, SchemaWrite},
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VoteType {
    /// Finalize vote.
    Finalize,
    /// Notarize vote.
    Notarize,
    /// Notarize fallback vote.
    NotarizeFallback,
    /// Skip vote
    Skip,
    /// Skip fallback vote.
    SkipFallback,
    /// Genesis vote.
    Genesis,
}

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, SchemaWrite, SchemaRead,
)]
pub enum Vote {
    /// A notarization vote
    Notarize(NotarizationVote),
    /// A finalization vote
    Finalize(FinalizationVote),
    /// A skip vote
    Skip(SkipVote),
    /// A notarization fallback vote
    NotarizeFallback(NotarizationFallbackVote),
    /// A skip fallback vote
    SkipFallback(SkipFallbackVote),
    /// A genesis vote, only used during the TowerBFT -> Alpenglow Migration
    Genesis(GenesisVote),
}

/// A vote cast by a validator for a given slot.
impl Vote {
    pub fn new_notarization_vote(slot: Slot, block_id: Hash) -> Self {
        Self::Notarize(NotarizationVote { slot, block_id })
    }

    pub fn new_finalization_vote(slot: Slot) -> Self {
        Self::Finalize(FinalizationVote { slot })
    }

    pub fn new_skip_vote(slot: Slot) -> Self {
        Self::Skip(SkipVote { slot })
    }

    pub fn new_notarize_fallback_vote(slot: Slot, block_id: Hash) -> Self {
        Self::NotarizeFallback(NotarizationFallbackVote { slot, block_id })
    }

    pub fn new_skip_fallback_vote(slot: Slot) -> Self {
        Self::SkipFallback(SkipFallbackVote { slot })
    }

    pub fn new_genesis_vote(slot: Slot, block_id: Hash) -> Self {
        Self::Genesis(GenesisVote { slot, block_id })
    }

    /// The slot this vote is cast for.
    pub fn slot(&self) -> Slot {
        match self {
            Self::Notarize(vote) => vote.slot,
            Self::Finalize(vote) => vote.slot,
            Self::Skip(vote) => vote.slot,
            Self::NotarizeFallback(vote) => vote.slot,
            Self::SkipFallback(vote) => vote.slot,
            Self::Genesis(vote) => vote.slot,
        }
    }

    /// The block id associated with the block which was voted for
    pub fn block_id(&self) -> Option<&Hash> {
        match self {
            Self::Notarize(vote) => Some(&vote.block_id),
            Self::Finalize(_) => None,
            Self::Skip(_) => None,
            Self::NotarizeFallback(vote) => Some(&vote.block_id),
            Self::SkipFallback(_) => None,
            Self::Genesis(vote) => Some(&vote.block_id),
        }
    }
}

/// Represents the type of a vote.
impl Vote {
    pub fn is_notarization_vote(&self) -> bool {
        matches!(self, Self::Notarize(_))
    }

    pub fn is_finalization_vote(&self) -> bool {
        matches!(self, Self::Finalize(_))
    }

    pub fn is_skip_vote(&self) -> bool {
        matches!(self, Self::Skip(_))
    }

    pub fn is_notarization_fallback_vote(&self) -> bool {
        matches!(self, Self::NotarizeFallback(_))
    }

    pub fn is_skip_fallback_vote(&self) -> bool {
        matches!(self, Self::SkipFallback(_))
    }

    pub fn is_genesis_vote(&self) -> bool {
        matches!(self, Self::Genesis(_))
    }

    pub fn get_vote_type(&self) -> VoteType {
        match self {
            Self::Notarize(_) => VoteType::Notarize,
            Self::Finalize(_) => VoteType::Finalize,
            Self::Skip(_) => VoteType::Skip,
            Self::NotarizeFallback(_) => VoteType::NotarizeFallback,
            Self::SkipFallback(_) => VoteType::SkipFallback,
            Self::Genesis(_) => VoteType::Genesis,
        }
    }
}

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    Default,
    Serialize,
    Deserialize,
    SchemaWrite,
    SchemaRead,
)]
pub struct NotarizationVote {
    /// The slot this vote is cast for.
    pub slot: Slot,
    /// The block id this vote is for.
    pub block_id: Hash,
}

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    Default,
    Serialize,
    Deserialize,
    SchemaWrite,
    SchemaRead,
)]
pub struct FinalizationVote {
    /// The slot this vote is cast for.
    pub slot: Slot,
}

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    Default,
    Serialize,
    Deserialize,
    SchemaWrite,
    SchemaRead,
)]
pub struct SkipVote {
    /// The slot this vote is cast for.
    pub slot: Slot,
}

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    Default,
    Serialize,
    Deserialize,
    SchemaWrite,
    SchemaRead,
)]
pub struct NotarizationFallbackVote {
    pub slot: Slot,
    pub block_id: Hash,
}

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    Default,
    Serialize,
    Deserialize,
    SchemaWrite,
    SchemaRead,
)]
pub struct SkipFallbackVote {
    pub slot: Slot,
}

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    Default,
    Serialize,
    Deserialize,
    SchemaWrite,
    SchemaRead,
)]
pub struct GenesisVote {
    /// The slot this vote is cast for.
    pub slot: Slot,
    /// The block id this vote is for.
    pub block_id: Hash,
}
