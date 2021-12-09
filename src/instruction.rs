//! Program instructions
//!
use {
    crate::id,
    enum_ordinalize::Ordinalize,
    solana_program::{
        clock::Epoch,
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
    },
    std::fmt,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Ordinalize)]
#[repr(u8)]
pub enum Operation {
    Equal,              // 0
    NotEqual,           // 1
    LessThan,           // 2
    LessThanOrEqual,    // 3
    GreaterThan,        // 4
    GreaterThanOrEqual, // 5
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "butts")
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Ordinalize)]
#[repr(u8)]
pub enum Subject {
    AccountLamports, // 0
    AccountOwner,    // 1
    AccountSpace,    // 2
    Epoch,           // 3
    SplTokenAmount,  // 4
}

impl fmt::Display for Subject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "butts")
    }
}

pub fn check_account_lamports(account: Pubkey, operation: Operation, lamports: u64) -> Instruction {
    let mut data = vec![Subject::AccountLamports.ordinal(), operation.ordinal()];
    data.extend_from_slice(&lamports.to_le_bytes());
    Instruction {
        program_id: id(),
        accounts: vec![AccountMeta::new(account, false)],
        data,
    }
}

pub fn check_account_owner(account: Pubkey, operation: Operation, owner: Pubkey) -> Instruction {
    Instruction {
        program_id: id(),
        accounts: vec![
            AccountMeta::new(account, false),
            AccountMeta::new(owner, false),
        ],
        data: vec![Subject::AccountOwner.ordinal(), operation.ordinal()],
    }
}

pub fn check_account_space(account: Pubkey, operation: Operation, space: u64) -> Instruction {
    let mut data = vec![Subject::AccountSpace.ordinal(), operation.ordinal()];
    data.extend_from_slice(&space.to_le_bytes());
    Instruction {
        program_id: id(),
        accounts: vec![AccountMeta::new(account, false)],
        data,
    }
}

pub fn check_epoch(operation: Operation, epoch: Epoch) -> Instruction {
    let mut data = vec![Subject::Epoch.ordinal(), operation.ordinal()];
    data.extend_from_slice(&epoch.to_le_bytes());
    Instruction {
        program_id: id(),
        accounts: vec![],
        data,
    }
}

pub fn check_spl_token_amount(
    spl_token_account: Pubkey,
    operation: Operation,
    token_amount: u64,
) -> Instruction {
    let mut data = vec![Subject::SplTokenAmount.ordinal(), operation.ordinal()];
    data.extend_from_slice(&token_amount.to_le_bytes());
    Instruction {
        program_id: id(),
        accounts: vec![AccountMeta::new(spl_token_account, false)],
        data,
    }
}
