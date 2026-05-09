use kc_domain::{CachedReleaseMetadata, ReleaseArtifactRecord, ReleaseMetadataCache};
use rusqlite::{params, OptionalExtension};

use crate::codec::enums::{encode_release_channel, parse_release_channel};
use crate::sqlite::store::{from_i64, to_i64, SqliteStore};
use crate::PersistenceError;

impl SqliteStore {
    fn put_release_internal(
        &mut self,
        release: &CachedReleaseMetadata,
    ) -> Result<(), PersistenceError> {
        let tx = self.conn.transaction()?;
        tx.execute(
            "INSERT OR REPLACE INTO release_metadata_cache (
                cache_key, release_id, version, channel, published_at_unix, fetched_at_unix, source_url
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                &release.cache_key,
                &release.release_id,
                &release.version,
                encode_release_channel(&release.channel),
                release.published_at_unix,
                release.fetched_at_unix,
                &release.source_url,
            ],
        )?;
        tx.execute(
            "DELETE FROM release_artifacts WHERE cache_key = ?1",
            [&release.cache_key],
        )?;
        for artifact in &release.artifacts {
            tx.execute(
                "INSERT INTO release_artifacts (
                    cache_key, name, download_url, size_bytes, content_type, checksum_hex
                 ) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                params![
                    &release.cache_key,
                    &artifact.name,
                    &artifact.download_url,
                    to_i64(artifact.size_bytes)?,
                    &artifact.content_type,
                    &artifact.checksum_hex,
                ],
            )?;
        }
        tx.commit()?;
        Ok(())
    }

    fn get_release_internal(
        &self,
        cache_key: &str,
    ) -> Result<Option<CachedReleaseMetadata>, PersistenceError> {
        let release = self
            .conn
            .query_row(
                "SELECT cache_key, release_id, version, channel, published_at_unix, fetched_at_unix, source_url
                 FROM release_metadata_cache WHERE cache_key = ?1",
                [cache_key],
                |row| {
                    Ok((
                        row.get::<_, String>(0)?,
                        row.get::<_, String>(1)?,
                        row.get::<_, String>(2)?,
                        row.get::<_, String>(3)?,
                        row.get::<_, i64>(4)?,
                        row.get::<_, i64>(5)?,
                        row.get::<_, String>(6)?,
                    ))
                },
            )
            .optional()?;

        let Some((
            cache_key,
            release_id,
            version,
            channel,
            published_at_unix,
            fetched_at_unix,
            source_url,
        )) = release
        else {
            return Ok(None);
        };

        let mut stmt = self.conn.prepare(
            "SELECT name, download_url, size_bytes, content_type, checksum_hex
             FROM release_artifacts WHERE cache_key = ?1 ORDER BY id ASC",
        )?;
        let rows = stmt.query_map([cache_key.clone()], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, i64>(2)?,
                row.get::<_, Option<String>>(3)?,
                row.get::<_, Option<String>>(4)?,
            ))
        })?;

        let mut artifacts = Vec::new();
        for row in rows {
            let (name, download_url, size_bytes, content_type, checksum_hex) = row?;
            artifacts.push(ReleaseArtifactRecord {
                name,
                download_url,
                size_bytes: from_i64(size_bytes)?,
                content_type,
                checksum_hex,
            });
        }
        Ok(Some(CachedReleaseMetadata {
            cache_key,
            release_id,
            version,
            channel: parse_release_channel(&channel)?,
            published_at_unix,
            fetched_at_unix,
            source_url,
            artifacts,
        }))
    }
}

impl ReleaseMetadataCache for SqliteStore {
    fn put_release_metadata(
        &mut self,
        release: &CachedReleaseMetadata,
    ) -> Result<(), kc_domain::DomainError> {
        self.put_release_internal(release).map_err(Into::into)
    }

    fn get_release_metadata(
        &self,
        cache_key: &str,
    ) -> Result<Option<CachedReleaseMetadata>, kc_domain::DomainError> {
        self.get_release_internal(cache_key).map_err(Into::into)
    }
}
