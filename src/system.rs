use std::collections::BTreeMap;
use std::ops::AddAssign;

use num::traits::{One, Zero};

pub trait Config {
	type AccountId: Ord + Clone;
	type BlockNumber: Zero + AddAssign + Copy + One;
	type Nonce: Zero + One + Copy;
}

/// This is the System Pallet.
/// It handles low level state needed for your blockchain.
#[derive(Debug)]
pub struct Pallet<T: Config> {
	/// The current block number.
	block_number: T::BlockNumber,
	/// A map from an account to their nonce.
	nonce: BTreeMap<T::AccountId, T::Nonce>,
}

impl<T: Config> Pallet<T> {
	/// Create a new instance of the System Pallet.
	pub fn new() -> Self {
		Self { block_number: T::BlockNumber::zero(), nonce: BTreeMap::new() }
	}

	/// Get the current block number.
	pub fn block_number(&self) -> T::BlockNumber {
		self.block_number
	}

	// This function can be used to increment the block number.
	// Increases the block number by one.
	pub fn inc_block_number(&mut self) {
		self.block_number += T::BlockNumber::one();
	}

	// Get the nonce of an AccountID
	pub fn nonce(&self, who: &T::AccountId) -> T::Nonce {
		*self.nonce.get(who).unwrap_or(&T::Nonce::zero())
	}

	// Increment the nonce of an account. This helps us keep track of how many transactions each
	// account has made.
	pub fn inc_nonce(&mut self, who: &T::AccountId) {
		let _nonce = *self.nonce.get(who).unwrap_or(&T::Nonce::zero());
		self.nonce.insert(who.clone(), _nonce + T::Nonce::one());
	}
}

#[cfg(test)]
mod test {
	struct TestConfig;

	impl super::Config for TestConfig {
		type AccountId = String;
		type BlockNumber = u32;
		type Nonce = u32;
	}

	#[test]
	fn init_system() {
		let system = super::Pallet::<TestConfig>::new();
		assert_eq!(system.block_number(), 0);
	}

	#[test]
	fn inc_block_number() {
		let mut system = super::Pallet::<TestConfig>::new();
		system.inc_block_number();
		assert_eq!(system.block_number(), 1);
	}

	#[test]
	fn check_nonce() {
		let system = super::Pallet::<TestConfig>::new();
		let alice = String::from("Alice");
		assert_eq!(system.nonce.get(&alice).unwrap_or(&0), &0);
	}

	#[test]
	fn inc_nonce() {
		let mut system = super::Pallet::<TestConfig>::new();
		let alice = String::from("Alice");
		system.inc_nonce(&alice);
		assert_eq!(system.nonce.get(&alice).unwrap_or(&0), &1);
	}
}
