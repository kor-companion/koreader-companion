use kc_domain::OperationTarget;

use crate::codec::address::{
    decode_component, decode_path_component, encode_address, encode_component,
    encode_path_component, parse_address,
};
use crate::PersistenceError;

pub fn encode_target(target: &OperationTarget) -> (String, String) {
    match target {
        OperationTarget::HostPath(path) => ("host_path".to_string(), encode_path_component(path)),
        OperationTarget::DevicePath(path) => {
            ("device_path".to_string(), encode_path_component(path))
        }
        OperationTarget::Address(address) => ("address".to_string(), encode_address(address)),
        OperationTarget::Payload(value) => ("payload".to_string(), encode_component(value)),
        OperationTarget::Logical(value) => ("logical".to_string(), encode_component(value)),
    }
}

pub fn parse_target(kind: &str, value: &str) -> Result<OperationTarget, PersistenceError> {
    match kind {
        "host_path" => Ok(OperationTarget::HostPath(decode_path_component(
            "target_value",
            value,
        )?)),
        "device_path" => Ok(OperationTarget::DevicePath(decode_path_component(
            "target_value",
            value,
        )?)),
        "address" => parse_address("target_value", value)
            .map(OperationTarget::Address)
            .map_err(|_| PersistenceError::InvalidTarget {
                field: "target_value",
                value: value.to_string(),
            }),
        "payload" => Ok(OperationTarget::Payload(decode_component(
            "target_value",
            value,
        )?)),
        "logical" => Ok(OperationTarget::Logical(decode_component(
            "target_value",
            value,
        )?)),
        _ => Err(PersistenceError::InvalidEnum {
            field: "target_kind",
            value: kind.to_string(),
        }),
    }
}
