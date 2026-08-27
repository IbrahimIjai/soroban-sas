use crate::errors::SASError;
use soroban_sdk::{Address, Env, String};

const MAX_SCHEMA_LENGTH: u32 = 1024;
const ZERO_ACCOUNT_STRKEY: &str = "GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF";
const ZERO_CONTRACT_STRKEY: &str = "CAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABSC4";

fn is_valid_identifier(value: &str) -> bool {
    let mut chars = value.chars();
    match chars.next() {
        Some(first) if first.is_ascii_alphabetic() || first == '_' => {}
        _ => return false,
    }

    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

fn is_valid_type(value: &str) -> bool {
    let value = value.trim();
    !value.is_empty()
        && value.chars().any(|c| c.is_ascii_alphabetic())
        && value.chars().all(|c| {
            c.is_ascii_alphanumeric()
                || matches!(c, '_' | '<' | '>' | '[' | ']' | ',' | ':' | '(' | ')' | ' ' | '?')
        })
}

pub fn validate_schema_syntax(_env: &Env, schema: &String) -> Result<(), SASError> {
    let schema = schema.trim();
    if schema.is_empty() {
        return Err(SASError::InvalidSchema);
    }
    if schema.len() > MAX_SCHEMA_LENGTH {
        return Err(SASError::InvalidSchema);
    }

    let mut field_count = 0u32;
    for field in schema.split(',') {
        let field = field.trim();
        if field.is_empty() {
            return Err(SASError::InvalidSchema);
        }

        let Some(split_index) = field.find(char::is_whitespace) else {
            return Err(SASError::InvalidSchema);
        };
        let (name, ty) = field.split_at(split_index);

        if !is_valid_identifier(name.trim()) || !is_valid_type(ty) {
            return Err(SASError::InvalidSchema);
        }
        field_count += 1;
    }

    if field_count == 0 {
        return Err(SASError::InvalidSchema);
    }

    Ok(())
}

pub fn validate_ttl(_env: &Env, current_time: u64, expiration_time: u64) -> Result<(), SASError> {
    if expiration_time > 0 && current_time >= expiration_time {
        return Err(SASError::InvalidTTL);
    }
    Ok(())
}

pub fn validate_recipient(_env: &Env, recipient: &Address) -> Result<(), SASError> {
    let recipient = recipient.to_string();
    if recipient == ZERO_ACCOUNT_STRKEY || recipient == ZERO_CONTRACT_STRKEY {
        return Err(SASError::InvalidRecipient);
    }
    Ok(())
}

pub fn check_revocable(
    _env: &Env,
    schema_revocable: bool,
    attestation_revocable: bool,
) -> Result<(), SASError> {
    if !schema_revocable && attestation_revocable {
        return Err(SASError::NotRevocable);
    }
    Ok(())
}
