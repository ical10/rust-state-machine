// a simplified version of header,
// on a real blockchain we would find:
// - parent block hash
// - state root
// - extrinsic root
// etc
pub struct Header<BlockNumber> {
	pub block_number: BlockNumber,
}

// an "extrinsic" is an external message from outside of the blockchain.
// it tells us who is making the call, and which call they are making
pub struct Extrinsic<Caller, Call> {
	pub call: Call,
	pub caller: Caller,
}

pub struct Block<Header, Extrinsic> {
	// The block header contains metadata about the block.
	pub header: Header,
	// The extrinsics represents the state transitions to be executed in this block.
	pub extrinsics: Vec<Extrinsic>,
}

// The Result type of our runtime. When everything completes successfully, we return `Ok(())`,
// otherwise return a static error message.
pub type DispatchResult = Result<(), &'static str>;

// A trait which allows us to dispatch an incoming extrinsic to the appropriate state transition
// function call.
pub trait Dispatch {
	// The type used to identify the caller of the function.
	type Caller;
	// The state transition function call the caller is trying to access.
	type Call;

	// This function takes a `caller` and the `call` they want to make, and returns a `Result`
	// based on the outcome of that function call.
	fn dispatch(&mut self, caller: Self::Caller, call: Self::Call) -> DispatchResult;
}
