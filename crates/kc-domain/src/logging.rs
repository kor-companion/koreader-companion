use std::path::PathBuf;

use crate::{
    workflow::{ExecutionId, OperationId, PlanId, PlanItemId},
    Address,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OperationTarget {
    HostPath(PathBuf),
    DevicePath(PathBuf),
    Address(Address),
    Payload(String),
    Logical(String),
}

impl OperationTarget {
    pub fn from_address(address: Address) -> Self {
        Self::Address(address)
    }

    pub fn primary_address(&self) -> Option<Address> {
        match self {
            Self::HostPath(path) | Self::DevicePath(path) => {
                Some(Address::filesystem(path.clone()))
            }
            Self::Address(address) => Some(address.clone()),
            Self::Payload(_) | Self::Logical(_) => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogAttribution {
    pub plan_id: PlanId,
    pub plan_item_id: PlanItemId,
    pub execution_id: ExecutionId,
    pub operation_id: OperationId,
    pub target: OperationTarget,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogSeverity {
    Info,
    Warning,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperationLogEntry {
    pub attribution: LogAttribution,
    pub severity: LogSeverity,
    pub message: String,
}

impl OperationLogEntry {
    pub fn new(
        attribution: LogAttribution,
        severity: LogSeverity,
        message: impl Into<String>,
    ) -> Self {
        Self {
            attribution,
            severity,
            message: message.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::{LogAttribution, LogSeverity, OperationLogEntry, OperationTarget};
    use crate::{ExecutionId, OperationId, PlanId, PlanItemId};

    #[test]
    fn log_attribution_stays_tied_to_plan_items() {
        let attribution = LogAttribution {
            plan_id: PlanId::new(3),
            plan_item_id: PlanItemId::new(11),
            execution_id: ExecutionId::new(9),
            operation_id: OperationId::new(15),
            target: OperationTarget::DevicePath(PathBuf::from("/.kobo/KoboRoot.tgz")),
        };
        let entry =
            OperationLogEntry::new(attribution.clone(), LogSeverity::Info, "write scheduled");

        assert_eq!(attribution.plan_id, PlanId::new(3));
        assert_eq!(attribution.plan_item_id, PlanItemId::new(11));
        assert_eq!(attribution.execution_id, ExecutionId::new(9));
        assert_eq!(
            entry.attribution.target,
            OperationTarget::DevicePath(PathBuf::from("/.kobo/KoboRoot.tgz"))
        );
    }
}
