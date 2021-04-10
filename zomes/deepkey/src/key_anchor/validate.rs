use hdk::prelude::*;

fn _validate_key_revocation(revoked_key_anchor: &KeyAnchor, key_revocation: &KeyRevocation) -> ExternResult<ValidateCallbackResult> {

}

fn _validate_key_generation(proposed_key_anchor: &KeyAnchor, key_generation: &KeyGeneration) -> ExternResult<ValidateCallbackResult> {
    if key_generation.new_key.get_raw_32() == proposed_key_anchor.as_ref() {
        Ok(ValidateCallbackResult::Valid)
    }
    else {
        Error::RegistrationWrongKey.into()
    }
}

#[hdk_extern]
/// All we care about is that the previous element created registration of this key.
fn validate_create_entry_key_anchor(validate_data: ValidateData) -> ExternResult<ValidateCallbackResult> {
    let proposed_key_anchor = match KeyAnchor::try_from(&validate_data.element) {
        Ok(proposed_key_anchor) => proposed_key_anchor,
        Err(e) => return Ok(ValidateCallbackResult::Invalid(e.to_string())),
    };

    match validate_data.element.header().prev_header() {
        Some(prev_header) => match resolve_dependency::<KeyRegistration>(prev_header.clone().into())? {
            Ok(ResolvedDependency(key_registration_element, key_registration)) => match key_registration_element.header() {
                Header::Create(_) => match key_registration {
                    KeyRegistration::Create(key_generation) => _validate_key_generation(&proposed_key_anchor, &key_generation),
                    _ => Error::RegistrationWrongOp.into(),
                },
                _ => Error::RegistrationWrongHeader.into(),
            },
            Err(validate_callback_result) => return Ok(validate_callback_result),
        },
        None => Error::RegistrationNone.into(),
    }
}

#[hdk_extern]
/// All we care about is that the previous element updated registration of this key (but not a delete-update).
fn validate_update_entry_key_anchor(validate_data: ValidateData) -> ExternResult<ValidateCallbackResult> {
    let proposed_key_anchor = match KeyAnchor::try_from(&validate_data.element) {
        Ok(proposed_key_anchor) => proposed_key_anchor,
        Err(e) => return Ok(ValidateCallbackResult::Invalid(e.to_string())),
    };

    match validate_data.element.header().prev_header() {
        Some(prev_header) => match resolve_dependency::<KeyRegistration>(prev_header.clone().into())? {
            Ok(ResolvedDependency(key_registration_element, key_registration)) => match key_registration_element.header() {
                Header::Update(_) => match key_registration {
                    KeyRegistration::Update(key_revocation, key_generation) => {
                        match _validate_key_generation(&proposed_key_anchor, &key_generation) {
                            Ok(ValidateCallbackResult::Valid) => { },
                            validate_callback_result => return validate_callback_result,
                        }

                        match validate_data.element.header() {
                            Update(key_anchor_update_header) => match resolve_dependency::<KeyAnchor>(key_anchor_update_header.original_header_address.clone())? {
                                Some(ResolvedDependency(_, updated_key_anchor)) => _validate_key_revocation(&updated_key_anchor, &key_revocation),
                                Err(validate_callback_result) => Ok(validate_callback_result),
                            },
                            _ => Error::RegistrationWrongHeader.into(),
                        }
                    },
                    _ => Error::RegistrationWrongOp.into(),
                },
                _ => Error::RegistrationWrongHeader.into(),
            },
            Err(validate_callback_result) => return Ok(validate_callback_result),
        },
        None => Error::RegistrationNone.into(),
    }
}

#[hdk_extern]
/// All we care about is that the previous element deleted (revoked) the right key.
fn validate_delete_entry_key_anchor(validate_data: ValidateData) -> ExternResult<ValidateCallbackResult> {
    let prev_element: Element = match validate_data.element.header().prev_header() {
        Some(prev_header) => match get(prev_header.clone().into(), GetOptions::content())? {
            Some(element) => prev_element,
            None => return Ok(ValidateCallbackResult::UnresolvedDependencies(vec![prev_header.clone().into()])),
        },
        None => Error::RegistrationNone.into(),
    };

    match prev_element.header() {
        Header::Delete(delete_header) => {
            match resolve_dependency::<KeyRegistration>(delete_header.deletes_address.clone().into())? {
                Ok(ResolvedDependency(key_registration_element, key_registration)) => match key_registration_element.header() {
                    Header::Update(_) => match key_registration {
                        KeyRegistration::Delete(key_revocation) => {
                            _validate_key_revocation()
                        },
                        _ => Error::RegistrationWrongOp.into(),
                    },
                    _ => Error::RegistrationWrongHeader.into()
                },
                Err(validate_callback_result) => return Ok(validate_callback_result),
            },
        },
        _ => Error::RegistrationWrongHeader.into(),
    }
}