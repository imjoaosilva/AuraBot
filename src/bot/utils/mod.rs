pub mod logs;
use chrono::Utc;

pub fn format_amount(amount: u64) -> String {
    if amount >= 1_000_000 {
        format!("{}kk", amount / 1_000_000)
    } else if amount >= 1_000 {
        format!("{}k", amount / 1_000)
    } else {
        amount.to_string()
    }
}

pub fn get_next_monday_at_18() -> chrono::DateTime<Utc> {
    use chrono::{Datelike, Duration, Timelike, Utc};

    let now = Utc::now();

    let days_until_monday = (7 - now.weekday().num_days_from_sunday()) % 7;
    let next_monday = now
        .checked_add_signed(Duration::days(days_until_monday as i64))
        .expect("Erro ao calcular a próxima segunda-feira");

    next_monday
        .with_hour(18)
        .expect("Erro ao definir a hora para 18")
        .with_minute(0)
        .expect("Erro ao definir os minutos para 0")
        .with_second(0)
        .expect("Erro ao definir os segundos para 0")
}

pub fn get_last_monday_at_18() -> chrono::DateTime<Utc> {
    use chrono::{Datelike, Duration, Timelike, Utc};

    let now = Utc::now();
    let weekday = now.weekday().num_days_from_sunday();

    let days_since_monday = if weekday == 0 { 6 } else { weekday as i64 - 1 };
    let last_monday = now
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
