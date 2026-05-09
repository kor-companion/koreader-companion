use kc_domain::{DeviceRecordRepository, KnownDeviceRecord};
use rusqlite::{params, OptionalExtension};

use crate::codec::address::{encode_address, parse_address};
use crate::codec::enums::{
    encode_device_kind, encode_support_level, parse_device_kind, parse_support_level,
};
use crate::sqlite::store::SqliteStore;
use crate::PersistenceError;

impl SqliteStore {
    fn upsert_device_internal(
        &mut self,
        record: &KnownDeviceRecord,
    ) -> Result<(), PersistenceError> {
        self.conn.execute(
            "INSERT INTO devices (
                id, kind, display_name, support_level, last_seen_at_unix, last_host_id, last_address
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
             ON CONFLICT(id) DO UPDATE SET
                 kind = excluded.kind,
                 display_name = excluded.display_name,
                 support_level = excluded.support_level,
                 last_seen_at_unix = excluded.last_seen_at_unix,
                 last_host_id = excluded.last_host_id,
                 last_address = excluded.last_address",
            params![
                &record.descriptor.id,
                encode_device_kind(&record.descriptor.kind),
                &record.descriptor.display_name,
                encode_support_level(record.descriptor.support_level),
                record.last_seen_at_unix,
                &record.last_host_id,
                record.last_address.as_ref().map(encode_address),
            ],
        )?;
        Ok(())
    }

    pub(crate) fn get_device_internal(
        &self,
        id: &str,
    ) -> Result<Option<KnownDeviceRecord>, PersistenceError> {
        self.conn
            .query_row(
                "SELECT id, kind, display_name, support_level, last_seen_at_unix, last_host_id, last_address
                 FROM devices WHERE id = ?1",
                [id],
                |row| {
                    Ok((
                        row.get::<_, String>(0)?,
                        row.get::<_, String>(1)?,
                        row.get::<_, String>(2)?,
                        row.get::<_, String>(3)?,
                        row.get::<_, i64>(4)?,
                        row.get::<_, Option<String>>(5)?,
                        row.get::<_, Option<String>>(6)?,
                    ))
                },
            )
            .optional()?
            .map(
                |(id, kind, display_name, support_level, last_seen_at_unix, last_host_id, last_address)| {
                    Ok(KnownDeviceRecord {
                        descriptor: kc_domain::DeviceDescriptor {
                            id,
                            kind: parse_device_kind(&kind)?,
                            display_name,
                            support_level: parse_support_level(&support_level)?,
                        },
                        last_seen_at_unix,
                        last_host_id,
                        last_address: last_address
                            .as_deref()
                            .map(|value| parse_address("device.last_address", value))
                            .transpose()?,
                    })
                },
            )
            .transpose()
    }
}

impl DeviceRecordRepository for SqliteStore {
    fn upsert_device_record(
        &mut self,
        record: &KnownDeviceRecord,
    ) -> Result<(), kc_domain::DomainError> {
        self.upsert_device_internal(record).map_err(Into::into)
    }

    fn get_device_record(
        &self,
        id: &str,
    ) -> Result<Option<KnownDeviceRecord>, kc_domain::DomainError> {
        self.get_device_internal(id).map_err(Into::into)
    }
}
