use anchor_lang::prelude::*;
use static_assertions::const_assert_eq;
use std::mem::size_of;

use crate::OracleError;

#[repr(C)]
#[derive(Debug, Default, AnchorSerialize, AnchorDeserialize, Copy, Clone)]
pub struct OracleData {
    pub last_update_slot: u64,
    pub price: f64,
    pub quote_size_usdc_native: u64,
    pub mint: Pubkey,
    pub standard_deviation: f64,
    pub recent_prices: [f64; 16], // unnecessary?
    pub name: [u8; 16],
    pub recent_price_index: u8,
    pub padding: [u8; 7],
}
const_assert_eq!(size_of::<OracleData>(), 8 + 8 + 8 + 32 + 8 + 128 + 16 + 8);
const_assert_eq!(size_of::<OracleData>(), 216);
const_assert_eq!(size_of::<OracleData>() % 8, 0);

impl OracleData {
    pub fn update(&mut self, new_price: f64, slot: u64) -> Result<()> {
        require!(new_price > 0.0, OracleError::InvalidOraclePrice);

        self.price = new_price;
        self.last_update_slot = slot;
        self.recent_price_index = (self.recent_price_index + 1) % 16;
        self.recent_prices[self.recent_price_index as usize] = new_price;
        self.standard_deviation = self.calc_std_deviation();
        Ok(())
    }

    pub fn set_quote_size(&mut self, quote_size_usdc_native: u64) -> Result<()> {
        require!(
            quote_size_usdc_native > 1_000_000,
            OracleError::InvalidOracleQuoteSize
        );
        self.quote_size_usdc_native = quote_size_usdc_native;
        Ok(())
    }

    fn calc_std_deviation(&self) -> f64 {
        let prices: Vec<f64> = self
            .recent_prices
            .iter()
            .filter(|p| *p > &0.0)
            .cloned()
            .collect();
        if prices.len() == 0 {
            return 0.0;
        }
        let mean = prices.iter().sum::<f64>() / (prices.len() as f64);
        let variance: f64 = prices
            .iter()
            .map(|p| {
                let diff = p - mean;
                diff * diff
            })
            .sum();

        let std_dev = variance.sqrt();
        std_dev
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_oracle_data_update_price() {
        let mut oracle = OracleData::default();
        let slot = 25u64;

        for i in 1..101 {
            let new_price = (i * 25) as f64;
            oracle.update(new_price, slot + i as u64).unwrap();

            let px = oracle.price;
            let sd = oracle.standard_deviation;
            let index = oracle.recent_price_index;
            let oracle_slot = oracle.last_update_slot;

            assert_eq!(px, new_price);
            assert_eq!(index, (i as u8) % 16);
            assert_eq!(oracle_slot, slot + i as u64);
        }
    }
}
