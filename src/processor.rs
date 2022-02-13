use {
    crate::{
        // error::VaultError,
        instruction::VaultInstruction,
        state::{
            Vault, VaultState,PREFIX,ERC20,Address
           
        },
    
    },
    borsh::{BorshDeserialize, BorshSerialize},
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint::ProgramResult,
        msg,
        program_option::COption,
        pubkey::Pubkey,
        rent::Rent,
        sysvar::Sysvar,
    },
    // spl_token::state::{Account, Mint},
};




pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    input: &[u8],
) -> ProgramResult {
    let instruction = VaultInstruction::try_from_slice(input)?;
    match instruction {
        VaultInstruction::InitVault(args) => {
            msg!("Instruction: Init Vault");
            process_init_vault(program_id, accounts, args.id,args.EthAddress.as_str())
        },
        _ => todo!(),

        
        // VaultInstruction::AddTokenToInactiveVault(args) => {
        //     msg!("Instruction: Add token to vault");
        //     process_add_token_to_inactivated_vault(program_id, accounts, args.amount)
        // }
      
    }
}

                    

pub fn process_init_vault(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    id:i32,
    EthERC20:&str
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let fraction_mint_info = next_account_info(account_info_iter)?;
    let redeem_treasury_info = next_account_info(account_info_iter)?;
    let fraction_treasury_info = next_account_info(account_info_iter)?;
    let vault_info = next_account_info(account_info_iter)?;
    // let authority_info = next_account_info(account_info_iter)?;
    // let pricing_lookup_address = next_account_info(account_info_iter)?;
    // let token_program_info = next_account_info(account_info_iter)?;
    // let rent = &Rent::from_account_info(next_account_info(account_info_iter)?)?;
    let mut vault = Vault::from_account_info(vault_info)?;
      
       
  
    let seeds = &[
        PREFIX.as_bytes(),
        program_id.as_ref(),
        vault_info.key.as_ref(),
    ];
    let (authority, _) = Pubkey::find_program_address(seeds, program_id);
    
  //PDA => program derivalble account   
  // authority => public key 

    // vault.token_program = *token_program_info.key;
    // vault.redeem_treasury = *redeem_treasury_info.key;
    // vault.fraction_treasury = *fraction_treasury_info.key;
    // vault.fraction_mint = *fraction_mint_info.key;
    // vault.pricing_lookup_address = *pricing_lookup_address.key;
    // vault.allow_further_share_creation = allow_further_share_creation;
    // vault.Owner = *authority_info.key;
    // vault.token_type_count = 0;
    // vault.state = VaultState::Inactive;
     vault.Erc20 =  ERC20{
            EthERC:Address::EthAddress(EthERC20.to_owned()),
            SolERC:Address::SolAddress(authority)
     } ;

     vault.amount = 0;  
     vault.lock = true ;
     vault.limit = 1000; // tokens 
     vault.serialize(&mut *vault_info.data.borrow_mut())?;
    // let fraction_mint: Mint = assert_initialized(fraction_mint_info)?;
    // let redeem_treasury: Account = assert_initialized(redeem_treasury_info)?;
    // let fraction_treasury: Account = assert_initialized(fraction_treasury_info)?;
    // let mut vault = Vault::from_account_info(vault_info)?;
 

    Ok(())
}




//erc 20 vault  


