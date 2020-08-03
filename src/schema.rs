table! {
    /// Representation of the `emails` table.
    ///
    /// (Automatically generated by Diesel.)
    emails (email) {
        /// The `email` column of the `emails` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        email -> Varchar,
        /// The `userid` column of the `emails` table.
        ///
        /// Its SQL type is `Uuid`.
        ///
        /// (Automatically generated by Diesel.)
        userid -> Uuid,
    }
}

table! {
    /// Representation of the `users` table.
    ///
    /// (Automatically generated by Diesel.)
    users (id) {
        /// The `id` column of the `users` table.
        ///
        /// Its SQL type is `Uuid`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Uuid,
        /// The `name` column of the `users` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        name -> Varchar,
        /// The `avi_location` column of the `users` table.
        ///
        /// Its SQL type is `Nullable<Varchar>`.
        ///
        /// (Automatically generated by Diesel.)
        avi_location -> Nullable<Varchar>,
        /// The `hashed_pwd` column of the `users` table.
        ///
        /// Its SQL type is `Bpchar`.
        ///
        /// (Automatically generated by Diesel.)
        hashed_pwd -> Bpchar,
    }
}

joinable!(emails -> users (userid));

allow_tables_to_appear_in_same_query!(
    emails,
    users,
);
