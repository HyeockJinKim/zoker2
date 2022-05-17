pub use contract::Contract;
pub use function::Func;

use std::collections::HashMap;
use std::sync::Arc;
use crate::generator::Generator;
use crate::operation::{Context, Operation};

mod contract;
mod function;
