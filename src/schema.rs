table! {
    r09_telegrams (id) {
        id -> BigSerial,
        time -> Timestamp,
        station -> Uuid,
        region -> Integer,
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
