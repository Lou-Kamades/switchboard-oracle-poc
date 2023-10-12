use anchor_lang::prelude::*;
use static_assertions::const_assert_eq;
use std::mem::size_of;

#[repr(C)]
#[derive(Debug, Default, AnchorSerialize, AnchorDeserialize, Copy, Clone)]
pub struct OracleData {
    pub last_update_slot: u64,
    pub price: i128,
    pub recent_prices: [i128; 16],
    pub name: [u8; 16],
    pub recent_price_index: u8,
    pub padding: [u8; 7],
}
const_assert_eq!(size_of::<OracleData>(), 8 + 16 + 16 + 256 + 8);
const_assert_eq!(size_of::<OracleData>(), 304);
const_assert_eq!(size_of::<OracleData>() % 8, 0);

impl OracleData {
    pub fn update(&mut self, new_price: i128, slot: u64) -> Result<()> {
        self.price = new_price;
        self.last_update_slot = slot;
        self.recent_price_index = (self.recent_price_index + 1) % 16;
        self.recent_prices[self.recent_price_index as usize] = new_price;
        Ok(())
    }

    pub fn deviation(&self) -> i128 {
        let prices = self.recent_prices;
        let max = prices.iter().max().unwrap_or_else(|| &0);
        let min = prices.iter().min().unwrap_or_else(|| &0);
        max - min
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_oracle_data_update_price() {
        let mut oracle = OracleData::default();
        let slot = 25u64;

        for i in 0..100 {
            let new_price = i * 25;
            oracle.update(new_price, slot + i as u64).unwrap();

            let px = oracle.price;
            let index = oracle.recent_price_index;
            let oracle_slot = oracle.last_update_slot;

            assert_eq!(px, new_price);
            assert_eq!(index, (i as u8 + 1) % 16);
            assert_eq!(oracle_slot, slot + i as u64);
        }
    }
}
