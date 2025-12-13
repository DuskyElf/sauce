pub mod checker;
pub mod errors;
pub mod types;
pub use checker::{typecheck_expr, typecheck_program, typecheck_stmt};
pub use errors::TypeError;
pub use types::Type;
