use chrono::prelude::*;


pub fn date_now()  ->  chrono::NaiveDateTime {
    return chrono::offset::Utc::now().naive_utc();
}