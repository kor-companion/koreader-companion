use kc_domain::{
    DomainEvent, LogAttribution, LogSeverity, OperationLogEntry, PersistenceStore,
    StoredOperationLog, VerificationStatus,
};

use crate::sqlite::store::{unix_now, SqliteStore};

impl PersistenceStore for SqliteStore {
    fn record_event(
        &mut self,
        attribution: &LogAttribution,
        event: &DomainEvent,
    ) -> Result<(), kc_domain::DomainError> {
        let (severity, message) = describe_event(event);
        self.append_log_internal(&StoredOperationLog {
            entry: OperationLogEntry::new(attribution.clone(), severity, message),
            recorded_at_unix: unix_now(),
        })
        .map_err(Into::into)
    }
}

fn describe_event(event: &DomainEvent) -> (LogSeverity, String) {
    match event {
        DomainEvent::WorkflowPlanned(snapshot) => (
            LogSeverity::Info,
            format!("workflow planned in {:?} mode", snapshot.mode),
        ),
        DomainEvent::ConfirmationRequired(gate, _) => (
            LogSeverity::Warning,
            format!("confirmation required: {}", gate.message),
        ),
        DomainEvent::PhaseChanged(phase, _) => (
            phase_severity(*phase),
            format!("workflow phase changed to {phase:?}"),
        ),
        DomainEvent::ProgressUpdated(update, _) => (
            LogSeverity::Info,
            update.message.clone().unwrap_or_else(|| {
                format!("progress {}/{}", update.completed_items, update.total_items)
            }),
        ),
        DomainEvent::VerificationReported(report, _) => {
            let summary = report
                .items
                .first()
                .and_then(|item| item.message.clone())
                .unwrap_or_else(|| "verification report recorded".to_string());
            (verification_severity(report.status), summary)
        }
        DomainEvent::PlanItemStarted(id, _) => (
            LogSeverity::Info,
            format!("plan item {} started", id.value()),
        ),
        DomainEvent::PlanItemCompleted(id, _) => (
            LogSeverity::Info,
            format!("plan item {} completed", id.value()),
        ),
        DomainEvent::WorkflowFinished(snapshot) => (
            phase_severity(snapshot.phase),
            format!("workflow finished in {:?}", snapshot.phase),
        ),
    }
}

fn phase_severity(phase: kc_domain::WorkflowPhase) -> LogSeverity {
    match phase {
        kc_domain::WorkflowPhase::Failed => LogSeverity::Error,
        kc_domain::WorkflowPhase::Cancelled | kc_domain::WorkflowPhase::AwaitingConfirmation => {
            LogSeverity::Warning
        }
        kc_domain::WorkflowPhase::Planned
        | kc_domain::WorkflowPhase::Ready
        | kc_domain::WorkflowPhase::Running
        | kc_domain::WorkflowPhase::Succeeded => LogSeverity::Info,
    }
}

fn verification_severity(status: VerificationStatus) -> LogSeverity {
    match status {
        VerificationStatus::Failed => LogSeverity::Error,
        VerificationStatus::Warning => LogSeverity::Warning,
        VerificationStatus::Pending | VerificationStatus::Passed => LogSeverity::Info,
    }
}
