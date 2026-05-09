use std::path::PathBuf;

use super::WorkflowExecution;
use crate::{
    ConfirmationId, ConfirmationKind, DomainError, ExecutionMode, OperationTarget, PlanId,
    PlanItem, PlanItemId, PlanItemKind, WorkflowKind, WorkflowPhase, WorkflowPlan,
};

#[test]
fn workflow_execution_advances_through_guarded_states() {
    let plan = WorkflowPlan::new(
        PlanId::new(7),
        WorkflowKind::Install,
        ExecutionMode::Guarded,
        vec![
            PlanItem::new(
                PlanItemId::new(1),
                PlanItemKind::Backup,
                "Back up launcher state",
                OperationTarget::DevicePath(PathBuf::from("/.kobo")),
            ),
            PlanItem::new(
                PlanItemId::new(2),
                PlanItemKind::Write,
                "Write KOReader payload",
                OperationTarget::DevicePath(PathBuf::from("/.adds/koreader")),
            )
            .destructive(true)
            .requires_confirmation(ConfirmationKind::ExternalDeviceMutation),
        ],
    )
    .unwrap();

    let mut execution = WorkflowExecution::new(plan);
    assert_eq!(execution.phase(), WorkflowPhase::AwaitingConfirmation);

    execution
        .approve_confirmation(ConfirmationId::new(2))
        .unwrap();
    assert_eq!(execution.phase(), WorkflowPhase::Ready);

    execution.start().unwrap();
    execution.begin_item(PlanItemId::new(1)).unwrap();
    assert_eq!(
        execution.complete_active_item().unwrap(),
        PlanItemId::new(1)
    );
    execution.begin_item(PlanItemId::new(2)).unwrap();
    assert_eq!(
        execution.complete_active_item().unwrap(),
        PlanItemId::new(2)
    );
    execution.finish().unwrap();

    let snapshot = execution.snapshot();
    assert_eq!(snapshot.phase, WorkflowPhase::Succeeded);
    assert_eq!(snapshot.completed_items, 2);
}

#[test]
fn terminal_workflow_phases_cannot_be_overwritten() {
    let plan = WorkflowPlan::new(
        PlanId::new(9),
        WorkflowKind::Verify,
        ExecutionMode::DryRun,
        vec![PlanItem::new(
            PlanItemId::new(1),
            PlanItemKind::Validate,
            "Verify staged payload",
            OperationTarget::Logical("payload".to_string()),
        )],
    )
    .unwrap();

    let mut execution = WorkflowExecution::new(plan);
    execution.start().unwrap();
    execution.begin_item(PlanItemId::new(1)).unwrap();
    execution.complete_active_item().unwrap();
    execution.finish().unwrap();

    let error = execution.fail("should stay succeeded").unwrap_err();
    assert_eq!(
        error,
        DomainError::InvalidWorkflowTransition {
            from: WorkflowPhase::Succeeded,
            to: WorkflowPhase::Failed,
        }
    );
}
