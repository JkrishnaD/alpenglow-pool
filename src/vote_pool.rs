use {
    crate::{types::Stake, vote::Vote},
    solana_bls_signatures::Signature as BLSSignature,
    solana_pubkey::Pubkey,
    std::collections::BTreeSet,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VoteMessage {
    /// The type of the vote.
    pub vote: Vote,
    /// The signature.
    pub signature: BLSSignature,
    /// The rank of the validator.
    pub rank: u16,
}

#[derive(Default)]
pub struct VotePool {
    pub votes: Vec<VoteMessage>,
    pub total_stake: Stake,
    pub prev_voted_validator: BTreeSet<Pubkey>,
}

impl VotePool {
    pub fn add_vote(
        &mut self,
        validator_vote_key: Pubkey,
        validator_stake: Stake,
        vote: VoteMessage,
    ) -> Option<Stake> {
        if !self.prev_voted_validator.insert(validator_vote_key) {
            return None;
        }
        self.votes.push(vote);
        self.total_stake = self.total_stake.saturating_add(validator_stake);
        Some(self.total_stake)
    }

    pub fn votes(&self) -> &[VoteMessage] {
        &self.votes
    }

    pub fn total_stake(&self) -> Stake {
        self.total_stake
    }

    pub fn has_prev_voted_validator(&self, validator_vote_key: &Pubkey) -> bool {
        self.prev_voted_validator.contains(validator_vote_key)
    }
}
