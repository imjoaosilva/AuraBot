pub mod logs;
use chrono::Utc;
use chrono::{Datelike, Duration, Timelike};
use chrono_tz::America::Sao_Paulo;

pub fn format_amount(amount: u64) -> String {
    if amount >= 1_000_000 {
        format!("{}kk", amount / 1_000_000)
    } else if amount >= 1_000 {
        format!("{}k", amount / 1_000)
    } else {
        amount.to_string()
    }
}

pub fn get_next_monday_at_18() -> chrono::DateTime<chrono_tz::Tz> {
    let now_br = Utc::now().with_timezone(&Sao_Paulo);

    let days_until_monday = match now_br.weekday() {
        chrono::Weekday::Mon => 7,
        chrono::Weekday::Tue => 6,
        chrono::Weekday::Wed => 5,
        chrono::Weekday::Thu => 4,
        chrono::Weekday::Fri => 3,
        chrono::Weekday::Sat => 2,
        chrono::Weekday::Sun => 1,
    };

    let next_monday = now_br
        .checked_add_signed(Duration::days(days_until_monday as i64))
        .expect("Error calculating the next Monday");

    next_monday
        .with_hour(18)
        .expect("Error setting the hour to 18")
        .with_minute(0)
        .expect("Error setting the minutes to 0")
        .with_second(0)
        .expect("Error setting the seconds to 0")
}

pub fn get_last_monday_at_18() -> chrono::DateTime<chrono_tz::Tz> {
    let now_br = Utc::now().with_timezone(&Sao_Paulo);
    let weekday = now_br.weekday().num_days_from_sunday();

    let days_since_monday = if weekday == 1 { 7 } else { weekday as i64 - 1 };
    let last_monday = now_br
        .checked_sub_signed(Duration::days(days_since_monday))
        .unwrap()
        .with_hour(18)
        .unwrap()
        .with_minute(0)
        .unwrap()
        .with_second(0)
        .unwrap();

    last_monday
}
