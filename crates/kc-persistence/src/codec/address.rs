use std::path::PathBuf;

use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use kc_domain::Address;

use crate::codec::enums::{encode_transport_kind, parse_transport_kind};
use crate::PersistenceError;

pub fn encode_address(address: &Address) -> String {
    match address {
        Address::LocalPath(path) => format!("local|{}", encode_path_component(path)),
        Address::ScopedPath {
            transport,
            scope,
            relative_path,
        } => format!(
            "scoped|{}|{}|{}",
            encode_transport_kind(transport),
            encode_component(scope),
            encode_path_component(relative_path)
        ),
        Address::Remote {
            transport,
            locator,
            path,
        } => format!(
            "remote|{}|{}|{}",
            encode_transport_kind(transport),
            encode_component(locator),
            encode_component(path)
        ),
        Address::Logical { scheme, value } => format!(
            "logical|{}|{}",
            encode_component(scheme),
            encode_component(value)
        ),
    }
}

pub fn parse_address(field: &'static str, value: &str) -> Result<Address, PersistenceError> {
    let parts = value.split('|').collect::<Vec<_>>();
    match parts.as_slice() {
        ["local", path] => Ok(Address::filesystem(decode_path_component(field, path)?)),
        ["scoped", transport, scope, relative_path] => Address::scoped(
            parse_transport_kind(transport)?,
            decode_component(field, scope)?,
            decode_path_component(field, relative_path)?,
        )
        .map_err(|_| PersistenceError::InvalidAddress {
            field,
            value: value.to_string(),
        }),
        ["remote", transport, locator, path] => Address::remote(
            parse_transport_kind(transport)?,
            decode_component(field, locator)?,
            decode_component(field, path)?,
        )
        .map_err(|_| PersistenceError::InvalidAddress {
            field,
            value: value.to_string(),
        }),
        ["logical", scheme, logical_value] => Address::logical(
            decode_component(field, scheme)?,
            decode_component(field, logical_value)?,
        )
        .map_err(|_| PersistenceError::InvalidAddress {
            field,
            value: value.to_string(),
        }),
        _ => Err(PersistenceError::InvalidAddress {
            field,
            value: value.to_string(),
        }),
    }
}

pub fn encode_component(value: &str) -> String {
    URL_SAFE_NO_PAD.encode(value.as_bytes())
}

pub fn decode_component(field: &'static str, value: &str) -> Result<String, PersistenceError> {
    let bytes = URL_SAFE_NO_PAD
        .decode(value)
        .map_err(|_| PersistenceError::InvalidEncoding {
            field,
            value: value.to_string(),
        })?;
    String::from_utf8(bytes).map_err(|_| PersistenceError::InvalidEncoding {
        field,
        value: value.to_string(),
    })
}

pub fn encode_path_component(path: &std::path::Path) -> String {
    URL_SAFE_NO_PAD.encode(path_to_bytes(path))
}

pub fn decode_path_component(
    field: &'static str,
    value: &str,
) -> Result<PathBuf, PersistenceError> {
    let bytes = URL_SAFE_NO_PAD
        .decode(value)
        .map_err(|_| PersistenceError::InvalidEncoding {
            field,
            value: value.to_string(),
        })?;
    bytes_to_path(field, value, bytes)
}

#[cfg(unix)]
fn path_to_bytes(path: &std::path::Path) -> Vec<u8> {
    use std::os::unix::ffi::OsStrExt;

    path.as_os_str().as_bytes().to_vec()
}

#[cfg(not(unix))]
fn path_to_bytes(path: &std::path::Path) -> Vec<u8> {
    path.as_os_str().to_string_lossy().into_owned().into_bytes()
}

#[cfg(unix)]
fn bytes_to_path(
    _field: &'static str,
    _value: &str,
    bytes: Vec<u8>,
) -> Result<PathBuf, PersistenceError> {
    use std::ffi::OsString;
    use std::os::unix::ffi::OsStringExt;

    Ok(PathBuf::from(OsString::from_vec(bytes)))
}

#[cfg(not(unix))]
fn bytes_to_path(
    field: &'static str,
    value: &str,
    bytes: Vec<u8>,
) -> Result<PathBuf, PersistenceError> {
    String::from_utf8(bytes)
        .map(PathBuf::from)
        .map_err(|_| PersistenceError::InvalidEncoding {
            field,
            value: value.to_string(),
        })
}
