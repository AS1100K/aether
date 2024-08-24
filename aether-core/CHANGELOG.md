# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.0] 24-08-2024

### Added

- Added Discord Chat Relay
- Added Discord Notifications for important events
- Added Discord to take command

### Changed

- Upgrade dependency `serde_json`
- Changed bot joining mechanism to retry forever

### Removed

- Removed azalea patch i.e. [`better-1.20.6`](https://github.com/as1100k-forks/azalea.git)

### Fixed

- Removed Auto Eat [Issue #24](https://github.com/AS1100K/aether/issues/24)

## [0.3.0-beta.1] 07-08-2024

### Added

- Added `InWorld`, and `ExecutingTask` Component [#22](https://github.com/AS1100K/aether/pull/22)
- Added `PearlLoad` Event [#22](https://github.com/AS1100K/aether/pull/22)
- Added Auto Eat
- Added Auto Totem

### Changed

- Migrated from Tokio to Bevy ECS [#22](https://github.com/AS1100K/aether/pull/22)

### Removed

- Removed Integration with `azalea_discord` [#22](https://github.com/AS1100K/aether/pull/22)

_For previous Changelog, you need to do `git blame`_
