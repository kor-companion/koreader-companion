use crate::{DomainError, ExecutionId, LogSeverity, OperationLogEntry};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoredOperationLog {
    pub entry: OperationLogEntry,
    pub recorded_at_unix: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct OperationLogQuery {
    pub execution_id: Option<ExecutionId>,
    pub minimum_severity: Option<LogSeverity>,
}

impl OperationLogQuery {
    pub fn all() -> Self {
        Self::default()
    }
}

pub trait OperationLogRepository {
    fn append_log(&mut self, entry: &StoredOperationLog) -> Result<(), DomainError>;
    fn list_logs(&self, query: &OperationLogQuery) -> Result<Vec<StoredOperationLog>, DomainError>;
}
