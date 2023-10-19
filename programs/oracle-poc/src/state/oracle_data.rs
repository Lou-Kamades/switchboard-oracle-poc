use anchor_lang::prelude::*;
use static_assertions::const_assert_eq;
use std::mem::size_of;

#[repr(C)]
#[derive(Debug, Default, AnchorSerialize, AnchorDeserialize, Copy, Clone)]
pub struct OracleData {
    pub last_update_slot: u64,
    pub price: f64,
    pub standard_deviation: f64,
    pub recent_prices: [f64; 16],
    pub name: [u8; 16],
    pub recent_price_index: u8,
    pub padding: [u8; 7],
}
const_assert_eq!(size_of::<OracleData>(), 8 + 8 + 8 + 128 + 16 + 8);
const_assert_eq!(size_of::<OracleData>(), 176);
const_assert_eq!(size_of::<OracleData>() % 8, 0);

impl OracleData {
    pub fn update(&mut self, new_price: f64, std_deviation: f64, slot: u64) -> Result<()> {
        self.price = new_price;
        self.standard_deviation = std_deviation;
        self.last_update_slot = slot;
        self.recent_price_index = (self.recent_price_index + 1) % 16;
        self.recent_prices[self.recent_price_index as usize] = new_price;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_oracle_data_update_price() {
        let mut oracle = OracleData::default();
        let slot = 25u64;
        let std_dev = 0.05;

        for i in 0..100 {
            let new_price = (i * 25) as f64;
            oracle.update(new_price, std_dev, slot + i as u64).unwrap();

            let px = oracle.price;
            let sd = oracle.standard_deviation;
            let index = oracle.recent_price_index;
            let oracle_slot = oracle.last_update_slot;

            assert_eq!(px, new_price);
            assert_eq!(sd, std_dev);
            assert_eq!(index, (i as u8 + 1) % 16);
            assert_eq!(oracle_slot, slot + i as u64);
        }
    }
}
