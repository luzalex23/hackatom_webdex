/// Calcula a taxa com base no valor e na porcentagem de fee_rate.
pub fn calculate_fee(amount: u64, fee_rate: u64) -> u64 {
    amount.checked_mul(fee_rate).unwrap().checked_div(100).unwrap()
}
