mod balances;
mod proof_of_existence;
mod support;
mod system;
use crate::support::Dispatch;

mod types {
	use crate::support;

	pub type AccountId = String;
	pub type Balance = u128;
	pub type BlockNumber = u32;
	pub type Nonce = u32;
	pub type Extrinsic = support::Extrinsic<AccountId, crate::RuntimeCall>;
	pub type Header = support::Header<BlockNumber>;
	pub type Block = support::Block<Header, Extrinsic>;
	pub type Content = &'static str;
}

#[derive(Debug)]
#[macros::runtime]
//The macros above replaces impl Runtime block, enum RuntimeCall, and impl block for Dispatch
pub struct Runtime {
	system: system::Pallet<Self>,
	balances: balances::Pallet<Self>,
	proof_of_existence: proof_of_existence::Pallet<Self>,
}

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

fn main() {
	let mut runtime = Runtime::new();
	let alice = String::from("alice");
	let bob = String::from("bob");
	let charlie = String::from("charlie");
	runtime.balances.set_balance(&alice, 100);

	let block_1 = types::Block {
		header: support::Header { block_number: 1 },
		extrinsics: vec![
			support::Extrinsic {
				caller: alice.clone(),
				call: RuntimeCall::balances(balances::Call::transfer {
					to: bob.clone(),
					amount: 20,
				}),
			},
			support::Extrinsic {
				caller: alice.clone(),
				call: RuntimeCall::balances(balances::Call::transfer { to: charlie, amount: 50 }),
			},
			support::Extrinsic {
				caller: alice,
				call: RuntimeCall::proof_of_existence(proof_of_existence::Call::create_claim {
					claim: "document",
				}),
			},
		],
	};

	let block_2 = types::Block {
		header: support::Header { block_number: 2 },
		extrinsics: vec![support::Extrinsic {
			caller: bob,
			call: RuntimeCall::proof_of_existence(proof_of_existence::Call::create_claim {
				claim: "document",
			}),
		}],
	};

	let _ = runtime.execute_block(block_1).expect("invalid block");
	let _ = runtime.execute_block(block_2).expect("invalid block");

	println!("{:#?}", runtime);
}
