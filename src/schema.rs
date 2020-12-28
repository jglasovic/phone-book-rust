table! {
    contact (id) {
        id -> Int4,
        first_name -> Varchar,
        last_name -> Varchar,
    }
}

table! {
    contact_tag (id) {
        id -> Int4,
        contact_id -> Int4,
        tag_id -> Int4,
    }
}

table! {
    email (id) {
        id -> Int4,
        email -> Varchar,
        contact_id -> Int4,
    }
}

table! {
    phone (id) {
        id -> Int4,
        phone_number -> Varchar,
        contact_id -> Int4,
    }
}

table! {
    tag (id) {
        id -> Int4,
        name -> Varchar,
    }
}

joinable!(contact_tag -> contact (contact_id));
joinable!(contact_tag -> tag (tag_id));
joinable!(email -> contact (contact_id));
joinable!(phone -> contact (contact_id));

allow_tables_to_appear_in_same_query!(
    contact,
    contact_tag,
    email,
    phone,
    tag,
);
