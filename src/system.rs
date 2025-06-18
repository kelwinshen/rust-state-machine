use num::traits::{One, Zero};
use std::{collections::BTreeMap, ops::AddAssign};
/// This is the System Pallet.
/// It handles low level state needed for your blockchain.

pub trait Config {
	type AccountId: Ord + Clone;

	type BlockNumber: Zero + One + AddAssign + Copy;

	type Nonce: Zero + One + Copy;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
	/// The current block number.
	block_number: T::BlockNumber,
	/// A map from an account to their nonce.
	nonce: BTreeMap<T::AccountId, T::Nonce>,
}

impl<T: Config> Pallet<T>
where
	T::AccountId: Ord + Clone,
	T::BlockNumber: Zero + One + AddAssign + Copy,
	T::Nonce: Zero + One + Copy,
{
	/// Create a new instance of the System Pallet.
	pub fn new() -> Self {
		Self { block_number: T::BlockNumber::zero(), nonce: BTreeMap::new() }
	}

	/// Get the current block number.
	pub fn block_number(&self) -> T::BlockNumber {
		/* TODO: Return the current block number. */
		self.block_number
	}

	// This function can be used to increment the block number.
	// Increases the block number by one.
	pub fn inc_block_number(&mut self) {
		/* TODO: Increment the current block number by one. */
		self.block_number += T::BlockNumber::one();
	}

	// Increment the nonce of an account. This helps us keep track of how many transactions each
	// account has made.
	pub fn inc_nonce(&mut self, who: &T::AccountId) {
		/* TODO: Get the current nonce of `who`, and increment it by one. */
		let nonce = *self.nonce.get(who).unwrap_or(&T::Nonce::zero());
		let new_nonce = nonce + T::Nonce::one();
		self.nonce.insert(who.clone(), new_nonce);
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
		let mut system = super::Pallet::<TestConfig>::new();
		/* TODO: Create a test which checks the following:

			- Increment the current block number.

			- Increment the nonce of `alice`.

			- Check the block number is what we expect.
			- Check the nonce of `alice` is what we expect.
			- Check the nonce of `bob` is what we expect.
		*/

		system.inc_block_number();

		system.inc_nonce(&String::from("alice"));

		assert_eq!(system.block_number(), 1);

		assert_eq!(system.nonce.get("alice").unwrap(), &1);

		assert_eq!(system.nonce.get("bob").unwrap_or(&0), &0);
	}
}
