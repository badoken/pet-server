diesel::table! {
    notes (id) {
        id -> Uuid,
        name -> Text,
        content -> Text,
    }
}