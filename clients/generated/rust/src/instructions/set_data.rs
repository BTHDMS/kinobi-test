//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! <https://github.com/kinobi-so/kinobi>
//!

use borsh::BorshDeserialize;
use borsh::BorshSerialize;

/// Accounts.
pub struct SetData {
      
              
          pub authority: solana_program::pubkey::Pubkey,
          
              
          pub pda: solana_program::pubkey::Pubkey,
      }

impl SetData {
  pub fn instruction(&self, args: SetDataInstructionArgs) -> solana_program::instruction::Instruction {
    self.instruction_with_remaining_accounts(args, &[])
  }
  #[allow(clippy::vec_init_then_push)]
  pub fn instruction_with_remaining_accounts(&self, args: SetDataInstructionArgs, remaining_accounts: &[solana_program::instruction::AccountMeta]) -> solana_program::instruction::Instruction {
    let mut accounts = Vec::with_capacity(2 + remaining_accounts.len());
                            accounts.push(solana_program::instruction::AccountMeta::new(
            self.authority,
            true
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            self.pda,
            false
          ));
                      accounts.extend_from_slice(remaining_accounts);
    let mut data = SetDataInstructionData::new().try_to_vec().unwrap();
          let mut args = args.try_to_vec().unwrap();
      data.append(&mut args);
    
    solana_program::instruction::Instruction {
      program_id: crate::KINOBI_TEST_ID,
      accounts,
      data,
    }
  }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct SetDataInstructionData {
            discriminator: [u8; 8],
            }

impl SetDataInstructionData {
  pub fn new() -> Self {
    Self {
                        discriminator: [223, 114, 91, 136, 197, 78, 153, 153],
                                }
  }
}

impl Default for SetDataInstructionData {
  fn default() -> Self {
    Self::new()
  }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetDataInstructionArgs {
                  pub data: u32,
      }


/// Instruction builder for `SetData`.
///
/// ### Accounts:
///
                      ///   0. `[writable, signer]` authority
                ///   1. `[writable]` pda
#[derive(Clone, Debug, Default)]
pub struct SetDataBuilder {
            authority: Option<solana_program::pubkey::Pubkey>,
                pda: Option<solana_program::pubkey::Pubkey>,
                        data: Option<u32>,
        __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl SetDataBuilder {
  pub fn new() -> Self {
    Self::default()
  }
            #[inline(always)]
    pub fn authority(&mut self, authority: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.authority = Some(authority);
                    self
    }
            #[inline(always)]
    pub fn pda(&mut self, pda: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.pda = Some(pda);
                    self
    }
                    #[inline(always)]
      pub fn data(&mut self, data: u32) -> &mut Self {
        self.data = Some(data);
        self
      }
        /// Add an aditional account to the instruction.
  #[inline(always)]
  pub fn add_remaining_account(&mut self, account: solana_program::instruction::AccountMeta) -> &mut Self {
    self.__remaining_accounts.push(account);
    self
  }
  /// Add additional accounts to the instruction.
  #[inline(always)]
  pub fn add_remaining_accounts(&mut self, accounts: &[solana_program::instruction::AccountMeta]) -> &mut Self {
    self.__remaining_accounts.extend_from_slice(accounts);
    self
  }
  #[allow(clippy::clone_on_copy)]
  pub fn instruction(&self) -> solana_program::instruction::Instruction {
    let accounts = SetData {
                              authority: self.authority.expect("authority is not set"),
                                        pda: self.pda.expect("pda is not set"),
                      };
          let args = SetDataInstructionArgs {
                                                              data: self.data.clone().expect("data is not set"),
                                    };
    
    accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
  }
}

  /// `set_data` CPI accounts.
  pub struct SetDataCpiAccounts<'a, 'b> {
          
                    
              pub authority: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub pda: &'b solana_program::account_info::AccountInfo<'a>,
            }

/// `set_data` CPI instruction.
pub struct SetDataCpi<'a, 'b> {
  /// The program to invoke.
  pub __program: &'b solana_program::account_info::AccountInfo<'a>,
      
              
          pub authority: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub pda: &'b solana_program::account_info::AccountInfo<'a>,
            /// The arguments for the instruction.
    pub __args: SetDataInstructionArgs,
  }

impl<'a, 'b> SetDataCpi<'a, 'b> {
  pub fn new(
    program: &'b solana_program::account_info::AccountInfo<'a>,
          accounts: SetDataCpiAccounts<'a, 'b>,
              args: SetDataInstructionArgs,
      ) -> Self {
    Self {
      __program: program,
              authority: accounts.authority,
              pda: accounts.pda,
                    __args: args,
          }
  }
  #[inline(always)]
  pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
    self.invoke_signed_with_remaining_accounts(&[], &[])
  }
  #[inline(always)]
  pub fn invoke_with_remaining_accounts(&self, remaining_accounts: &[(&'b solana_program::account_info::AccountInfo<'a>, bool, bool)]) -> solana_program::entrypoint::ProgramResult {
    self.invoke_signed_with_remaining_accounts(&[], remaining_accounts)
  }
  #[inline(always)]
  pub fn invoke_signed(&self, signers_seeds: &[&[&[u8]]]) -> solana_program::entrypoint::ProgramResult {
    self.invoke_signed_with_remaining_accounts(signers_seeds, &[])
  }
  #[allow(clippy::clone_on_copy)]
  #[allow(clippy::vec_init_then_push)]
  pub fn invoke_signed_with_remaining_accounts(
    &self,
    signers_seeds: &[&[&[u8]]],
    remaining_accounts: &[(&'b solana_program::account_info::AccountInfo<'a>, bool, bool)]
  ) -> solana_program::entrypoint::ProgramResult {
    let mut accounts = Vec::with_capacity(2 + remaining_accounts.len());
                            accounts.push(solana_program::instruction::AccountMeta::new(
            *self.authority.key,
            true
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            *self.pda.key,
            false
          ));
                      remaining_accounts.iter().for_each(|remaining_account| {
      accounts.push(solana_program::instruction::AccountMeta {
          pubkey: *remaining_account.0.key,
          is_signer: remaining_account.1,
          is_writable: remaining_account.2,
      })
    });
    let mut data = SetDataInstructionData::new().try_to_vec().unwrap();
          let mut args = self.__args.try_to_vec().unwrap();
      data.append(&mut args);
    
    let instruction = solana_program::instruction::Instruction {
      program_id: crate::KINOBI_TEST_ID,
      accounts,
      data,
    };
    let mut account_infos = Vec::with_capacity(2 + 1 + remaining_accounts.len());
    account_infos.push(self.__program.clone());
                  account_infos.push(self.authority.clone());
                        account_infos.push(self.pda.clone());
              remaining_accounts.iter().for_each(|remaining_account| account_infos.push(remaining_account.0.clone()));

    if signers_seeds.is_empty() {
      solana_program::program::invoke(&instruction, &account_infos)
    } else {
      solana_program::program::invoke_signed(&instruction, &account_infos, signers_seeds)
    }
  }
}

/// Instruction builder for `SetData` via CPI.
///
/// ### Accounts:
///
                      ///   0. `[writable, signer]` authority
                ///   1. `[writable]` pda
#[derive(Clone, Debug)]
pub struct SetDataCpiBuilder<'a, 'b> {
  instruction: Box<SetDataCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> SetDataCpiBuilder<'a, 'b> {
  pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
    let instruction = Box::new(SetDataCpiBuilderInstruction {
      __program: program,
              authority: None,
              pda: None,
                                            data: None,
                    __remaining_accounts: Vec::new(),
    });
    Self { instruction }
  }
      #[inline(always)]
    pub fn authority(&mut self, authority: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.authority = Some(authority);
                    self
    }
      #[inline(always)]
    pub fn pda(&mut self, pda: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.pda = Some(pda);
                    self
    }
                    #[inline(always)]
      pub fn data(&mut self, data: u32) -> &mut Self {
        self.instruction.data = Some(data);
        self
      }
        /// Add an additional account to the instruction.
  #[inline(always)]
  pub fn add_remaining_account(&mut self, account: &'b solana_program::account_info::AccountInfo<'a>, is_writable: bool, is_signer: bool) -> &mut Self {
    self.instruction.__remaining_accounts.push((account, is_writable, is_signer));
    self
  }
  /// Add additional accounts to the instruction.
  ///
  /// Each account is represented by a tuple of the `AccountInfo`, a `bool` indicating whether the account is writable or not,
  /// and a `bool` indicating whether the account is a signer or not.
  #[inline(always)]
  pub fn add_remaining_accounts(&mut self, accounts: &[(&'b solana_program::account_info::AccountInfo<'a>, bool, bool)]) -> &mut Self {
    self.instruction.__remaining_accounts.extend_from_slice(accounts);
    self
  }
  #[inline(always)]
  pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
    self.invoke_signed(&[])
  }
  #[allow(clippy::clone_on_copy)]
  #[allow(clippy::vec_init_then_push)]
  pub fn invoke_signed(&self, signers_seeds: &[&[&[u8]]]) -> solana_program::entrypoint::ProgramResult {
          let args = SetDataInstructionArgs {
                                                              data: self.instruction.data.clone().expect("data is not set"),
                                    };
        let instruction = SetDataCpi {
        __program: self.instruction.__program,
                  
          authority: self.instruction.authority.expect("authority is not set"),
                  
          pda: self.instruction.pda.expect("pda is not set"),
                          __args: args,
            };
    instruction.invoke_signed_with_remaining_accounts(signers_seeds, &self.instruction.__remaining_accounts)
  }
}

#[derive(Clone, Debug)]
struct SetDataCpiBuilderInstruction<'a, 'b> {
  __program: &'b solana_program::account_info::AccountInfo<'a>,
            authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                pda: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                        data: Option<u32>,
        /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
  __remaining_accounts: Vec<(&'b solana_program::account_info::AccountInfo<'a>, bool, bool)>,
}
