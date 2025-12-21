# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [0.1.0] - 2024-12-21

### Added

- Initial release
- Core audit engine with hardware, services, telemetry detection
- Sync engine with timer, power, services, firewall, MSI modules
- Persistence engine with snapshot and rollback
- CLI with 7 commands: audit, analyze, sync, status, interactive, rollback, verify
- Interactive mode with category-based selection
- Laptop detection with hardware-aware recommendations
- Automatic snapshot before any modification
- Three profiles: gaming, privacy, workstation
- Telemetry domain blocklist (42 domains)
- JSON report export

### Security

- All modifications reversible via snapshot
- Non-destructive audit and analyze commands
- Confirmation required for sync operations
