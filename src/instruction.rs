use {
   
    borsh::{BorshDeserialize, BorshSerialize},
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        sysvar,
        borsh::try_from_slice_unchecked // serilization deserilization
    },
};
 
// => serlization vault binary 
// => deri
  
use solana_program::program_error::ProgramError;
use std::convert::TryInto;


/// spl => erc 
/// AMM, dspefi spl 
/// erc 20
 



#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, Clone)]
pub struct VaultID {
    pub id: i32,
    pub EthAddress:String
}



/// Instructions supported by the Fraction program.
#[derive(BorshSerialize, BorshDeserialize, Clone)]
pub enum VaultInstruction{

    InitVault(VaultID),
    AddTokenToVault


}





#[allow(clippy::too_many_arguments)]
pub fn try_from_slice_checked<T: BorshDeserialize>(
    data: &[u8],

    data_size: usize
) -> Result<T, ProgramError> {
    // if (data[0] != data_type as u8 && data[0] != Key::Uninitialized as u8)
    //     || data.len() != data_size
    // {
    //     return Err(VaultError::DataTypeMismatch.into());
    // }

    let result: T = try_from_slice_unchecked(data)?;

    Ok(result)
}



/// Creates an InitVault instruction
#[allow(clippy::too_many_arguments)]
pub fn create_init_vault_instruction(
    program_id: Pubkey,
    ERC20:&str,
    vault: Pubkey,
    vault_authority: Pubkey,
    
    
) -> Instruction {
     let mut counter =-1 ;
    Instruction {
        program_id,
        accounts: vec![
       
            AccountMeta::new(vault, false),
            AccountMeta::new_readonly(vault_authority, false),
           
 
        ],
        data: VaultInstruction::InitVault(VaultID {
            id:counter+1, // i32 0 1 2
            EthAddress:String::from("") // ox 
        })
        .try_to_vec()
        .unwrap(),
    }

}

