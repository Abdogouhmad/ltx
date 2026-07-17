//! AST node types, each implementing [`crate::Parse`].
//!
//! Currently includes:
//! - [`Command`] — a control sequence with its braced arguments.
//! - [`Group`] — a balanced `{ … }` group.
//! - [`Text`] — a run of plain text.
//! - [`Comment`] - Handling comment
//! - [`Math`] - Handling Math
pub mod command;
pub mod comment;
pub mod group;
pub mod math;
pub mod text;

pub use command::Command;
pub use comment::Comment;
pub use group::Group;
pub use text::Text;
pub use math::Math;
