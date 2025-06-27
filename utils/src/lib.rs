pub mod as_ref_and_into;
pub use as_ref_and_into::AsRefAndInto;

pub mod assign;
pub use assign::assign;

pub mod biggest_gap;
pub use biggest_gap::{biggest_gap, biggest_gap_per};

pub mod cum_sum;
pub use cum_sum::CumSum;

pub mod enum_map;

pub mod find_combination;
pub use find_combination::find_combination;

pub mod from;

pub mod hashmap;
pub use hashmap::GetVec;

pub mod inherit;

pub mod into_this;
pub use into_this::{IntoThis, TryIntoThis};

pub mod iter;

pub mod kronecker_delta;
pub use kronecker_delta::kron_delta;

pub mod number_theory;
pub use number_theory::get_divisors;

pub mod one_of;

pub mod option;
pub use option::OptionExt;

pub mod result;

pub mod time_stamp;
pub use time_stamp::{TimeStamp, TimeStampProvider, DiscreteTimeStamp, DiscreteTimeStampProvider};

