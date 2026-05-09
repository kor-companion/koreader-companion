use kc_domain::{
    ExecutionId, LogAttribution, OperationLogEntry, OperationLogQuery, OperationLogRepository,
    StoredOperationLog,
};
use rusqlite::params;

use crate::codec::enums::{encode_log_severity, parse_log_severity};
use crate::codec::target::{encode_target, parse_target};
use crate::sqlite::store::{from_i64, matches_minimum_severity, severity_rank, SqliteStore};
use crate::PersistenceError;

impl SqliteStore {
    pub(crate) fn append_log_internal(
        &mut self,
        entry: &StoredOperationLog,
    ) -> Result<(), PersistenceError> {
        let (target_kind, target_value) = encode_target(&entry.entry.attribution.target);
        self.conn.execute(
            "INSERT INTO operation_logs (
                plan_id, plan_item_id, execution_id, operation_id,
                target_kind, target_value, severity, message, recorded_at_unix
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                super::to_i64(entry.entry.attribution.plan_id.value())?,
                super::to_i64(entry.entry.attribution.plan_item_id.value())?,
                super::to_i64(entry.entry.attribution.execution_id.value())?,
                super::to_i64(entry.entry.attribution.operation_id.value())?,
                target_kind,
                target_value,
                encode_log_severity(entry.entry.severity),
                &entry.entry.message,
                entry.recorded_at_unix,
            ],
        )?;
        Ok(())
    }

    pub(crate) fn list_logs_internal(
        &self,
        query: &OperationLogQuery,
    ) -> Result<Vec<StoredOperationLog>, PersistenceError> {
        let minimum_rank = query.minimum_severity.map(severity_rank);
        let execution_id = query
            .execution_id
            .map(|id| super::to_i64(id.value()))
            .transpose()?;

        let mut stmt = self.conn.prepare(
            "SELECT plan_id, plan_item_id, execution_id, operation_id,
                    target_kind, target_value, severity, message, recorded_at_unix
             FROM operation_logs
             WHERE (?1 IS NULL OR execution_id = ?1)
               AND (?2 IS NULL OR
                    CASE severity
                        WHEN 'info' THEN 1
                        WHEN 'warning' THEN 2
                        WHEN 'error' THEN 3
                        ELSE 0
                    END >= ?2)
             ORDER BY id ASC",
        )?;
        let rows = stmt.query_map(
            params![execution_id, minimum_rank],
            |row| {
                Ok((
                    row.get::<_, i64>(0)?,
                    row.get::<_, i64>(1)?,
                    row.get::<_, i64>(2)?,
                    row.get::<_, i64>(3)?,
                    row.get::<_, String>(4)?,
                    row.get::<_, String>(5)?,
                    row.get::<_, String>(6)?,
                    row.get::<_, String>(7)?,
                    row.get::<_, i64>(8)?,
                ))
            },
        )?;

        let mut logs = Vec::new();
        for row in rows {
            let (
                plan_id,
                plan_item_id,
                execution_id,
                operation_id,
                target_kind,
                target_value,
                severity,
                message,
                recorded_at_unix,
            ) = row?;
            let severity = parse_log_severity(&severity)?;
            let log = StoredOperationLog {
                entry: OperationLogEntry::new(
                    LogAttribution {
                        plan_id: kc_domain::PlanId::new(from_i64(plan_id)?),
                        plan_item_id: kc_domain::PlanItemId::new(from_i64(plan_item_id)?),
                        execution_id: ExecutionId::new(from_i64(execution_id)?),
                        operation_id: kc_domain::OperationId::new(from_i64(operation_id)?),
                        target: parse_target(&target_kind, &target_value)?,
                    },
                    severity,
                    message,
                ),
                recorded_at_unix,
            };
            if matches_minimum_severity(query.minimum_severity, severity) {
                logs.push(log);
            }
        }
        Ok(logs)
    }
}

impl OperationLogRepository for SqliteStore {
    fn append_log(&mut self, entry: &StoredOperationLog) -> Result<(), kc_domain::DomainError> {
        self.append_log_internal(entry).map_err(Into::into)
    }

    fn list_logs(
        &self,
        query: &OperationLogQuery,
    ) -> Result<Vec<StoredOperationLog>, kc_domain::DomainError> {
        self.list_logs_internal(query).map_err(Into::into)
    }
}
