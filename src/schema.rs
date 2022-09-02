
table! {
    r09_telegrams (id) {
        id -> BigSerial,
        time -> Timestamp,
        station -> Uuid,
        telegram_type -> SmallInt,
        delay -> Nullable<Integer>,
        reporting_point -> Integer,
        junction -> Integer,
        direction -> SmallInt,
        request_status -> SmallInt,
        priority -> Nullable<SmallInt>,
        direction_request -> Nullable<SmallInt>,
        line -> Nullable<Integer>,
        run_number -> Nullable<Integer>,
        destination_number -> Nullable<Integer>,
        train_length -> Nullable<SmallInt>,
        vehicle_number -> Nullable<Integer>,
        operator -> Nullable<SmallInt>,
    }
}

table! {
    raw_telegrams (id) {
        id -> BigSerial,
        time -> Timestamp,
        station -> Uuid,
        region -> Integer,
        telegram_type -> SmallInt,
        data -> Binary,
    }
}

table! {
    stations (id) {
        id -> Uuid,
        token -> Nullable<VarChar>,
        name -> Text,
        lat -> Double,
        lon -> Double,
        region -> Serial,
        owner -> Uuid,
        approved -> Bool,
        deactivated -> Bool,
        public -> Bool,
        radio -> Nullable<Integer>,
        architecture -> Nullable<Integer>,
        device -> Nullable<Integer>,
        elevation -> Nullable<Double>,
        telegram_decoder_version -> Nullable<Array<Integer>>,
        antenna -> Nullable<Integer>,
    }
}

table! {
    users (id) {
        id -> Uuid,
        name -> Text,
        email -> Text,
        password -> VarChar,
        role -> Integer,
        email_setting -> Integer,
        deactivated -> Bool,
    }
}

table! {
    regions (id) {
        id -> Nullable<Serial>,
        name -> Text,
        transport_company -> Text,
        regional_company -> Nullable<Text>,
        frequency -> Nullable<BigInt>,
        r09_type -> Nullable<Integer>,
        encoding -> Nullable<Integer>,
    }
}

