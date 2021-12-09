//! Program state processor

use {
    crate::instruction::*,
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint::ProgramResult,
        msg,
        program_error::ProgramError,
        program_pack::Pack,
        pubkey::Pubkey,
        sysvar::{clock::Clock, Sysvar},
    },
    std::convert::TryInto,
};

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    input: &[u8],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let subject = input
        .get(0)
        .cloned()
        .and_then(Subject::from_ordinal)
        .ok_or(ProgramError::InvalidArgument)?;
    let operation = input
        .get(1)
        .cloned()
        .and_then(Operation::from_ordinal)
        .ok_or(ProgramError::InvalidArgument)?;

    let ok = {
        if subject == Subject::AccountOwner {
            let actual_owner = next_account_info(account_info_iter)?.owner;
            let test_owner = next_account_info(account_info_iter)?.key;

            msg!(&format!(
                "{}: {} {} {}",
                subject, actual_owner, operation, test_owner
            ));

            match operation {
                Operation::Equal => actual_owner == test_owner,
                Operation::NotEqual => actual_owner != test_owner,
                _ => return Err(ProgramError::InvalidArgument),
            }
        } else {
            let actual_value = match subject {
                Subject::AccountLamports => next_account_info(account_info_iter)?.lamports(),
                Subject::AccountOwner => unreachable!(),
                Subject::AccountSpace => next_account_info(account_info_iter)?.data_len() as u64,
                Subject::Epoch => Clock::get()?.epoch,
                Subject::SplTokenAmount => {
                    let account = accounts
                        .get(0)
                        .and_then(|account| {
                            if *account.owner == spl_token::id() {
                                Some(account)
                            } else {
                                None
                            }
                        })
                        .ok_or(ProgramError::InvalidArgument)?;

                    let token_account = spl_token::state::Account::unpack(&account.data.borrow())?;
                    token_account.amount
                }
            };
            let test_value = input[2..10]
                .try_into()
                .ok()
                .map(u64::from_le_bytes)
                .ok_or(ProgramError::InvalidArgument)?;

            msg!(&format!(
                "{}: {} {} {}",
                subject, actual_value, operation, test_value
            ));

            match operation {
                Operation::Equal => actual_value == test_value,
                Operation::NotEqual => actual_value != test_value,
                Operation::LessThan => actual_value < test_value,
                Operation::LessThanOrEqual => actual_value <= test_value,
                Operation::GreaterThan => actual_value > test_value,
                Operation::GreaterThanOrEqual => actual_value >= test_value,
            }
        }
    };

    if ok {
        Ok(())
    } else {
        Err(ProgramError::Custom(0))
    }
}
