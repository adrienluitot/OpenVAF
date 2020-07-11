/*
 * ******************************************************************************************
 * Copyright (c) 2020 Pascal Kuthe. This file is part of the frontend project.
 * It is subject to the license terms in the LICENSE file found in the top-level directory
 *  of this distribution and at  https://gitlab.com/DSPOM/OpenVAF/blob/master/LICENSE.
 *  No part of frontend, including this file, may be copied, modified, propagated, or
 *  distributed except according to the terms contained in the LICENSE file.
 * *****************************************************************************************
 */

use crate::ast::NetType::GROUND;
use crate::ast_lowering::ast_to_hir_fold::Fold;
use crate::ast_lowering::error::Error::{DisciplineMismatch, NatureNotPotentialOrFlow};
use crate::ast_lowering::error::{AllowedNatures, NetInfo};
use crate::hir::Net;
use crate::hir::{Branch, BranchDeclaration, DisciplineAccess};
use crate::ir::{AttributeNode, Attributes, BranchId, DisciplineId, NatureId, NetId, Node, PortId};
use crate::sourcemap::span::DUMMY_SP;
use crate::symbol::{keywords, Ident};
use crate::{ast, Ast, HashMap};

/// Handles branch resolution which is more complicated because unnamed branches exist and discipline comparability has to be enforced
pub struct BranchResolver {
    unnamed_branches: HashMap<(NetId, NetId), BranchId>,
    unnamed_port_branches: HashMap<PortId, BranchId>,
    implicit_grounds: HashMap<DisciplineId, NetId>,
}

impl<'lt> BranchResolver {
    #[must_use]
    pub fn new(ast: &'lt Ast) -> Self {
        Self {
            unnamed_port_branches: HashMap::with_capacity(16),
            unnamed_branches: HashMap::with_capacity(32),
            implicit_grounds: HashMap::with_capacity(ast.disciplines.len() as usize),
        }
    }
    /// Resolves a DisciplineAccess (for example `V(b)` or `V(x,y)`)
    ///
    /// # Arguments
    ///
    /// * `fold` - The calling fold which is used for name resolution and error handling
    ///
    /// * `nature_name` - The identifier of the nature (for example `V` in the case of `V(X,Y)`)
    ///
    /// * `discipline` - The id of the Discipline of a `BranchAccess` ( that has been resolved using [`resolve_branch_access`](crate::ast_lowering::branch_resolution::BranchResolver::resolve_discipline_access)

    pub fn resolve_discipline_access(
        fold: &mut Fold<'lt>,
        nature_name: &Ident,
        discipline: DisciplineId,
    ) -> Option<DisciplineAccess> {
        match nature_name.name {
            keywords::flow => Some(DisciplineAccess::Flow),
            keywords::potential => Some(DisciplineAccess::Potential),
            _ => {
                resolve!(fold; nature_name as
                    Nature(id) => {
                        return Self::resolve_nature_access(fold,id,discipline);
                    }
                );
                None
            }
        }
    }

    pub fn resolve_nature_access(
        fold: &mut Fold<'lt>,
        id: NatureId,
        discipline: DisciplineId,
    ) -> Option<DisciplineAccess> {
        match id {
            id if Some(id) == fold.hir[discipline].contents.flow_nature => {
                Some(DisciplineAccess::Flow)
            }
            id if Some(id) == fold.hir[discipline].contents.potential_nature => {
                Some(DisciplineAccess::Potential)
            }
            _ => {
                fold.error(NatureNotPotentialOrFlow {
                    nature: fold.hir[id].contents.ident,
                    allowed_natures: AllowedNatures::from_discipline(discipline, &fold.hir),
                });
                None
            }
        }
    }

    /// Resolves a branch access such as `(NET1,NET2)`,`(<PORT>)` or `(BRANCH)`
    ///
    /// # Arguments
    ///
    /// * `fold` - The calling fold which is used for name resolution and error handling
    ///
    /// * `branch_access` - A reference to an Ast node for a branch access call
    ///
    ///
    /// # Returns
    /// The Id of the resolved branch and its Discipline (if the resolution succeeded)
    pub fn resolve_branch_access(
        &mut self,
        fold: &mut Fold<'lt>,
        branch_access: &Node<ast::BranchAccess>,
    ) -> Option<(BranchId, DisciplineId)> {
        match branch_access.contents {
            ast::BranchAccess::Implicit(ref branch) => {
                let (branch, discipline) = self.resolve_branch(fold, branch)?;
                match branch {
                    Branch::Port(port) => {
                        if let Some(id) = self.unnamed_port_branches.get(&port) {
                            return Some((*id, discipline));
                        } else {
                            let branch_id = fold.hir.branches.push(AttributeNode {
                                attributes: Attributes::EMPTY,
                                span: branch_access.span,
                                contents: BranchDeclaration {
                                    name: Ident::from_str_and_span(
                                        format!(
                                            "( <{}> )",
                                            fold.hir[fold.hir[port].net].contents.name
                                        )
                                        .as_str(),
                                        branch_access.span,
                                    ),
                                    branch,
                                },
                            });
                            self.unnamed_port_branches.insert(port, branch_id);
                            return Some((branch_id, discipline));
                        }
                    }

                    Branch::Nets(net1, net2) => {
                        if let Some(id) = self.unnamed_branches.get(&(net1, net2)) {
                            return Some((*id, discipline));
                        } else {
                            let branch_id = fold.hir.branches.push(AttributeNode {
                                attributes: Attributes::EMPTY,
                                span: branch_access.span,
                                contents: BranchDeclaration {
                                    name: Ident::from_str_and_span(
                                        format!(
                                            "({} , {})",
                                            fold.hir[net1].contents.name.name.as_str(),
                                            fold.hir[net2].contents.name.name.as_str()
                                        )
                                        .as_str(),
                                        branch_access.span,
                                    ),
                                    branch,
                                },
                            });
                            self.unnamed_branches.insert((net1, net2), branch_id);
                            return Some((branch_id, discipline));
                        }
                    }
                }
            }

            ast::BranchAccess::BranchOrNodePotential(ref name) => {
                resolve_hierarchical!(fold; name as
                    Branch(id) => {
                        let discipline = match fold.hir[id].contents.branch {
                            Branch::Port(portid) => {
                                fold.hir[fold.hir[portid].net].contents.discipline
                            }
                            Branch::Nets(net1, _) => fold.hir[net1].contents.discipline
                        };
                        return Some((id,discipline))
                    },

                    // Needed to resolve ambiguities. Inefficient but will do
                    Port(_id) => {
                        return self.resolve_branch_access(fold,&branch_access.clone_as(ast::BranchAccess::Implicit(ast::Branch::Port(name.clone()))))
                    },

                    Net(_id) => {
                        return self.resolve_branch_access(fold,&branch_access.clone_as(ast::BranchAccess::Implicit(ast::Branch::NetToGround(name.clone()))))
                    }
                )
            }
        }

        None
    }

    /// Resolves a branch such as (NET1,NET2) or (<PORT>)
    ///
    /// # Arguments
    ///
    /// * fold - The calling fold which is used for name resolution and error handling
    ///
    /// * branch - An Ast node describing a branch
    ///
    ///
    /// # Returns
    /// The Id of the resolved branch and its Discipline  (if the resolution succeeded)

    pub fn resolve_branch(
        &mut self,
        fold: &mut Fold<'lt>,
        branch: &ast::Branch,
    ) -> Option<(Branch, DisciplineId)> {
        match branch {
            ast::Branch::Port(ref port) => {
                resolve_hierarchical!(fold; port as Port(port_id) => {
                    return Some((Branch::Port(port_id),fold.hir[fold.hir[port_id].net].contents.discipline));
                });
            }
            ast::Branch::NetToGround(ref net_ident) => {
                let mut net = None;
                resolve_hierarchical!(fold; net_ident as
                    Net(id) => {
                        net = Some(id);
                    },
                    Port(id) => {
                        net = Some(fold.hir[id].net);
                    }
                );
                if let Some(net) = net {
                    let discipline = fold.hir[net].contents.discipline;
                    let ground_net =
                        *self.implicit_grounds.entry(discipline).or_insert_with(|| {
                            fold.hir.nets.push(AttributeNode {
                                contents: Net {
                                    name: Ident::from_str("implicit_ground"),
                                    discipline,
                                    signed: false,
                                    net_type: GROUND,
                                },
                                span: DUMMY_SP,
                                attributes: Attributes::EMPTY,
                            })
                        });
                    return Some((Branch::Nets(net, ground_net), discipline));
                }
            }

            ast::Branch::Nets(ref net1, ref net2) => {
                let mut first_net = None;
                resolve_hierarchical!(fold; net1 as
                    Net(id) => {
                        first_net = Some(id);
                    },
                    Port(id) => {
                        first_net = Some(fold.hir[id].net);
                    }
                );

                let mut second_net = None;
                resolve_hierarchical!(fold; net2 as
                    Net(second_id) => {
                        second_net = Some(second_id)
                    },
                    Port(second_id) => {
                        second_net = Some(fold.hir[second_id].net)
                    }
                );

                if let (Some(first_net), Some(second_net)) = (first_net, second_net) {
                    if fold.hir[first_net].contents.discipline
                        == fold.hir[second_net].contents.discipline
                    {
                        //doesn't matter which nets discipline we use since we asserted that they are equal
                        return Some((
                            Branch::Nets(first_net, second_net),
                            fold.hir[first_net].contents.discipline,
                        ));
                    } else {
                        let first_net = &fold.hir[first_net];
                        let first_discipline =
                            fold.hir[first_net.contents.discipline].contents.ident.name;

                        let second_net = &fold.hir[second_net];
                        let second_discipline =
                            fold.hir[second_net.contents.discipline].contents.ident.name;

                        let err = DisciplineMismatch(
                            NetInfo {
                                discipline: first_discipline,
                                name: first_net.contents.name.name,
                                declaration: first_net.span,
                            },
                            NetInfo {
                                discipline: second_discipline,
                                name: second_net.contents.name.name,
                                declaration: second_net.span,
                            },
                            net1.span().extend(net2.span()),
                        );

                        fold.error(err)
                    }
                }
            }
        }

        None
    }
}