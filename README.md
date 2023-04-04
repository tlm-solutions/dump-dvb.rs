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
		BIGSERIAL id PK
		UUID trekkie_run FK "trekkie_runs(id)"
		TIMESTAMP timestamp
		DOUBLE lat
		DOUBLE lon
		DOUBLE elevation         "optional"
		DOUBLE accuracy          "optional"
		DOUBLE vertical_accuracy "optional"
		DOUBLE bearing           "optional"
		DOUBLE speed             "optional"
	}

	r09_telegrams {
		BIGSERIAL id PK
		TIMESTAMP time
		UUID station FK "stations(id)"
		BIGINT r09_type
		INT delay              "optional"
		INT reporting_point
		INT junction
		SMALLINT direction
		SMALLINT request_status
		SMALLINT priority           "optional"
		SMALLINT direction_request  "optional"
		INT line               "optional"
		INT run_number         "optional"
		INT destination_number "optional"
		INT train_length       "optional"
		INT vehicle_number     "optional"
		SMALLINT operator           "optional"
		BIGINT region FK "regions(id)"
	}

	r09_transmission_locations {
		BIGSERIAL id PK
		BIGINT region FK "regions(id)"
		INT reporting_point
		DOUBLE lat
		DOUBLE lon
	}

	r09_transmission_locations_raw {
		BIGSERIAL id PK
		BIGINT region FK "regions(id)"
		INT reporting_point
		DOUBLE lat
		DOUBLE lon
		UUID trekkie_run FK "trekkie_runs(id)"
		UUID run_owner FK "users(id)"
	}

	raw_telegrams {
		BIGSERIAL id PK
		TIMESTAMP time
		UUID station FK "stations(id)"
		BIGINT telegram_type
		BYTEA data
	}

	regions {
		BIGSERIAL id PK
		TEXT name
		TEXT transport_company
		TEXT regional_company   "optional"
		BIGINT frequency          "optional"
		BIGINT r09_type           "optional"
		INT encoding           "optional"
		BOOLEAN deactivated
	}

	stations {
		UUID id PK
		VARCHAR(36) token                 "optional"
		TEXT name
		DOUBLE lat
		DOUBLE lon
		BIGSERIAL region FK "regions(id)"
		UUID owner FK "users(id)"
		BOOLEAN approved
		BOOLEAN deactivated
		BOOLEAN public
		INT radio                    "optional"
		INT architecture             "optional"
		INT device                   "optional"
		DOUBLE elevation              "optional"
		INT antenna                  "optional"
		TEXT telegram_decoder_version "optional"
		TEXT notes                    "optional"
        UUID organization FK "organizations(id)"
	}

	trekkie_runs {
		TIMESTAMP start_time
		TIMESTAMP end_time
		INT line
		INT run
		BIGSERIAL region FK "regions(id)"
		UUID owner FK "users(id)"
		BOOLEAN finished
		UUID id PK
        BOOLEAN correlated
	}

	users {
		UUID id PK
		TEXT name          "optional"
		TEXT email         "optional"
		VARCHAR(100) password
		INT role
		INT email_setting "optional"
		BOOLEAN deactivated
	}

    organizations {
		UUID id PK
		TEXT name
        BOOLEAN public
        UUID owner FK "users(id)"
        BOOLEAN deactivated
	}

    org_users_relations {
        UUID id PK
        UUID organization FK "organizations(id)"
        UUID user_id FK "users(id)"
        INT role
    }



  gps_points }|--|| trekkie_runs : "contains"
  r09_telegrams }|--|| regions : "received in"
  r09_telegrams }|--|| stations : "received"
  r09_transmission_locations }|--|| regions : "has"
  r09_transmission_locations_raw }|--|| regions : ""
  r09_transmission_locations_raw }|--|| trekkie_runs : "contains"
  r09_transmission_locations_raw }|--|| users : ""
  raw_telegrams }|--|| stations : "received"
  stations }|--|| regions : "contains"
  stations }|--|| users : "owns"
  stations }|--|| organizations: "belongs"
  organizations }|--|| users : "manages"
  trekkie_runs }|--|| regions : "in"
  trekkie_runs }|--|| users : "from"
  org_users_relations }|--|| users : "has role"
  org_users_relations }|--|| organizations : "associated key"
```
