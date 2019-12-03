use chrono::{NaiveDate, NaiveDateTime};
use sprattus::*;

#[derive(FromSql, Debug)]
#[sql(table = "customer")]
pub struct Customer {
    customer_id: i32,
    store_id: i16,
    first_name: String,
    last_name: String,
    email: Option<String>,
    address_id: i16,
    activebool: bool,
    create_date: NaiveDate,
    last_update: NaiveDateTime,
    active: i32,
}

#[derive(ToSql, FromSql)]
#[sql(table = "customer")]
pub struct NewCustomer {
    #[sql(primary_key)]
    pub customer_id: i32,
    pub store_id: i16,
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    pub address_id: i16,
    pub active: i32,
}
