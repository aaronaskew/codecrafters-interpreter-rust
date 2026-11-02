pub use interpret::*;
pub use parse::*;
pub use scan::*;

mod interpret;
mod parse;
mod scan;
mod token;

pub const SYNTAX_ERROR_EXIT_CODE: i32 = 65;
pub const RUNTIME_ERROR_EXIT_CODE: i32 = 70;
