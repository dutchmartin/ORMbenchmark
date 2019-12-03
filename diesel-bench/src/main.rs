#[macro_use]
extern crate diesel;

pub mod schema;
pub mod models;

use dotenv::dotenv;
use diesel::prelude::*;
use std::env;
use self::schema::*;
use self::models::*;
use crate::schema::customer::columns::customer_id;

fn main() {
    let connection = establish_connection();

    let results = customer::table
        .limit(20)
        .order_by(customer_id.asc())
        .load::<Customer>(&connection)
        .expect("Error loading");
    for customer in results {
        dbg!(customer);
    }
    let new_customers =
        vec!(
            NewCustomer {
                store_id: 1,
                first_name: String::from("john"),
                last_name: String::from("doe"),
                email: Some(String::from("jhon@doe.com")),
                address_id: 2,
                active: 1,
            },
            NewCustomer {
                store_id: 1,
                first_name: String::from("john"),
                last_name: String::from("doe"),
                email: Some(String::from("jhon@doe.com")),
                address_id: 2,
                active: 1,
            },
            NewCustomer {
                store_id: 1,
                first_name: String::from("john"),
                last_name: String::from("doe"),
                email: Some(String::from("jhon@doe.com")),
                address_id: 2,
                active: 1,
            },
            NewCustomer {
                store_id: 1,
                first_name: String::from("john"),
                last_name: String::from("doe"),
                email: Some(String::from("jhon@doe.com")),
                address_id: 2,
                active: 1,
            },
            NewCustomer {
                store_id: 1,
                first_name: String::from("john"),
                last_name: String::from("doe"),
                email: Some(String::from("jhon@doe.com")),
                address_id: 2,
                active: 1,
            },
            NewCustomer {
                store_id: 1,
                first_name: String::from("john"),
                last_name: String::from("doe"),
                email: Some(String::from("jhon@doe.com")),
                address_id: 2,
                active: 1,
            },
            NewCustomer {
                store_id: 1,
                first_name: String::from("john"),
                last_name: String::from("doe"),
                email: Some(String::from("jhon@doe.com")),
                address_id: 2,
                active: 1,
            },
            NewCustomer {
                store_id: 1,
                first_name: String::from("john"),
                last_name: String::from("doe"),
                email: Some(String::from("jhon@doe.com")),
                address_id: 2,
                active: 1,
            },
            NewCustomer {
                store_id: 1,
                first_name: String::from("john"),
                last_name: String::from("doe"),
                email: Some(String::from("jhon@doe.com")),
                address_id: 2,
                active: 1,
            },
            NewCustomer {
                store_id: 1,
                first_name: String::from("john"),
                last_name: String::from("doe"),
                email: Some(String::from("jhon@doe.com")),
                address_id: 2,
                active: 1,
            },
            NewCustomer {
                store_id: 1,
                first_name: String::from("john"),
                last_name: String::from("doe"),
                email: Some(String::from("jhon@doe.com")),
                address_id: 2,
                active: 1,
            },
            NewCustomer {
                store_id: 1,
                first_name: String::from("john"),
                last_name: String::from("doe"),
                email: Some(String::from("jhon@doe.com")),
                address_id: 2,
                active: 1,
            },
            NewCustomer {
                store_id: 1,
                first_name: String::from("john"),
                last_name: String::from("doe"),
                email: Some(String::from("jhon@doe.com")),
                address_id: 2,
                active: 1,
            },
            NewCustomer {
                store_id: 1,
                first_name: String::from("john"),
                last_name: String::from("doe"),
                email: Some(String::from("jhon@doe.com")),
                address_id: 2,
                active: 1,
            },
            NewCustomer {
                store_id: 1,
                first_name: String::from("john"),
                last_name: String::from("doe"),
                email: Some(String::from("jhon@doe.com")),
                address_id: 2,
                active: 1,
            },
            NewCustomer {
                store_id: 1,
                first_name: String::from("john"),
                last_name: String::from("doe"),
                email: Some(String::from("jhon@doe.com")),
                address_id: 2,
                active: 1,
            },
            NewCustomer {
                store_id: 1,
                first_name: String::from("john"),
                last_name: String::from("doe"),
                email: Some(String::from("jhon@doe.com")),
                address_id: 2,
                active: 1,
            },
            NewCustomer {
                store_id: 1,
                first_name: String::from("john"),
                last_name: String::from("doe"),
                email: Some(String::from("jhon@doe.com")),
                address_id: 2,
                active: 1,
            },
            NewCustomer {
                store_id: 1,
                first_name: String::from("john"),
                last_name: String::from("doe"),
                email: Some(String::from("jhon@doe.com")),
                address_id: 2,
                active: 1,
            },
            NewCustomer {
                store_id: 1,
                first_name: String::from("john"),
                last_name: String::from("doe"),
                email: Some(String::from("jhon@doe.com")),
                address_id: 2,
                active: 1,
            }
        );
    diesel::insert_into(customer::table)
        .values(&new_customers)
        .execute(&connection)
        .expect("Error on insert");
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}