# TLMS Rust Crate

[![built with nix](https://builtwithnix.org/badge.svg)](https://builtwithnix.org)

This crate contains all the reusable code for our vehicle tracking efforts. To
use, just drop it into your `Cargo.toml`.

## Building and Hacking

### With Nix (aka easy way)

This flake provides a devshell, which exposes all the dependencies
automatically. Just run `nix develop` anywhere in the repo.

### Without Nix

Just install the dependencies, build is done by cargo.

List of dependencies:
```
grpc
protobuf
websocketpp
pkg-config
postgresql_14
openssl
diesel-cli
```

## Documentation

Run `cargo doc --all-features --open` in a nix devshell, hosted version coming
soon-ish ;).

## Features 

List of rust features this crate exposes: `schema`, `management`, `locations`,
`telegrams`, `measurements`, `receivers`, `trekkie`, `gps`

## Entity Relationship diagram

```mermaid
erDiagram
	gps_points {
		Int8 id PK
		Uuid trekkie_run
		Timestamp timestamp
		Float8 lat
		Float8 lon
		Float8 elevation         "optional"
		Float8 accuracy          "optional"
		Float8 vertical_accuracy "optional"
		Float8 bearing           "optional"
		Float8 speed             "optional"
	}

	r09_telegrams {
		Int8 id PK
		Timestamp time
		Uuid station
		Int8 r09_type
		Int4 delay "optional"
		Int4 reporting_point
		Int4 junction
		Int2 direction
		Int2 request_status
		Int2 priority "optional"
		Int2 direction_request "optional"
		Int4 line "optional"
		Int4 run_number "optional"
		Int4 destination_number "optional"
		Int4 train_length "optional"
		Int4 vehicle_number "optional"
		Int2 operator "optional"
		Int8 region "optional"
	}

	r09_transmission_locations {
		Int8 id PK
		Int8 region
		Int4 reporting_point
		Float8 lat
		Float8 lon
	}

	r09_transmission_locations_raw {
		Int8 id PK
		Int8 region
		Int4 reporting_point
		Float8 lat
		Float8 lon
		Uuid trekkie_run
		Uuid run_owner
	}

	raw_telegrams {
		Int8 id PK
		Timestamp time
		Uuid station
		Int8 telegram_type
		Bytea data
	}

	regions {
		Int8 id PK
		Text name
		Text transport_company
		Text regional_company "optional"
		Int8 frequency "optional"
		Int8 r09_type "optional"
		Int4 encoding "optional"
		Bool deactivated
	}

	stations {
		Uuid id PK
		Varchar token "optional"
		Text name
		Float8 lat
		Float8 lon
		Int8 region
		Uuid owner
		Bool approved
		Bool deactivated
		Bool public
		Int4 radio "optional"
		Int4 architecture "optional"
		Int4 device "optional"
		Float8 elevation "optional"
		Int4 antenna "optional"
		Text telegram_decoder_version "optional"
		Text notes "optional"
	}

	trekkie_runs {
		Timestamp start_time
		Timestamp end_time
		Int4 line
		Int4 run
		Int8 region
		Uuid owner
		Bool finished
		Uuid id
	}

	users {
		Uuid id
		Text name "optional"
		Text email "optional"
		Varchar password
		Int4 role
		Int4 email_setting "optional"
		Bool deactivated
	}
```
