/// Calcula a taxa com base no valor e na porcentagem fee_rate.
/// Retorna None se ocorrer overflow.
pub fn calculate_fee(amount: u64, fee_rate: u64) -> Option<u64> {
    amount.checked_mul(fee_rate)?.checked_div(100)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_fee_normal() {
        // Valor 1000 com fee 5% deve resultar em 50
        let fee = calculate_fee(1000, 5);
        assert_eq!(fee, Some(50));
    }

    #[test]
    fn test_calculate_fee_overflow() {
        // Teste para garantir que um overflow seja tratado corretamente
        let max = u64::MAX;
        let fee = calculate_fee(max, 100);
        // Se ocorrer overflow, a função retornará None.
        assert_eq!(fee, None);
    }
}