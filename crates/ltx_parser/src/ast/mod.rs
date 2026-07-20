//! AST node types, each implementing [`crate::Parse`].
//!
//! Currently includes:
//! - [`Command`] — a control sequence with its braced arguments.
//! - [`Group`] — a balanced `{ … }` group.
//! - [`Text`] — a run of plain text.
//! - [`Comment`] - a LaTeX comment.
//! - [`Math`] - a LaTeX math expression.
//! - [`Environment`] - a LaTeX environment.
pub mod command;
pub mod comment;
pub mod environment;
pub mod group;
pub mod math;
pub mod text;

pub use command::Command;
pub use comment::Comment;
pub use environment::Environment;
pub use group::Group;
pub use math::Math;
pub use text::Text;
