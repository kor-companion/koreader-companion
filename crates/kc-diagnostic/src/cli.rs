use kc_domain::DomainError;

pub enum Command {
    Foundation,
    Probe { path: String },
}

pub fn parse_command(args: &[String]) -> Result<Command, DomainError> {
    match args {
        [] => Ok(Command::Foundation),
        [command] if command == "foundation" || command == "report" => Ok(Command::Foundation),
        [command, path] if command == "probe" => Ok(Command::Probe { path: path.clone() }),
        _ => Err(DomainError::Validation(
            "invalid diagnostic command usage".to_string(),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_accepts_foundation_and_probe_forms() {
        assert!(matches!(parse_command(&[]).unwrap(), Command::Foundation));
        assert!(matches!(
            parse_command(&["probe".to_string(), "/mnt/kobo".to_string()]).unwrap(),
            Command::Probe { .. }
        ));
    }

    #[test]
    fn parser_rejects_invalid_usage() {
        assert!(matches!(
            parse_command(&["probe".to_string()]),
            Err(DomainError::Validation(message)) if message == "invalid diagnostic command usage"
        ));
    }
}
