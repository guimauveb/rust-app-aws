table! {
    articles (id) {
        id -> Int4,
        title -> Text,
        pub_date -> Timestamp,
        published -> Bool,
        preview -> Text,
        image -> Text,
        content -> Text,
    }
}
