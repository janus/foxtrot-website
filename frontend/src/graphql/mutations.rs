// Define your GraphQL mutations here
pub const TRANSFER_FUNDS: &str = r#"
    mutation TransferFunds($from: ID!, $to: ID!, $amount: Float!) {
        transferFunds(from: $from, to: $to, amount: $amount) {
            success
            message
        }
    }
"#;

