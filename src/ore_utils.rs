use drillx_2::Solution;
use ore_api::{
  consts::{PROOF, BUS_ADDRESSES},
  instruction,
  ID as ORE_ID,
};
use coal_api::instruction::mine_ore;
use solana_sdk::{
  pubkey::Pubkey,
  instruction::Instruction,
};


pub fn get_ore_auth_ix(signer: Pubkey) -> Instruction {
  let proof = ore_proof_pubkey(signer);
  instruction::auth(proof)
}

pub fn get_ore_mine_ix(signer: Pubkey, solution: Solution, bus: usize) -> Instruction {
  mine_ore(signer, signer, BUS_ADDRESSES[bus], solution)
}

pub fn get_ore_register_ix(signer: Pubkey) -> Instruction {
  instruction::open(signer, signer, signer)
}

pub fn ore_proof_pubkey(authority: Pubkey) -> Pubkey {
  Pubkey::find_program_address(&[PROOF, authority.as_ref()], &ORE_ID).0
}