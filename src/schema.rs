// @generated automatically by Diesel CLI.

diesel::table! {
    gps_points (id) {
        id -> Int8,
        trekkie_run -> Int8,
        timestamp -> Timestamp,
        lat -> Float8,
        lon -> Float8,
        elevation -> Nullable<Float8>,
        accuracy -> Nullable<Float8>,
        verical_accuracy -> Nullable<Float8>,
        bearing -> Nullable<Float8>,
        speed -> Nullable<Float8>,
    }
}

diesel::table! {
    internal_stations (id) {
        id -> Uuid,
        wireguard_number -> Nullable<Int4>,
    }
}

diesel::table! {
    r09_telegrams (id) {
        id -> Int8,
        time -> Timestamp,
        station -> Uuid,
        telegram_type -> Int8,
        delay -> Nullable<Int4>,
        reporting_point -> Int4,
        junction -> Int4,
        direction -> Int2,
        request_status -> Int2,
        priority -> Nullable<Int2>,
        direction_request -> Nullable<Int2>,
        line -> Nullable<Int4>,
        run_number -> Nullable<Int4>,
        destination_number -> Nullable<Int4>,
        train_length -> Nullable<Int2>,
        vehicle_number -> Nullable<Int4>,
        operator -> Nullable<Int2>,
        region -> Int8,
    }
}

diesel::table! {
    raw_telegrams (id) {
        id -> Int8,
        time -> Timestamp,
        station -> Uuid,
        telegram_type -> Int8,
        data -> Bytea,
    }
}

diesel::table! {
    regions (id) {
        id -> Int8,
        name -> Text,
        transport_company -> Text,
        regional_company -> Nullable<Text>,
        frequency -> Nullable<Int8>,
        r09_type -> Nullable<Int4>,
        encoding -> Nullable<Int4>,
        deactivated -> Bool,
    }
}

diesel::table! {
    sessions (owner) {
        owner -> Uuid,
        start_time -> Timestamp,
        token -> Varchar,
    }
}

diesel::table! {
    station_history (id) {
        id -> Int8,
        changed_time -> Timestamp,
        station_id -> Uuid,
        name -> Text,
        lat -> Float8,
        lon -> Float8,
        region -> Int8,
        approved -> Bool,
        deactivated -> Bool,
        public -> Bool,
        radio -> Nullable<Int4>,
        architecture -> Nullable<Int4>,
        device -> Nullable<Int4>,
        elevation -> Nullable<Float8>,
        telegram_decoder_version -> Nullable<Array<Nullable<Int4>>>,
        antenna -> Nullable<Int4>,
    }
}

diesel::table! {
    stations (id) {
        id -> Uuid,
        token -> Nullable<Varchar>,
        name -> Text,
        lat -> Float8,
        lon -> Float8,
        region -> Int8,
        owner -> Uuid,
        approved -> Bool,
        deactivated -> Bool,
        public -> Bool,
        radio -> Nullable<Int4>,
        architecture -> Nullable<Int4>,
        device -> Nullable<Int4>,
        elevation -> Nullable<Float8>,
        antenna -> Nullable<Int4>,
        telegram_decoder_version -> Nullable<Text>,
        notes -> Nullable<Text>,
    }
}

diesel::table! {
    trekkie_runs (id) {
        id -> Int8,
        start_time -> Timestamp,
        end_time -> Timestamp,
        line -> Int4,
        run -> Int4,
        gps_file -> Text,
        region -> Int8,
        owner -> Uuid,
        finished -> Bool,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        name -> Nullable<Text>,
        email -> Nullable<Text>,
        password -> Varchar,
        role -> Int4,
        email_setting -> Nullable<Int4>,
        deactivated -> Bool,
    }
}

diesel::joinable!(gps_points -> trekkie_runs (trekkie_run));
diesel::joinable!(internal_stations -> stations (id));
diesel::joinable!(r09_telegrams -> stations (station));
diesel::joinable!(raw_telegrams -> stations (station));
diesel::joinable!(sessions -> users (owner));
diesel::joinable!(station_history -> regions (region));
diesel::joinable!(station_history -> stations (station_id));
diesel::joinable!(stations -> regions (region));
diesel::joinable!(stations -> users (owner));
diesel::joinable!(trekkie_runs -> regions (region));
diesel::joinable!(trekkie_runs -> users (owner));

diesel::allow_tables_to_appear_in_same_query!(
    gps_points,
    internal_stations,
    r09_telegrams,
    raw_telegrams,
    regions,
    sessions,
    station_history,
    stations,
    trekkie_runs,
    users,
);
