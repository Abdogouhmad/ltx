//! AST node types, each implementing [`crate::Parse`].
//!
//! Includes:
//! - [`Document`] — top-level root node
//! - [`PreambleItem`] — preamble items (`\documentclass`, `\usepackage`, etc.)
//! - [`DocumentBodyNode`] — document body / environment items
//! - [`DocumentClassDecl`] — `\documentclass` declaration
//! - [`UsePackage`] — `\usepackage` declaration
//! - [`Command`] — control sequence and its arguments
//! - [`Arg`], [`OptionalArg`] — argument variants
//! - [`Environment`] — LaTeX environment (`\begin{...}...\end{...}`)
//! - [`Group`] — balanced `{...}` group
//! - [`Math`] — math expression
//! - [`Text`] — plain text run
//! - [`Comment`] — LaTeX comment

pub mod arg;
pub mod body_node;
pub mod command;
pub mod comment;
pub mod document;
pub mod document_class;
pub mod environment;
pub mod group;
pub mod math;
pub mod preamble;
pub mod text;
pub mod use_package;

pub use arg::{Arg, OptionalArg};
pub use body_node::DocumentBodyNode;
pub use command::Command;
pub use comment::Comment;
pub use document::Document;
pub use document_class::DocumentClassDecl;
pub use environment::Environment;
pub use group::Group;
pub use math::Math;
pub use preamble::PreambleItem;
pub use text::Text;
pub use use_package::UsePackage;
