use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("BmRad81tyedXignzFL9NEDzQmnDVpcUDvc3Ny8q7gKU1");

#[program]
pub mod alive_vote {
    use super::*;

    pub fn create_vote_definition(_ctx: Context<CreateVoteDef>, title: String) -> Result<()> {
        let vote_definition: &mut Account<VoteDefinition> = &mut _ctx.accounts.vote_definition;
        let author: &Signer = &_ctx.accounts.author;

        if title.chars().count() > MAX_TITLE_LENGTH {
            return Err(ErrorCode::TitleTooLong.into())
        }

        vote_definition.title = title;
        vote_definition.author = *author.key;

        Ok(())
    }

    pub fn update_vote_definition(_ctx: Context<CreateVoteDef>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateVoteDef<'info> {
    #[account(init, payer = author, space = VoteDefinition::LEN)]
    pub vote_definition: Account<'info, VoteDefinition>,
    #[account(mut)]
    pub author: Signer<'info>,
    #[account(address = system_program::ID)]
    pub system_program: AccountInfo<'info>,
}

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub enum ElectionType {
    Poll,
    Primary,
    General,
    Recall,
    Referendum,
    Initiative,
}

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub enum VoteType {
    Office,
    Proposition,
}

#[account]
pub struct VoteDefinition {
    pub author: Pubkey,
    pub title: String, //128 characters
    pub election_type: ElectionType,
    pub vote_type: VoteType,
    pub max_vote_choices: u8,
    pub min_vote_choices: u8,
    pub allow_write_in: bool,
    pub election_date: String, //YYYYMMDD
}

const DISCRIMINATOR_LENGTH: usize = 8;
const PUBLIC_KEY_LENGTH: usize = 32;
const TITLE_LENGTH_PREFIX: usize = 4;
const MAX_TITLE_LENGTH: usize = 50 * 4; // 50 chars max.
const ELECTION_TYPE_LENGTH: usize = 4;

impl VoteDefinition {
    const LEN: usize = DISCRIMINATOR_LENGTH
        + PUBLIC_KEY_LENGTH //author
        + TITLE_LENGTH_PREFIX
        + MAX_TITLE_LENGTH
        + ELECTION_TYPE_LENGTH;
}

#[error_code]
pub enum ErrorCode {
    #[msg("The title cannot excedd 128 characters.")]
    TitleTooLong,
}
