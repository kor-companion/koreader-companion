use kc_domain::{
    BackupEntryKind, CachedReleaseChannel, DeviceKind, LogSeverity, SupportLevel, TransportKind,
};

use crate::PersistenceError;

pub fn encode_transport_kind(value: &TransportKind) -> String {
    match value {
        TransportKind::LocalFilesystem => "known:local-filesystem".to_string(),
        TransportKind::UsbMassStorage => "known:usb-mass-storage".to_string(),
        TransportKind::NetworkShare => "known:network-share".to_string(),
        TransportKind::Adb => "known:adb".to_string(),
        TransportKind::Ssh => "known:ssh".to_string(),
        TransportKind::MobileDocumentProvider => "known:mobile-document-provider".to_string(),
        TransportKind::Other(value) => format!("other:{}", super::address::encode_component(value)),
    }
}

pub fn parse_transport_kind(value: &str) -> Result<TransportKind, PersistenceError> {
    parse_tagged_other(value, "transport_kind", |token| match token {
        "local-filesystem" => Ok(TransportKind::LocalFilesystem),
        "usb-mass-storage" => Ok(TransportKind::UsbMassStorage),
        "network-share" => Ok(TransportKind::NetworkShare),
        "adb" => Ok(TransportKind::Adb),
        "ssh" => Ok(TransportKind::Ssh),
        "mobile-document-provider" => Ok(TransportKind::MobileDocumentProvider),
        _ => Err(PersistenceError::InvalidEnum {
            field: "transport_kind",
            value: value.to_string(),
        }),
    })
    .map(|parsed| match parsed {
        ParsedTaggedOther::Known(kind) => kind,
        ParsedTaggedOther::Other(other) => TransportKind::Other(other),
    })
}

pub fn encode_device_kind(value: &DeviceKind) -> String {
    match value {
        DeviceKind::Kobo => "known:kobo".to_string(),
        DeviceKind::PocketBook => "known:pocketbook".to_string(),
        DeviceKind::Kindle => "known:kindle".to_string(),
        DeviceKind::Android => "known:android".to_string(),
        DeviceKind::Remarkable => "known:remarkable".to_string(),
        DeviceKind::Other(value) => format!("other:{}", super::address::encode_component(value)),
    }
}

pub fn parse_device_kind(value: &str) -> Result<DeviceKind, PersistenceError> {
    parse_tagged_other(value, "device_kind", |token| match token {
        "kobo" => Ok(DeviceKind::Kobo),
        "pocketbook" => Ok(DeviceKind::PocketBook),
        "kindle" => Ok(DeviceKind::Kindle),
        "android" => Ok(DeviceKind::Android),
        "remarkable" => Ok(DeviceKind::Remarkable),
        _ => Err(PersistenceError::InvalidEnum {
            field: "device_kind",
            value: value.to_string(),
        }),
    })
    .map(|parsed| match parsed {
        ParsedTaggedOther::Known(kind) => kind,
        ParsedTaggedOther::Other(other) => DeviceKind::Other(other),
    })
}

pub fn encode_support_level(value: SupportLevel) -> &'static str {
    match value {
        SupportLevel::Supported => "supported",
        SupportLevel::Experimental => "experimental",
        SupportLevel::Unsupported => "unsupported",
    }
}

pub fn parse_support_level(value: &str) -> Result<SupportLevel, PersistenceError> {
    match value {
        "supported" => Ok(SupportLevel::Supported),
        "experimental" => Ok(SupportLevel::Experimental),
        "unsupported" => Ok(SupportLevel::Unsupported),
        _ => Err(PersistenceError::InvalidEnum {
            field: "support_level",
            value: value.to_string(),
        }),
    }
}

pub fn encode_log_severity(value: LogSeverity) -> &'static str {
    match value {
        LogSeverity::Info => "info",
        LogSeverity::Warning => "warning",
        LogSeverity::Error => "error",
    }
}

pub fn parse_log_severity(value: &str) -> Result<LogSeverity, PersistenceError> {
    match value {
        "info" => Ok(LogSeverity::Info),
        "warning" => Ok(LogSeverity::Warning),
        "error" => Ok(LogSeverity::Error),
        _ => Err(PersistenceError::InvalidEnum {
            field: "severity",
            value: value.to_string(),
        }),
    }
}

pub fn encode_backup_entry_kind(value: BackupEntryKind) -> &'static str {
    match value {
        BackupEntryKind::File => "file",
        BackupEntryKind::Directory => "directory",
    }
}

pub fn parse_backup_entry_kind(value: &str) -> Result<BackupEntryKind, PersistenceError> {
    match value {
        "file" => Ok(BackupEntryKind::File),
        "directory" => Ok(BackupEntryKind::Directory),
        _ => Err(PersistenceError::InvalidEnum {
            field: "backup_entry_kind",
            value: value.to_string(),
        }),
    }
}

pub fn encode_release_channel(value: &CachedReleaseChannel) -> String {
    match value {
        CachedReleaseChannel::Stable => "known:stable".to_string(),
        CachedReleaseChannel::Prerelease => "known:prerelease".to_string(),
        CachedReleaseChannel::Other(value) => {
            format!("other:{}", super::address::encode_component(value))
        }
    }
}

pub fn parse_release_channel(value: &str) -> Result<CachedReleaseChannel, PersistenceError> {
    parse_tagged_other(value, "release_channel", |token| match token {
        "stable" => Ok(CachedReleaseChannel::Stable),
        "prerelease" => Ok(CachedReleaseChannel::Prerelease),
        _ => Err(PersistenceError::InvalidEnum {
            field: "release_channel",
            value: value.to_string(),
        }),
    })
    .map(|parsed| match parsed {
        ParsedTaggedOther::Known(channel) => channel,
        ParsedTaggedOther::Other(other) => CachedReleaseChannel::Other(other),
    })
}

enum ParsedTaggedOther<T> {
    Known(T),
    Other(String),
}

fn parse_tagged_other<T>(
    value: &str,
    field: &'static str,
    parse_known: impl FnOnce(&str) -> Result<T, PersistenceError>,
) -> Result<ParsedTaggedOther<T>, PersistenceError> {
    if let Some(token) = value.strip_prefix("known:") {
        return parse_known(token).map(ParsedTaggedOther::Known);
    }

    if let Some(encoded) = value.strip_prefix("other:") {
        let decoded = super::address::decode_component(field, encoded)?;
        if decoded.trim().is_empty() {
            return Err(PersistenceError::InvalidEnum {
                field,
                value: value.to_string(),
            });
        }
        return Ok(ParsedTaggedOther::Other(decoded));
    }

    Err(PersistenceError::InvalidEnum {
        field,
        value: value.to_string(),
    })
}
