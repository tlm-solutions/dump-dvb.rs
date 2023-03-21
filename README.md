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
		Uuid trekkie_run FK
		Timestamp timestamp
		Float8 lat
		Float8 lon
		Nullable<Float8> elevation
		Nullable<Float8> accuracy
		Nullable<Float8> vertical_accuracy
		Nullable<Float8> bearing
		Nullable<Float8> speed
	}
```
