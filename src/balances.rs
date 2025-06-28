use std::collections::BTreeMap;

use num::traits::{CheckedAdd, CheckedSub, Zero};

pub trait Config: crate::system::Config {
	type Balance: Zero + CheckedSub + CheckedAdd + Copy;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
	balances: BTreeMap<T::AccountId, T::Balance>,
}

// The macros below replaces Call enum and impl blocks for Dispatch
#[macros::call]
impl<T: Config> Pallet<T> {
	pub fn transfer(
		&mut self,
		caller: T::AccountId,
		to: T::AccountId,
		amount: T::Balance,
	) -> Result<(), &'static str> {
		let zero = T::Balance::zero();
		let caller_balance = self.balances.get(&caller).unwrap_or(&zero);
		let to_balance = self.balances.get(&to).unwrap_or(&zero);

		let new_caller_balance = caller_balance.checked_sub(&amount).ok_or("Not enough funds.")?;
		let new_to_balance =
			to_balance.checked_add(&amount).ok_or("Overflow when adding to balance")?;

		self.balances.insert(caller.clone(), new_caller_balance);
		self.balances.insert(to.clone(), new_to_balance);

		Ok(())
	}
}

impl<T: Config> Pallet<T> {
	pub fn new() -> Self {
		Self { balances: BTreeMap::new() }
	}

	pub fn set_balance(&mut self, who: &T::AccountId, amount: T::Balance) {
		self.balances.insert(who.clone(), amount);
	}

	pub fn balance(&self, who: &T::AccountId) -> T::Balance {
		*self.balances.get(who).unwrap_or(&T::Balance::zero())
	}
}

mod tests {
	use crate::system;
	struct TestConfig;

	impl system::Config for TestConfig {
		type AccountId = String;
		type BlockNumber = u32;
		type Nonce = u32;
	}

	impl super::Config for TestConfig {
		type Balance = u128;
	}

	#[test]
	fn init_balances() {
		let mut balances = super::Pallet::<TestConfig>::new();

		assert_eq!(balances.balance(&"alice".to_string()), 0);
		balances.set_balance(&"alice".to_string(), 100);
		assert_eq!(balances.balance(&"alice".to_string()), 100);
		assert_eq!(balances.balance(&"bob".to_string()), 0);
	}

	#[test]
	fn transfer_balance() {
		let mut balances = super::Pallet::<TestConfig>::new();

		let alice = "alice".to_string();
		let bob = "bob".to_string();

		balances.set_balance(&alice, 100);
		assert_eq!(balances.balance(&alice), 100);
		assert_eq!(balances.transfer(alice.clone(), bob.clone(), 10), Ok(()));
		assert_eq!(balances.balance(&alice), 90);
		assert_eq!(balances.balance(&bob), 10);
	}

	#[test]
	fn transfer_balance_insufficient_fund() {
		let mut balances = super::Pallet::<TestConfig>::new();

		let alice = "alice".to_string();
		let bob = "bob".to_string();

		balances.set_balance(&alice, 100);
		assert_eq!(balances.balance(&alice), 100);
		assert_eq!(balances.transfer(alice.clone(), bob.clone(), 110), Err("Not enough funds."));
		assert_eq!(balances.balance(&alice), 100);
		assert_eq!(balances.balance(&bob), 0);
	}

	#[test]
	fn transfer_balance_overflow() {
		let mut balances = super::Pallet::<TestConfig>::new();

		let alice = "alice".to_string();
		let bob = "bob".to_string();

		balances.set_balance(&alice, 100);
		balances.set_balance(&bob, u128::MAX);
		assert_eq!(balances.balance(&alice), 100);
		assert_eq!(balances.balance(&bob), u128::MAX);
		assert_eq!(
			balances.transfer(alice.clone(), bob.clone(), 10),
			Err("Overflow when adding to balance")
		);
		assert_eq!(balances.balance(&alice), 100);
		assert_eq!(balances.balance(&bob), u128::MAX);
	}
}
