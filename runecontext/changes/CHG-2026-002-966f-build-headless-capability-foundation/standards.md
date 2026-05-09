## Applicable Standards
- `standards/development/source-size-policy.md`
- `standards/architecture/transport-aware-addressing.md`

## Resolution Notes
These standards were promoted from durable rules established by the headless foundation change.

`development/source-size-policy` captures the repository-wide rule to keep implementation files reviewable and enforce the source-size limit in fast verification.

`architecture/transport-aware-addressing` captures the rule that shared layers stay transport-aware so future ADB, SSH, SAF, and similar integrations do not require core workflow rewrites.
