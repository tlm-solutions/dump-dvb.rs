# TLMS.rs Changelog

## Unreleased

## v0.9.0

### Breaking

- Removed Graph Defitions
- Removed Depreacated Structs
- schema and type for train_length to be consistent with the database schema
- `locations::graph` yeetus-deletus
- Region moved to `structs`
- InsertRegion moved to `structs`
- type renamed `R09Types` -> `R09Type`
- `R09Type` moved to `telegrams::R09`
- `R09SaveTelegram.telegram_type` now uses `R09Type` instead of `i64`
- `telegram_type` is not named `r09_type` everywhere
- lat and lon are now everywhere f64

### Added

- Waypoints
- Chemo Service
- custom diesel \[de\]serializer for the `R09Type`
- r09_transmission_locations
- r09_transmission_locations_raw
- `TransmissionLocation`, `TransmissionLocationRaw`, `InsertTransmissionLocationRaw`, `InsertTransmissionLocation` for tables above

### Fixed

### Misc

## 0.8.0
This release is a start of us properly releasing. Anything before this lost to
history.



