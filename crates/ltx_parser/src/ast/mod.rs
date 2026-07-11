//! AST node types, each implementing [`crate::Parse`].
//!
//! Currently includes:
//! - [`Command`] — a control sequence with its braced arguments.
//! - [`Group`] — a balanced `{ … }` group.
//! - [`Text`] — a run of plain text.

pub mod command;
pub mod group;
pub mod text;

pub use command::Command;
pub use group::Group;
pub use text::Text;
