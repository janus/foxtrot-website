// Define your GraphQL queries here
pub const GET_USER_DATA: &str = r#"
    query GetUserData($id: ID!) {
        user(id: $id) {
            id
            name
            balance
        }
    }
"#;

