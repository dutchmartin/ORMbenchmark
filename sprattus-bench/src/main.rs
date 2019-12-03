mod models;

use crate::models::{Customer, NewCustomer};
use sprattus::*;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let conn = Connection::new("postgresql://tg:@localhost/postgres").await?;

    let limit: i64 = 20;
    let customers: Vec<Customer> = conn
        .query_multiple("select * from customer order by customer_id asc limit $1::Int8", &[&limit])
        .await?;
    for customer in customers {
        dbg!(customer);
    }
    let new_customers = vec![
        NewCustomer {
            customer_id: 0,
            store_id: 1,
            first_name: String::from("john"),
            last_name: String::from("doe"),
            email: Some(String::from("jhon@doe.com")),
            address_id: 2,
            active: 1,
        },
        NewCustomer {
            customer_id: 0,
            store_id: 1,
            first_name: String::from("john"),
            last_name: String::from("doe"),
            email: Some(String::from("jhon@doe.com")),
            address_id: 2,
            active: 1,
        },
        NewCustomer {
            customer_id: 0,
            store_id: 1,
            first_name: String::from("john"),
            last_name: String::from("doe"),
            email: Some(String::from("jhon@doe.com")),
            address_id: 2,
            active: 1,
        },
        NewCustomer {
            customer_id: 0,
            store_id: 1,
            first_name: String::from("john"),
            last_name: String::from("doe"),
            email: Some(String::from("jhon@doe.com")),
            address_id: 2,
            active: 1,
        },
        NewCustomer {
            customer_id: 0,
            store_id: 1,
            first_name: String::from("john"),
            last_name: String::from("doe"),
            email: Some(String::from("jhon@doe.com")),
            address_id: 2,
            active: 1,
        },
        NewCustomer {
            customer_id: 0,
            store_id: 1,
            first_name: String::from("john"),
            last_name: String::from("doe"),
            email: Some(String::from("jhon@doe.com")),
            address_id: 2,
            active: 1,
        },
        NewCustomer {
            customer_id: 0,
            store_id: 1,
            first_name: String::from("john"),
            last_name: String::from("doe"),
            email: Some(String::from("jhon@doe.com")),
            address_id: 2,
            active: 1,
        },
        NewCustomer {
            customer_id: 0,
            store_id: 1,
            first_name: String::from("john"),
            last_name: String::from("doe"),
            email: Some(String::from("jhon@doe.com")),
            address_id: 2,
            active: 1,
        },
        NewCustomer {
            customer_id: 0,
            store_id: 1,
            first_name: String::from("john"),
            last_name: String::from("doe"),
            email: Some(String::from("jhon@doe.com")),
            address_id: 2,
            active: 1,
        },
        NewCustomer {
            customer_id: 0,
            store_id: 1,
            first_name: String::from("john"),
            last_name: String::from("doe"),
            email: Some(String::from("jhon@doe.com")),
            address_id: 2,
            active: 1,
        },
        NewCustomer {
            customer_id: 0,
            store_id: 1,
            first_name: String::from("john"),
            last_name: String::from("doe"),
            email: Some(String::from("jhon@doe.com")),
            address_id: 2,
            active: 1,
        },
        NewCustomer {
            customer_id: 0,
            store_id: 1,
            first_name: String::from("john"),
            last_name: String::from("doe"),
            email: Some(String::from("jhon@doe.com")),
            address_id: 2,
            active: 1,
        },
        NewCustomer {
            customer_id: 0,
            store_id: 1,
            first_name: String::from("john"),
            last_name: String::from("doe"),
            email: Some(String::from("jhon@doe.com")),
            address_id: 2,
            active: 1,
        },
        NewCustomer {
            customer_id: 0,
            store_id: 1,
            first_name: String::from("john"),
            last_name: String::from("doe"),
            email: Some(String::from("jhon@doe.com")),
            address_id: 2,
            active: 1,
        },
        NewCustomer {
            customer_id: 0,
            store_id: 1,
            first_name: String::from("john"),
            last_name: String::from("doe"),
            email: Some(String::from("jhon@doe.com")),
            address_id: 2,
            active: 1,
        },
        NewCustomer {
            customer_id: 0,
            store_id: 1,
            first_name: String::from("john"),
            last_name: String::from("doe"),
            email: Some(String::from("jhon@doe.com")),
            address_id: 2,
            active: 1,
        },
        NewCustomer {
            customer_id: 0,
            store_id: 1,
            first_name: String::from("john"),
            last_name: String::from("doe"),
            email: Some(String::from("jhon@doe.com")),
            address_id: 2,
            active: 1,
        },
        NewCustomer {
            customer_id: 0,
            store_id: 1,
            first_name: String::from("john"),
            last_name: String::from("doe"),
            email: Some(String::from("jhon@doe.com")),
            address_id: 2,
            active: 1,
        },
        NewCustomer {
            customer_id: 0,
            store_id: 1,
            first_name: String::from("john"),
            last_name: String::from("doe"),
            email: Some(String::from("jhon@doe.com")),
            address_id: 2,
            active: 1,
        },
        NewCustomer {
            customer_id: 0,
            store_id: 1,
            first_name: String::from("john"),
            last_name: String::from("doe"),
            email: Some(String::from("jhon@doe.com")),
            address_id: 2,
            active: 1,
        },
    ];
    conn.create_multiple(&new_customers).await?;
    Ok(())
}
