mod balances;
mod support;
mod system;
mod proof_of_existence;
mod types {
	pub type AccountId = String;
	pub type Balance = u128;
	pub type BlockNumber = u32;
	pub type Nonce = u32;
	pub type Extrinsic = crate::support::Extrinsic<AccountId, crate::RuntimeCall>;
	pub type Header = crate::support::Header<BlockNumber>;
	pub type Block = crate::support::Block<Header, Extrinsic>;
    pub type Content = &'static str;
}
// This is our main Runtime.
// It accumulates all of the different pallets we want to use.

use crate::support::Dispatch;

#[derive(Debug)]
#[macros::runtime]
pub struct Runtime {
	system: system::Pallet<Self>,
	balances: balances::Pallet<Self>,
    proof_of_existence: proof_of_existence::Pallet<Self>,
}

// pub enum RuntimeCall {
// 	Balances(balances::Call<Runtime>),
//     ProofOfExistence(proof_of_existence::Call<Runtime>),
// }

impl system::Config for Runtime {
	type AccountId = types::AccountId;
	type BlockNumber = types::BlockNumber;
	type Nonce = types::Nonce;
}

impl balances::Config for Runtime {
	type Balance = types::Balance;
}

impl proof_of_existence::Config for Runtime {
	type Content = types::Content;
}

// impl Runtime {
// 	// Create a new instance of the main Runtime, by creating a new instance of each pallet.
// 	fn new() -> Self {
// 		Self { system: system::Pallet::new(), balances: balances::Pallet::new(), proof_of_existence: proof_of_existence::Pallet::new(),}
// 	}

// 	fn execute_block(&mut self, block: types::Block) -> support::DispatchResult {
// 		self.system.inc_block_number();
// 		if block.header.block_number != self.system.block_number() {
// 			return Err("block number does not match what is expected");
// 		}

// 		for (i, support::Extrinsic { caller, call }) in block.extrinsics.into_iter().enumerate() {
// 			self.system.inc_nonce(&caller);
// 			let _res = self.dispatch(caller, call).map_err(|e| {
// 				eprintln!(
// 					"Extrinsic Error\n\tBlock Number: {}\n\tExtrinsic Number: {}\n\tError: {}",
// 					block.header.block_number, i, e
// 				)
// 			});
// 		}
// 		Ok(())
// 	}
// }

// impl crate::support::Dispatch for Runtime {
// 	type Caller = <Runtime as system::Config>::AccountId;
// 	type Call = RuntimeCall;

// 	fn dispatch(
// 		&mut self,
// 		caller: Self::Caller,
// 		runtime_call: Self::Call,
// 	) -> support::DispatchResult {
// 		match runtime_call {
// 		RuntimeCall::Balances(call) => {
// 				self.balances.dispatch(caller, call)?;
// 			},
//             RuntimeCall::ProofOfExistence(call) => {
// 				self.proof_of_existence.dispatch(caller, call)?;
// 			},
// 		}
// 		Ok(())
// 	}
// }

fn main() {
	/* TODO: Create a mutable variable `runtime`, which is a new instance of `Runtime`. */
	let mut runtime = Runtime::new();
	/* TODO: Set the balance of `alice` to 100, allowing us to execute other transactions. */
	runtime.balances.set_balance(&"alice".to_string(), 100);

	// // start emulating a block
	// /* TODO: Increment the block number in system. */
	// runtime.system.inc_block_number();
	// /* TODO: Assert the block number is what we expect. */

	// assert_eq!(runtime.system.block_number(), 1);

	// // first transaction
	// /* TODO: Increment the nonce of `alice`. */
	// runtime.system.inc_nonce(&"alice".to_string());
	// /* TODO: Execute a transfer from `alice` to `bob` for 30 tokens.
	// 	- The transfer _could_ return an error. We should use `map_err` to print
	// 	  the error if there is one.
	// 	- We should capture the result of the transfer in an unused variable like `_res`.
	// */

	// let _ = runtime.balances.transfer("alice".to_string(), "bob".to_string(), 30);

	// // second transaction
	// /* TODO: Increment the nonce of `alice` again. */
	// runtime.system.inc_nonce(&"alice".to_string());
	// /* TODO: Execute another balance transfer, this time from `alice` to `charlie` for 20. */
	// let _ = runtime.balances.transfer("alice".to_string(), "charlie".to_string(), 20);

	// let alice_balance = runtime.balances.balance(&"alice".to_string());

    let alice = "alice".to_string();
	let bob = "bob".to_string();
	let charlie = "charlie".to_string();

    let block_1 = types::Block {
		header: support::Header { block_number: 1 },
		extrinsics: vec![
			support::Extrinsic {
				caller: alice.clone(),
				call: RuntimeCall::balances(balances::Call::transfer { to: bob.clone(), amount: 30 }),
			},
			support::Extrinsic {
				caller: alice.clone(),
				call: RuntimeCall::balances(balances::Call::transfer { to: charlie.clone(), amount: 20 }),
			},
		],
	};

    let block_2 = types::Block {
		header: support::Header { block_number: 2 },
		extrinsics: vec![
			support::Extrinsic {
				caller: alice.clone(),
				call: RuntimeCall::proof_of_existence(proof_of_existence::Call::create_claim {
					claim: "Hello, world!",
				}),
			},
			support::Extrinsic {
				caller: bob.clone(),
				call: RuntimeCall::proof_of_existence(proof_of_existence::Call::create_claim {
					claim: "Hello, world!",
				}),
			},
		],
	};

	let block_3 = types::Block {
		header: support::Header { block_number: 3 },
		extrinsics: vec![
			support::Extrinsic {
				caller: alice,
				call: RuntimeCall::proof_of_existence(proof_of_existence::Call::revoke_claim {
					claim: "Hello, world!",
				}),
			},
			support::Extrinsic {
				caller: bob,
				call: RuntimeCall::proof_of_existence(proof_of_existence::Call::create_claim {
					claim: "Hello, world!",
				}),
			},
		],
	};


	
	
	runtime.execute_block(block_1).expect("invalid block");
    runtime.execute_block(block_2).expect("invalid block");
	runtime.execute_block(block_3).expect("invalid block");
	println!("Runtime state {:#?}", runtime);
}
