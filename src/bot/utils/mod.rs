pub mod logs;
use chrono::Weekday;
use chrono::{Datelike, Duration, Timelike};

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
    let now_br = chrono::Utc::now().with_timezone(&chrono_tz::America::Sao_Paulo);

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
        .checked_add_signed(chrono::Duration::days(days_until_monday as i64))
        .expect("Error calculating the next Monday");

    next_monday
        .with_hour(18)
        .unwrap()
        .with_minute(0)
        .unwrap()
        .with_second(0)
        .unwrap()
}

pub fn get_last_monday_at_18() -> chrono::DateTime<chrono_tz::Tz> {
    let now_br = chrono::Utc::now().with_timezone(&chrono_tz::America::Sao_Paulo);
    let weekday = now_br.weekday();

    if weekday == Weekday::Mon && now_br.hour() < 18 {
        let subtract = now_br - Duration::days(7);
        return subtract
            .with_hour(18)
            .expect("Erro ao definir a hora para 18")
            .with_minute(0)
            .expect("Erro ao definir os minutos para 0")
            .with_second(0)
            .expect("Erro ao definir os segundos para 0");
    }

    let days_to_subtract = match weekday {
        Weekday::Mon => 0,
        _ => weekday.num_days_from_monday() as i64,
    };

    let last_monday = now_br - Duration::days(days_to_subtract);

    last_monday
        .with_hour(18)
        .expect("Erro ao definir a hora para 18")
        .with_minute(0)
        .expect("Erro ao definir os minutos para 0")
        .with_second(0)
        .expect("Erro ao definir os segundos para 0")
}
