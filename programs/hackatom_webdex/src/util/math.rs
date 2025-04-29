/// Calcula a taxa com base no valor e na porcentagem fee_rate.
/// Retorna None se ocorrer overflow.
pub fn calculate_fee(amount: u64, fee_rate: u64) -> Option<u64> {
    amount.checked_mul(fee_rate)?.checked_div(100)
}
