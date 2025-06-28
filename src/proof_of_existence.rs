use crate::support::DispatchResult;
use core::fmt::Debug;
use std::collections::BTreeMap;

pub trait Config: crate::system::Config {
	// The type which represents the content that can be claimed using this pallet.
	// Could be the content directly as bytes, or better yet the hash of that content.
	// We leave that decision to the runtime developer.
	type Content: Debug + Ord;
}

// This is the Proof of Existence Module.
// It is a simple module that allows accounts to claim existence of some data.
#[derive(Debug)]
pub struct Pallet<T: Config> {
	// A simple storage map from content to the owner of that content.
	// Accounts can make multiple different claims (contents), but each claim can only have one
	// owner.
	claims: BTreeMap<T::Content, T::AccountId>,
}

// The macros below replaces Call enum and impl block for Dispatch
#[macros::call]
impl<T: Config> Pallet<T> {
	// Create a new claim on behalf of the `caller`.
	// This function will return an error if someone already has claimed that content.
	fn create_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
		if self.claims.contains_key(&claim) {
			return Err(&"This content is already claimed");
		}
		self.claims.insert(claim, caller);
		Ok(())
	}

	fn revoke_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
		let _owner = self.get_claim(&claim).ok_or("Claim does not exist")?;
		if _owner != &caller {
			return Err(&"The caller is not the owner of the claim");
		}
		self.claims.remove(&claim);
		Ok(())
	}
}

impl<T: Config> Pallet<T> {
	// Create a new instance of the Proof of Existence Module.
	pub fn new() -> Self {
		Self { claims: BTreeMap::new() }
	}

	// Get the owner (if any) of a claim
	fn get_claim(&self, claim: &T::Content) -> Option<&T::AccountId> {
		self.claims.get(claim)
	}
}

mod test {
	struct TestConfig;
	impl super::Config for TestConfig {
		type Content = &'static str;
	}

	impl crate::system::Config for TestConfig {
		type BlockNumber = u32;
		type Nonce = u32;
		type AccountId = &'static str;
	}

	#[test]
	fn basic_proof_of_existence() {
		// Instantiate PoE module
		let mut poe = super::Pallet::<TestConfig>::new();

		// Create a claim from alice
		let _ = poe.create_claim(&"alice", "document");

		// Compare if claim is valid
		assert_eq!(poe.get_claim(&"document"), Some(&"alice"));

		// Revoke alice's claim using bob account
		let res = poe.revoke_claim("bob", "document");
		assert_eq!(res, Err("The caller is not the owner of the claim"));

		// Revoke non-existent claim
		let res2 = poe.revoke_claim("alice", "document_2");
		assert_eq!(res2, Err("Claim does not exist"));

		// Revoke properly using alice's claim
		let res3 = poe.revoke_claim("alice", "document");
		assert_eq!(res3, Ok(()));
		assert_eq!(poe.get_claim(&"document"), None);
	}
}
