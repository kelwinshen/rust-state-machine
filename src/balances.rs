use num::traits::{CheckedAdd, CheckedSub, Zero};
use std::collections::BTreeMap;

pub trait Config: crate::system::Config {
	type Balance: Zero + CheckedSub + CheckedAdd + Copy;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
	balances: BTreeMap<T::AccountId, T::Balance>,
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

#[macros::call]
impl<T: Config> Pallet<T> {
	
	
	
	pub fn transfer(
		&mut self,
		caller: T::AccountId,
		to: T::AccountId,
		amount: T::Balance,
	) -> crate::support::DispatchResult {
		let caller_balance = self.balance(&caller);
		let to_balance = self.balance(&to);

		let new_caller_balance = caller_balance.checked_sub(&amount).ok_or("Not enough funds.")?;
		let new_to_balance = to_balance.checked_add(&amount).ok_or("Overflow")?;

		self.balances.insert(caller, new_caller_balance);
		self.balances.insert(to, new_to_balance);

		Ok(())
	}
}

// pub enum Call<T: Config> {
// 	Transfer { to: T::AccountId, amount: T::Balance },
// }


// impl<T: Config> crate::support::Dispatch for Pallet<T> {
// 	type Caller = T::AccountId;
// 	type Call = Call<T>;

// 	fn dispatch(
// 		&mut self,
// 		caller: Self::Caller,
// 		call: Self::Call,
// 	) -> crate::support::DispatchResult {
// 		match call {
// 			Call::Transfer { to, amount } => {
// 				self.transfer(caller, to, amount)?;
// 			},
// 		}
// 		Ok(())
// 	}
// }

#[cfg(test)]
mod tests {
	struct TestConfig;

	impl crate::system::Config for TestConfig {
		type AccountId = String;
		type BlockNumber = u32;
		type Nonce = u32;
	}

	impl super::Config for TestConfig {
		type Balance = u128;
	}

	#[test]
	fn init_balances() {
		/* TODO: Create a mutable variable `balances`, which is a new instance of `Pallet`. */
		let mut balances = super::Pallet::<TestConfig>::new();
		/* TODO: Assert that the balance of `alice` starts at zero. */
		assert_eq!(balances.balance(&"alice".to_string()), 0);
		/* TODO: Set the balance of `alice` to 100. */
		balances.set_balance(&"alice".to_string(), 100);
		/* TODO: Assert the balance of `alice` is now 100. */
		assert_eq!(balances.balance(&"alice".to_string()), 100);
		/* TODO: Assert the balance of `bob` has not changed and is 0. */
		assert_eq!(balances.balance(&"bob".to_string()), 0);
	}

	#[test]
	fn transfer_balance() {
		let mut balances = super::Pallet::<TestConfig>::new();

		/* TODO: Create a test that checks the following:
			- That `alice` cannot transfer funds she does not have.
			- That `alice` can successfully transfer funds to `bob`.
			- That the balance of `alice` and `bob` is correctly updated.

		*/

		balances.set_balance(&"alice".to_string(), 100);

		assert_eq!(
			balances.transfer(String::from("alice"), String::from("bob"), 200),
			Err("Not enough fund.")
		);

		assert_eq!(balances.transfer(String::from("alice"), String::from("bob"), 100), Ok(()));

		assert_eq!(balances.balance(&"alice".to_string()), 0);

		assert_eq!(balances.balance(&"bob".to_string()), 100);
	}
}
