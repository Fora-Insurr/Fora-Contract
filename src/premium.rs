use crate::types::PremiumInput;

const BASE_PREMIUM: i128 = 100;

pub fn calculate_premium(input: &PremiumInput) -> i128 {
    let age_factor = if input.age > 50 { 2u32 } else { 1u32 };
    let risk_factor = input.location_risk.max(1);
    let coverage_factor = input.coverage_type.max(1);
    let safety_discount = if input.safety_score > 80 { 80u32 } else { 100u32 };

    BASE_PREMIUM
        * age_factor as i128
        * risk_factor as i128
        * coverage_factor as i128
        * safety_discount as i128
        / 100
}
