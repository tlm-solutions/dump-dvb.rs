// @generated automatically by Diesel CLI.

diesel::table! {
    gps_points (id) {
        id -> Int8,
        trekkie_run -> Uuid,
        timestamp -> Timestamp,
        lat -> Float8,
        lon -> Float8,
        elevation -> Nullable<Float8>,
        accuracy -> Nullable<Float8>,
        vertical_accuracy -> Nullable<Float8>,
        bearing -> Nullable<Float8>,
        speed -> Nullable<Float8>,
    }
}

diesel::table! {
    org_users_relation (id) {
        id -> Uuid,
        organisation -> Uuid,
        user_id -> Uuid,
        role -> Int4,
    }
}

diesel::table! {
    organization (id) {
        id -> Uuid,
        name -> Text,
        public -> Bool,
    }
}

diesel::table! {
    r09_telegrams (id) {
        id -> Int8,
        time -> Timestamp,
        station -> Uuid,
        r09_type -> Int8,
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
        train_length -> Nullable<Int4>,
        vehicle_number -> Nullable<Int4>,
        operator -> Nullable<Int2>,
        region -> Int8,
    }
}

diesel::table! {
    r09_transmission_locations (id) {
        id -> Int8,
        region -> Int8,
        reporting_point -> Int4,
        lat -> Float8,
        lon -> Float8,
        ground_truth -> Bool,
    }
}

diesel::table! {
    r09_transmission_locations_raw (id) {
        id -> Int8,
        region -> Int8,
        reporting_point -> Int4,
        lat -> Float8,
        lon -> Float8,
        trekkie_run -> Uuid,
        run_owner -> Uuid,
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
        r09_type -> Nullable<Int8>,
        encoding -> Nullable<Int4>,
        deactivated -> Bool,
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
        organization -> Uuid,
    }
}

diesel::table! {
    trekkie_runs (id) {
        start_time -> Timestamp,
        end_time -> Timestamp,
        line -> Int4,
        run -> Int4,
        region -> Int8,
        owner -> Uuid,
        finished -> Bool,
        id -> Uuid,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        name -> Nullable<Text>,
        email -> Nullable<Text>,
        password -> Varchar,
        email_setting -> Nullable<Int4>,
        deactivated -> Bool,
        admin -> Bool,
    }
}

diesel::joinable!(gps_points -> trekkie_runs (trekkie_run));
diesel::joinable!(org_users_relation -> organization (organisation));
diesel::joinable!(org_users_relation -> users (user_id));
diesel::joinable!(r09_telegrams -> regions (region));
diesel::joinable!(r09_telegrams -> stations (station));
diesel::joinable!(r09_transmission_locations -> regions (region));
diesel::joinable!(r09_transmission_locations_raw -> regions (region));
diesel::joinable!(r09_transmission_locations_raw -> trekkie_runs (trekkie_run));
diesel::joinable!(r09_transmission_locations_raw -> users (run_owner));
diesel::joinable!(raw_telegrams -> stations (station));
diesel::joinable!(stations -> organization (organization));
diesel::joinable!(stations -> regions (region));
diesel::joinable!(stations -> users (owner));
diesel::joinable!(trekkie_runs -> regions (region));
diesel::joinable!(trekkie_runs -> users (owner));

diesel::allow_tables_to_appear_in_same_query!(
    gps_points,
    org_users_relation,
    organization,
    r09_telegrams,
    r09_transmission_locations,
    r09_transmission_locations_raw,
    raw_telegrams,
    regions,
    stations,
    trekkie_runs,
    users,
);
