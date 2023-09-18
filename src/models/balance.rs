#[derive(sqlx::Type, strum_macros::EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum BType {
    Cash,
    Card,
}
pub struct Balance {
    pub id: i32,
    pub uid: i32,
    pub btype: String,
    pub name: String,
    pub iban: Option<String>,
    pub balance: i32,
}

