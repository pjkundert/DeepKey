use hdk::prelude::*;
use crate::error::Error;
use crate::keyset_root::entry::KeysetRoot;
use crate::device_authorization::device_invite_acceptance::entry::DeviceInviteAcceptance;
use crate::validate_classic::*;

// The joining proof is added to the chain before init.
const JOINING_PROOF_CHAIN_INDEX: u32 = 2;

// @todo - e.g. configurable difficulty over hashing the DNA - https://docs.rs/pow/0.2.0/pow/
#[derive(Debug, Serialize, Deserialize)]
pub struct ProofOfWork([u8; 32]);

// @todo
#[derive(Debug, Serialize, Deserialize)]
pub struct ProofOfStake([u8; 32]);

// @todo
#[derive(Debug, Serialize, Deserialize)]
pub struct ProofOfAuthority([u8; 32]);

#[derive(Debug, Serialize, Deserialize)]
enum MembraneProof {
    // No additional membrane.
    None,
    // Proof of Work membrane.
    ProofOfWork(ProofOfWork),
    // Proof of Stake membrane.
    ProofOfStake(ProofOfStake),
    // Proof of Authority membrane.
    ProofOfAuthority(ProofOfAuthority),
}

#[derive(Debug, Serialize, Deserialize)]
enum KeysetProof {
    KeysetRoot(KeysetRoot),
    DeviceInviteAcceptance(DeviceInviteAcceptance),
}

//#[hdk_entry(id = "joining_proof")]
#[hdk_entry_helper]
pub struct JoiningProof {
    keyset_proof: KeysetProof,
    membrane_proof: MembraneProof,
}
/* 
 * TODO: How do we allow all CRUD?
 * 
impl TryFrom<&Record> for JoiningProof {
    type Error = crate::error::Error;
    fn try_from(element: &Record) -> Result<Self, Self::Error> {
        match element.action() {
            // Only
            Action::Create(_) | Action::Update(_) | Action::Delete(_) => {
                Ok(match element.entry() {
                    RecordEntry::Present(serialized) => match Self::try_from(serialized) {
                        Ok(deserialized) => deserialized,
                        Err(e) => return Err(crate::error::Error::Wasm(e)),
                    }
                    __ => return Err(crate::error::Error::EntryMissing),
                })
            },
            _ => Err(crate::error::Error::WrongHeader),
        }

    }
}
 */

#[hdk_extern]
fn validate_create_entry_joining_proof(validate_data: ValidateData) -> ExternResult<ValidateCallbackResult> {
    match JoiningProof::try_from(&validate_data.element) {
        Ok(_) => if validate_data.element.action().action_seq() == JOINING_PROOF_CHAIN_INDEX {
            Ok(ValidateCallbackResult::Valid)
        } else {
            Error::JoiningProofPosition.into()
        },
        Err(e) => Ok(ValidateCallbackResult::Invalid(e.to_string())),
    }
}

#[hdk_extern]
fn validate_update_entry_joining_proof(_validate_data: ValidateData) -> ExternResult<ValidateCallbackResult> {
    Error::UpdateJoiningProof.into()
}

#[hdk_extern]
fn validate_delete_entry_joining_proof(_validate_data: ValidateData) -> ExternResult<ValidateCallbackResult> {
    Error::DeleteJoiningProof.into()
}
