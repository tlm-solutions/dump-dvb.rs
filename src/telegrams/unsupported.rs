
use super::TelegramTypes;

struct UnsupportedTelegram {
    pub telegram_type: TelegramType,
    pub data: Vec<u8>
}


struct UnsupportedSaveTelegram {
    pub id: Option<i64>,

    pub time: NaiveDateTime,
    pub station: Uuid,
    pub region: i32,

    pub telegram_type: TelegramType,
    pub data: Vec<u8>
}


