//! `AstIdMap` allows to create stable IDs for "large" syntax nodes like items
//! and macro calls.
//!
//! Specifically, it enumerates all items in a file and uses position of a an
//! item as an ID. That way, id's don't change unless the set of items itself
//! changes.

use std::{
    any::type_name,
    fmt,
    hash::{Hash, Hasher},
    marker::PhantomData,
};

use arena::{Arena, Idx, RawIdx};
use basedb::FileId;
use syntax::{ast, match_ast, AstNode, AstPtr, SyntaxNode, SyntaxNodePtr};

// #[derive(Debug, PartialEq, Eq, Hash)]
// pub struct AstId<N: AstNode> {
//     file: FileId,
//     ast: FileAstId<N>,
// }

// impl<N: AstNode> Copy for AstId<N> {}

// impl<N: AstNode> Clone for AstId<N> {
//     fn clone(&self) -> AstId<N> {
//         *self
//     }
// }

/// `AstId` points to an AST node in a specific file.
pub struct FileAstId<N: AstNode> {
    raw: ErasedFileAstId,
    _ty: PhantomData<fn() -> N>,
}

impl<N: AstNode> Clone for FileAstId<N> {
    fn clone(&self) -> FileAstId<N> {
        *self
    }
}

impl<N: AstNode> Copy for FileAstId<N> {}

impl<N: AstNode> PartialEq for FileAstId<N> {
    fn eq(&self, other: &Self) -> bool {
        self.raw == other.raw
    }
}
impl<N: AstNode> Eq for FileAstId<N> {}
impl<N: AstNode> Hash for FileAstId<N> {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.raw.hash(hasher);
    }
}

impl<N: AstNode> fmt::Debug for FileAstId<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FileAstId::<{}>({})", type_name::<N>(), RawIdx::from(self.raw))
    }
}

impl<N: AstNode> FileAstId<N> {
    // Can't make this a From implementation because of coherence
    pub fn upcast<M: AstNode>(self) -> FileAstId<M>
    where
        N: Into<M>,
    {
        FileAstId { raw: self.raw, _ty: PhantomData }
    }
}

type ErasedFileAstId = Idx<SyntaxNodePtr>;

/// Maps items' `SyntaxNode`s to `ErasedFileAstId`s and back.
#[derive(Debug, PartialEq, Eq, Default)]
pub struct AstIdMap {
    arena: Arena<SyntaxNodePtr>,
}

impl AstIdMap {
    pub(crate) fn from_source(node: &SyntaxNode) -> AstIdMap {
        assert!(node.parent().is_none());
        let mut res = AstIdMap::default();
        // By walking the tree in breadth-first order we make sure that parents
        // get lower ids then children. That is, adding a new child does not
        // change parent's id. This means that, say, adding a new function to a
        // trait does not change ids of top-level items, which helps caching.
        bdfs(node, |it| {
            match_ast! {
                match it {
                    ast::Item(module_item) => {
                        res.alloc(module_item.syntax());
                        true
                    },
                    ast::BlockStmt(block) => {
                        res.alloc(block.syntax());
                        true
                    },

                    ast::BodyPortDecl(_decl) => false,
                    ast::ParamDecl(_decl) => false,
                    ast::VarDecl(_decl) => false,

                    ast::Param(decl) => {
                        res.alloc(decl.syntax());
                        true
                    },

                    ast::Var(decl) => {
                        res.alloc(decl.syntax());
                        true
                    },

                    // Also includes variable decl/parameters inside BlockStmts
                    // These technically form a seperate enum.
                    // However the syntax node is the same so they can be cast
                    // to a module item and are therefore also covered here.
                    ast::ModuleItem(item) => {
                        res.alloc(item.syntax());
                        true
                    },

                    ast::FunctionArg(item) => {
                        res.alloc(item.syntax());
                        true
                    },


                    ast::NatureAttr(item) => {
                        res.alloc(item.syntax());
                        true
                    },


                    ast::DisciplineAttr(item) => {
                        res.alloc(item.syntax());
                        true
                    },

                    ast::PortDecl(item) => {
                        res.alloc(item.syntax());
                        true
                    },

                    _ => false,
                }
            }
        });
        res
    }

    pub fn ast_id<N: AstNode>(&self, item: &N) -> FileAstId<N> {
        let raw = self.erased_ast_id(item.syntax());
        FileAstId { raw, _ty: PhantomData }
    }

    fn erased_ast_id(&self, item: &SyntaxNode) -> ErasedFileAstId {
        let ptr = SyntaxNodePtr::new(item);
        match self.arena.iter_enumerated().find(|(_id, i)| **i == ptr) {
            Some((it, _)) => it,
            None => panic!(
                "Can't find {:?} in AstIdMap",
                item,
                // self.arena.iter_enumerated().map(|(_id, i)| i).collect::<Vec<_>>(),
            ),
        }
    }

    pub fn get<N: AstNode>(&self, id: FileAstId<N>) -> AstPtr<N> {
        self.arena[id.raw].cast::<N>().unwrap()
    }

    fn alloc(&mut self, item: &SyntaxNode) -> ErasedFileAstId {
        self.arena.push_and_get_key(SyntaxNodePtr::new(item))
    }
}

/// Walks the subtree in bdfs order, calling `f` for each node. What is bdfs
/// order? It is a mix of breadth-first and depth first orders. Nodes for which
/// `f` returns true are visited breadth-first, all the other nodes are explored
/// depth-first.
///
/// In other words, the size of the bfs queue is bound by the number of "true"
/// nodes.
fn bdfs(node: &SyntaxNode, mut f: impl FnMut(SyntaxNode) -> bool) {
    let mut curr_layer = vec![node.clone()];
    let mut next_layer = vec![];
    while !curr_layer.is_empty() {
        curr_layer.drain(..).for_each(|node| {
            let mut preorder = node.preorder();
            while let Some(event) = preorder.next() {
                match event {
                    syntax::WalkEvent::Enter(node) => {
                        if f(node.clone()) {
                            next_layer.extend(node.children());
                            preorder.skip_subtree();
                        }
                    }
                    syntax::WalkEvent::Leave(_) => {}
                }
            }
        });
        std::mem::swap(&mut curr_layer, &mut next_layer);
    }
}