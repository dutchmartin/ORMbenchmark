use chrono::{NaiveDate, NaiveDateTime};
use crate::schema::*;

#[derive(Queryable, Debug)]
#[diesel(table_name="customer")]
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

#[derive(Insertable)]
#[table_name="customer"]
pub struct NewCustomer {
    pub store_id: i16,
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    pub address_id: i16,
    pub active: i32,
}