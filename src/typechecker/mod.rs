pub mod checker;
pub mod errors;
pub mod types;
pub use checker::{typecheck_program, typecheck_expr, typecheck_stmt};
pub use types::Type;
pub use errors::TypeError;
