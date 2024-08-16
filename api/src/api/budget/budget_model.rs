pub enum Period {
    OneWeek,
    TwoWeeks,
    OneMonth,
    ThreeMonths,
    SixMonths,
    OneYear,
}

pub struct Budget {
    pub id: String,
    pub title: String,
    pub description: String,
    pub amount: i32,
    pub currency: String,
    pub period: Period,
    pub created_at: String,
    pub updated_at: String,
}
