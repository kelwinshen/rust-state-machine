use core::fmt::Debug;
use std::collections::BTreeMap;

use crate::support::DispatchResult;

pub trait Config: crate::system::Config {
	/// The type which represents the content that can be claimed using this pallet.
	/// Could be the content directly as bytes, or better yet the hash of that content.
	/// We leave that decision to the runtime developer.
	type Content: Debug + Ord;
}

/// This is the Proof of Existence Module.
/// It is a simple module that allows accounts to claim existence of some data.
#[derive(Debug)]
pub struct Pallet<T: Config> {
	/// A simple storage map from content to the owner of that content.
	/// Accounts can make multiple different claims, but each claim can only have one owner.
	/* TODO: Add a field `claims` which is a `BTreeMap` fom `T::Content` to `T::AccountId`. */
    claims: BTreeMap<T::Content, T::AccountId>
    
}

impl<T: Config> Pallet<T> {
	
	pub fn new() -> Self {
		Self { claims: BTreeMap::new() }
	}

	
	pub fn get_claim(&self, claim: &T::Content) -> Option<&T::AccountId> {
		self.claims.get(claim)
	}
}


#[macros::call]
impl<T: Config> Pallet<T> {
	
	
	pub fn create_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
		if self.claims.contains_key(&claim) {
			return Err("this content is already claimed");
		}
		self.claims.insert(claim, caller);
		Ok(())
	}

	
	
	
	pub fn revoke_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
		let owner = self.get_claim(&claim).ok_or("claim does not exist")?;
		if caller != *owner {
			return Err("this content is owned by someone else");
		}
		self.claims.remove(&claim);
		Ok(())
	}
}

// impl<T: Config> Pallet<T> {
// 	/// Create a new instance of the Proof of Existence Module.
// 	pub fn new() -> Self {
// 		/* TODO: Return a new instance of the `Pallet` struct. */
//         Self{
//             claims: BTreeMap::new()
//         }
// 	}

//     pub fn get_claim(&self, claim: &T::Content) -> Option<&T::AccountId> {
// 		self.claims.get(claim)
// 	}


//     pub fn create_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
// 		if self.claims.contains_key(&claim) {
// 			return Err("this content is already claimed");
// 		}
// 		self.claims.insert(claim, caller);
// 		Ok(())
// 	}

//     pub fn revoke_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
// 		let owner = self.get_claim(&claim).ok_or("claim does not exist")?;
// 		if caller != *owner {
// 			return Err("this content is owned by someone else");
// 		}
// 		self.claims.remove(&claim);
// 		Ok(())
// 	}

// }



// pub enum Call<T: Config> {
// 	CreateClaim { claim: T::Content },
// 	RevokeClaim { claim: T::Content },
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
// 			Call::CreateClaim { claim } => {
// 				self.create_claim(caller, claim)?;
// 			},
// 			Call::RevokeClaim { claim } => {
// 				self.revoke_claim(caller, claim)?;
// 			},
// 		}
// 		Ok(())
// 	}
// }


#[cfg(test)]
mod test {
	struct TestConfig;

	impl super::Config for TestConfig {
		type Content = &'static str;
	}

	impl crate::system::Config for TestConfig {
		type AccountId = &'static str;
		type BlockNumber = u32;
		type Nonce = u32;
	}

	#[test]
	fn basic_proof_of_existence() {
		let mut poe = super::Pallet::<TestConfig>::new();
		assert_eq!(poe.get_claim(&"Hello, world!"), None);
		assert_eq!(poe.create_claim("alice", "Hello, world!"), Ok(()));
		assert_eq!(poe.get_claim(&"Hello, world!"), Some(&"alice"));
		assert_eq!(
			poe.create_claim("bob", "Hello, world!"),
			Err("this content is already claimed")
		);
		assert_eq!(poe.revoke_claim("alice", "Hello, world!"), Ok(()));
		assert_eq!(poe.create_claim("bob", "Hello, world!"), Ok(()));
	}
}