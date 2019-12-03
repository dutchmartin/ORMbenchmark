table! {
    customer (customer_id) {
        customer_id -> Int4,
        store_id -> Int2,
        first_name -> Varchar,
        last_name -> Varchar,
        email -> Nullable<Varchar>,
        address_id -> Int2,
        activebool -> Bool,
        create_date -> Date,
        last_update -> Timestamp,
        active -> Int4,
    }
}