use cosmwasm_schema::cw_serde;
use crate::tasks::create_account::AccountCreationProof;

#[cw_serde]
pub enum ExecuteMsg {
    AccountCreation(AccountCreationProof)
}