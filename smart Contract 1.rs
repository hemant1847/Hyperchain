use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, pubkey::Pubkey,
    program_error::ProgramError, program_pack::Pack, system_instruction,
};
use std::str::from_utf8;

// Define the structure of the medicine account data.
// This will hold the medicine data, such as the unique ID, name, and manufacturer information.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Medicine {
    pub unique_id: String,
    pub medicine_name: String,
    pub brand_name: String,
    pub manufacturer: String,
    pub manufacturer_address: String,
    pub manufactured_on: String,
    pub expiry_date: String,
}

impl Medicine {
    // Create a new medicine account.
    pub fn new(
        unique_id: String,
        medicine_name: String,
        brand_name: String,
        manufacturer: String,
        manufacturer_address: String,
        manufactured_on: String,
        expiry_date: String,
    ) -> Self {
        Medicine {
            unique_id,
            medicine_name,
            brand_name,
            manufacturer,
            manufacturer_address,
            manufactured_on,
            expiry_date,
        }
    }
}

// Define the program ID for the smart contract.
// This is the public key of the account that contains the smart contract code.
pub fn program_id() -> Pubkey {
    solana_program::declare_id!("YOUR_PROGRAM_ID_HERE")
}

// Define the entry point for the smart contract.
entrypoint!(process_instruction);

// Define the processing logic for the smart contract.
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // Ensure that the program ID is correct.
    if program_id != &program_id() {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Decode the instruction data into a Medicine struct.
    let medicine_data = Medicine::unpack(instruction_data)?;

    // Ensure that the instruction data is valid.
    if medicine_data.unique_id.is_empty() {
        return Err(ProgramError::InvalidInstructionData);
    }

    // Get the account that will hold the medicine data.
    let accounts_iter = &mut accounts.iter();
    let medicine_account = next_account_info(accounts_iter)?;

    // Ensure that the medicine account is owned by the program.
    if medicine_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Unpack the existing medicine data from the medicine account.
    let mut existing_medicine_data = Medicine::unpack_unchecked(&medicine_account.data.borrow())?;

    // Ensure that the medicine account has not already been initialized.
    if existing_medicine_data.unique_id != "" {
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    // Set the medicine data in the medicine account.
    existing_medicine_data.unique_id = medicine_data.unique_id;
    existing_medicine_data.medicine_name = medicine_data.medicine_name;
    existing_medicine_data.brand_name = medicine_data.brand_name;
    existing_medicine_data.manufacturer = medicine_data.manufacturer;
    existing_medicine_data.manufacturer_address = medicine_data.manufacturer_address;
    existing_medicine_data.manufactured_on = medicine_data.manufactured_on;
    existing_medicine_data.expiry_date = medicine_data.expiry_date;

    // Pack the medicine data and update the medicine account.
    Medicine::pack(existing_medicine_data, &mut medicine_account.data.borrow_mut())?;

    Ok(())
