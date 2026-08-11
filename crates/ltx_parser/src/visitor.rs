//! Visitor traits and default AST walk implementations for [`ltx_parser`].

use crate::ast::{
    Arg, Command, Comment, Document, DocumentBodyNode, DocumentClassDecl, Environment, Group, Math,
    PreambleItem, Text, UsePackage,
};

/// Immutable AST visitor trait.
pub trait Visitor<'ast> {
    /// Visit a [`Document`] root node.
    fn visit_document(&mut self, doc: &Document<'ast>) {
        walk_document(self, doc);
    }

    /// Visit a [`PreambleItem`].
    fn visit_preamble_item(&mut self, item: &PreambleItem<'ast>) {
        walk_preamble_item(self, item);
    }

    /// Visit a [`DocumentClassDecl`].
    fn visit_document_class(&mut self, doc_class: &DocumentClassDecl<'ast>) {
        let _ = doc_class;
    }

    /// Visit a [`UsePackage`].
    fn visit_use_package(&mut self, use_pkg: &UsePackage<'ast>) {
        let _ = use_pkg;
    }

    /// Visit a [`DocumentBodyNode`].
    fn visit_body_node(&mut self, node: &DocumentBodyNode<'ast>) {
        walk_body_node(self, node);
    }

    /// Visit a [`Command`].
    fn visit_command(&mut self, cmd: &Command<'ast>) {
        walk_command(self, cmd);
    }

    /// Visit an [`Environment`].
    fn visit_environment(&mut self, env: &Environment<'ast>) {
        walk_environment(self, env);
    }

    /// Visit a [`Group`].
    fn visit_group(&mut self, group: &Group<'ast>) {
        let _ = group;
    }

    /// Visit a [`Math`] expression.
    fn visit_math(&mut self, math: &Math<'ast>) {
        let _ = math;
    }

    /// Visit a [`Text`] run.
    fn visit_text(&mut self, text: &Text<'ast>) {
        let _ = text;
    }

    /// Visit a [`Comment`].
    fn visit_comment(&mut self, comment: &Comment<'ast>) {
        let _ = comment;
    }
}

/// Mutable AST visitor trait.
pub trait VisitorMut<'ast> {
    /// Visit a [`Document`] root node.
    fn visit_document(&mut self, doc: &mut Document<'ast>) {
        walk_document_mut(self, doc);
    }

    /// Visit a [`PreambleItem`].
    fn visit_preamble_item(&mut self, item: &mut PreambleItem<'ast>) {
        walk_preamble_item_mut(self, item);
    }

    /// Visit a [`DocumentClassDecl`].
    fn visit_document_class(&mut self, doc_class: &mut DocumentClassDecl<'ast>) {
        let _ = doc_class;
    }

    /// Visit a [`UsePackage`].
    fn visit_use_package(&mut self, use_pkg: &mut UsePackage<'ast>) {
        let _ = use_pkg;
    }

    /// Visit a [`DocumentBodyNode`].
    fn visit_body_node(&mut self, node: &mut DocumentBodyNode<'ast>) {
        walk_body_node_mut(self, node);
    }

    /// Visit a [`Command`].
    fn visit_command(&mut self, cmd: &mut Command<'ast>) {
        walk_command_mut(self, cmd);
    }

    /// Visit an [`Environment`].
    fn visit_environment(&mut self, env: &mut Environment<'ast>) {
        walk_environment_mut(self, env);
    }

    /// Visit a [`Group`].
    fn visit_group(&mut self, group: &mut Group<'ast>) {
        let _ = group;
    }

    /// Visit a [`Math`] expression.
    fn visit_math(&mut self, math: &mut Math<'ast>) {
        let _ = math;
    }

    /// Visit a [`Text`] run.
    fn visit_text(&mut self, text: &mut Text<'ast>) {
        let _ = text;
    }

    /// Visit a [`Comment`].
    fn visit_comment(&mut self, comment: &mut Comment<'ast>) {
        let _ = comment;
    }
}

/// Default walk function for [`Document`].
pub fn walk_document<'ast, V: Visitor<'ast> + ?Sized>(visitor: &mut V, doc: &Document<'ast>) {
    for item in &doc.preamble {
        visitor.visit_preamble_item(item);
    }
    for node in &doc.body {
        visitor.visit_body_node(node);
    }
}

/// Default walk function for [`PreambleItem`].
pub fn walk_preamble_item<'ast, V: Visitor<'ast> + ?Sized>(
    visitor: &mut V,
    item: &PreambleItem<'ast>,
) {
    match item {
        PreambleItem::DocumentClass(dc) => visitor.visit_document_class(dc),
        PreambleItem::UsePackage(pkg) => visitor.visit_use_package(pkg),
        PreambleItem::Command(cmd) => visitor.visit_command(cmd),
        PreambleItem::Text(t) => visitor.visit_text(t),
        PreambleItem::Comment(c) => visitor.visit_comment(c),
        PreambleItem::Group(g) => visitor.visit_group(g),
    }
}

/// Default walk function for [`DocumentBodyNode`].
pub fn walk_body_node<'ast, V: Visitor<'ast> + ?Sized>(
    visitor: &mut V,
    node: &DocumentBodyNode<'ast>,
) {
    match node {
        DocumentBodyNode::Environment(env) => visitor.visit_environment(env),
        DocumentBodyNode::Command(cmd) => visitor.visit_command(cmd),
        DocumentBodyNode::Text(t) => visitor.visit_text(t),
        DocumentBodyNode::Math(m) => visitor.visit_math(m),
        DocumentBodyNode::Comment(c) => visitor.visit_comment(c),
        DocumentBodyNode::Group(g) => visitor.visit_group(g),
    }
}

/// Default walk function for [`Command`].
pub fn walk_command<'ast, V: Visitor<'ast> + ?Sized>(visitor: &mut V, cmd: &Command<'ast>) {
    for arg in &cmd.args {
        if let Arg::Braced(g) = arg {
            visitor.visit_group(g);
        }
    }
}

/// Default walk function for [`Environment`].
pub fn walk_environment<'ast, V: Visitor<'ast> + ?Sized>(visitor: &mut V, env: &Environment<'ast>) {
    for node in &env.body {
        visitor.visit_body_node(node);
    }
}

/// Default mutable walk function for [`Document`].
pub fn walk_document_mut<'ast, V: VisitorMut<'ast> + ?Sized>(
    visitor: &mut V,
    doc: &mut Document<'ast>,
) {
    for item in &mut doc.preamble {
        visitor.visit_preamble_item(item);
    }
    for node in &mut doc.body {
        visitor.visit_body_node(node);
    }
}

/// Default mutable walk function for [`PreambleItem`].
pub fn walk_preamble_item_mut<'ast, V: VisitorMut<'ast> + ?Sized>(
    visitor: &mut V,
    item: &mut PreambleItem<'ast>,
) {
    match item {
        PreambleItem::DocumentClass(dc) => visitor.visit_document_class(dc),
        PreambleItem::UsePackage(pkg) => visitor.visit_use_package(pkg),
        PreambleItem::Command(cmd) => visitor.visit_command(cmd),
        PreambleItem::Text(t) => visitor.visit_text(t),
        PreambleItem::Comment(c) => visitor.visit_comment(c),
        PreambleItem::Group(g) => visitor.visit_group(g),
    }
}

/// Default mutable walk function for [`DocumentBodyNode`].
pub fn walk_body_node_mut<'ast, V: VisitorMut<'ast> + ?Sized>(
    visitor: &mut V,
    node: &mut DocumentBodyNode<'ast>,
) {
    match node {
        DocumentBodyNode::Environment(env) => visitor.visit_environment(env),
        DocumentBodyNode::Command(cmd) => visitor.visit_command(cmd),
        DocumentBodyNode::Text(t) => visitor.visit_text(t),
        DocumentBodyNode::Math(m) => visitor.visit_math(m),
        DocumentBodyNode::Comment(c) => visitor.visit_comment(c),
        DocumentBodyNode::Group(g) => visitor.visit_group(g),
    }
}

/// Default mutable walk function for [`Command`].
pub fn walk_command_mut<'ast, V: VisitorMut<'ast> + ?Sized>(
    visitor: &mut V,
    cmd: &mut Command<'ast>,
) {
    for arg in &mut cmd.args {
        if let Arg::Braced(g) = arg {
            visitor.visit_group(g);
        }
    }
}

/// Default mutable walk function for [`Environment`].
pub fn walk_environment_mut<'ast, V: VisitorMut<'ast> + ?Sized>(
    visitor: &mut V,
    env: &mut Environment<'ast>,
) {
    for node in &mut env.body {
        visitor.visit_body_node(node);
    }
}
