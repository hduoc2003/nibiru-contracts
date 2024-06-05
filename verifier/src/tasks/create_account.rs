use crate::error::ContractError;
use crate::state::{BASE_PROOF, EXTRA_PROOF};
use crate::types::Groth16Proof;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, Uint256};
use crate::tasks::Tasks;
use crate::utils::verify_proof;

#[cw_serde]
pub struct AccountCreationProof {
    pub relayer_hash: Uint256,
    pub email_addr_pointer: Uint256,
    pub account_key_commit: Uint256,
    pub wallet_salt: Uint256,
    pub psi_point: [Uint256; 2],
    pub proof: Groth16Proof,
}

pub fn verify_account_creation(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: AccountCreationProof,
) -> Result<Response, ContractError> {
    let pub_signals = vec![
        msg.relayer_hash,
        msg.email_addr_pointer,
        msg.account_key_commit,
        msg.wallet_salt,
        msg.psi_point[0],
        msg.psi_point[1]
    ];
    verify_proof(&BASE_PROOF.load(deps.storage)?, &EXTRA_PROOF.load(deps.storage, Tasks::CREATE_ACCOUNT)?, &msg.proof, &pub_signals)?;
    Ok(Response::default())
}
