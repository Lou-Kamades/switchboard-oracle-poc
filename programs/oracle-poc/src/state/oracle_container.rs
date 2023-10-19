use anchor_lang::prelude::*;
use static_assertions::const_assert_eq;
use std::mem::size_of;

use crate::{name_from_str, OracleError};

use super::OracleData;

#[repr(C)]
#[account(zero_copy(unsafe))]
#[derive(Default, AnchorSerialize, AnchorDeserialize)]
pub struct OracleContainer {
    pub bump: u8,
    pub oracles: [OracleData; 16],
    pub num_oracles: u8,
    pub padding: [u8; 6],
}
const_assert_eq!(
    size_of::<OracleContainer>(),
    size_of::<OracleData>() * 16 + 16
);
const_assert_eq!(size_of::<OracleContainer>(), 2832); // PDA limit is 10KB, we can go up to 10MB if we use an on-curve account.
const_assert_eq!(size_of::<OracleContainer>() % 8, 0);

impl OracleContainer {
    pub fn add_oracle(&mut self, oracle_name: &str) -> Result<()> {
        let index = self.num_oracles;
        let oracle_name_bytes = name_from_str(oracle_name)?;
        require!(index < 15, OracleError::OracleContainerFull);
        require!(
            self.oracle_is_new(&oracle_name_bytes),
            OracleError::OracleAlreadyAdded
        );

        self.oracles[index as usize].name = oracle_name_bytes;
        self.num_oracles += 1;
        Ok(())
    }

    fn oracle_is_new(&self, oracle_name_bytes: &[u8; 16]) -> bool {
        for oracle in self.oracles {
            if &oracle.name == oracle_name_bytes {
                return false;
            }
        }
        return true;
    }

    // fn remove_oracle(&mut self) -> Result<()> {
    //     todo!();
    //     Ok(())
    // }

    pub fn update_oracle(&mut self, oracle_name: &str, new_price: f64, std_deviation: f64, slot: u64) -> Result<()> {
        let oracle_name_bytes = name_from_str(oracle_name)?;
        let oracle = self
            .oracles
            .iter_mut()
            .find(|o| o.name == oracle_name_bytes)
            .ok_or(OracleError::OracleNotFound)?;
        oracle.update(new_price, std_deviation, slot)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_add_oracle() {
        let mut oracle_container = OracleContainer::default();
        let name = "TEST".to_string();

        oracle_container.add_oracle(&name).unwrap();
        assert_eq!(
            oracle_container.oracles[0].name,
            name_from_str(&name).unwrap()
        );
    }

    #[test]
    pub fn test_add_oracle_already_added() {
        let mut oracle_container = OracleContainer::default();
        let name = "TEST".to_string();

        oracle_container.add_oracle(&name).unwrap();
        let res = oracle_container.add_oracle(&name);

        assert!(res.is_err());
        assert_eq!(res.err().unwrap(), OracleError::OracleAlreadyAdded.into());
    }

    #[test]
    pub fn test_add_oracle_container_full() {
        let mut oracle_container = OracleContainer::default();
        let a = "a".to_string();
        let b = "b".to_string();
        let c = "c".to_string();
        let d = "d".to_string();
        let e = "e".to_string();
        let f = "f".to_string();
        let g = "g".to_string();
        let h = "h".to_string();
        let i = "i".to_string();
        let j = "j".to_string();
        let k = "k".to_string();
        let l = "l".to_string();
        let m = "m".to_string();
        let n = "n".to_string();
        let o = "o".to_string();
        let p = "p".to_string();

        oracle_container.add_oracle(&a).unwrap();
        oracle_container.add_oracle(&b).unwrap();
        oracle_container.add_oracle(&c).unwrap();
        oracle_container.add_oracle(&d).unwrap();
        oracle_container.add_oracle(&e).unwrap();
        oracle_container.add_oracle(&f).unwrap();
        oracle_container.add_oracle(&g).unwrap();
        oracle_container.add_oracle(&h).unwrap();
        oracle_container.add_oracle(&i).unwrap();
        oracle_container.add_oracle(&j).unwrap();
        oracle_container.add_oracle(&k).unwrap();
        oracle_container.add_oracle(&l).unwrap();
        oracle_container.add_oracle(&m).unwrap();
        oracle_container.add_oracle(&n).unwrap();
        oracle_container.add_oracle(&o).unwrap();
        let res = oracle_container.add_oracle(&p);

        assert!(res.is_err());
        assert_eq!(res.err().unwrap(), OracleError::OracleContainerFull.into());
    }
}
