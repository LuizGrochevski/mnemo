use chrono::NaiveDate;

#[derive(Debug)]
pub struct Card {
    pub id: i64,
    pub front: String,
    pub back: String,
    pub interval: i32,
    pub repetition: i32,
    pub ease_factor: f64,
    pub due_date: NaiveDate,
}
