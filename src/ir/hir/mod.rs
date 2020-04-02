/*
 * ******************************************************************************************
 * Copyright (c) 2019 Pascal Kuthe. This file is part of the VARF project.
 * It is subject to the license terms in the LICENSE file found in the top-level directory
 *  of this distribution and at  https://gitlab.com/jamescoding/VARF/blob/master/LICENSE.
 *  No part of VARF, including this file, may be copied, modified, propagated, or
 *  distributed except according to the terms contained in the LICENSE file.
 * *****************************************************************************************
 */

use std::ops::Range;
use std::ptr::NonNull;

use crate::ast::Parameter;
use crate::compact_arena::{NanoArena, SafeRange, TinyArena};
use crate::ir::ast::{
    Ast, Attribute, AttributeNode, Attributes, BinaryOperator, BuiltInFunctionCall, Function,
    Nature, NetType, Node, UnaryOperator, Variable,
};
use crate::ir::{
    AttributeId, BranchId, DisciplineId, ExpressionId, FunctionId, ModuleId, NatureId, NetId,
    ParameterId, PortId, StatementId, VariableId, Write,
};
use crate::symbol::Ident;
use crate::Span;

//pub mod visitor;

/// An High level (tree) IR representing a Verilog-AMS project;
/// It provides stable indicies for every Node because the entire Tree is immutable once created;
/// It uses preallocated constant size arrays for better performance
/// Compared to an AST all references are resolved to their respective ids here and unnecessary constructs like blocks are ignored

//TODO make this into a general proc macro with lifetimes like compact arena
pub struct Hir<'tag> {
    //TODO unsized
    //TODO configure to use different arena sizes
    //Declarations
    pub(crate) parameters: TinyArena<'tag, AttributeNode<'tag, Parameter<'tag>>>,
    //    nature: NanoArena<'tag,Nature>
    pub(crate) branches: NanoArena<'tag, AttributeNode<'tag, BranchDeclaration<'tag>>>,
    pub(crate) nets: TinyArena<'tag, AttributeNode<'tag, Net<'tag>>>,
    pub(crate) ports: NanoArena<'tag, Port<'tag>>,
    pub(crate) variables: TinyArena<'tag, AttributeNode<'tag, Variable<'tag>>>,
    pub(crate) modules: NanoArena<'tag, AttributeNode<'tag, Module<'tag>>>,
    pub(crate) functions: NanoArena<'tag, AttributeNode<'tag, Function<'tag>>>,
    pub(crate) disciplines: NanoArena<'tag, AttributeNode<'tag, Discipline<'tag>>>,
    pub(crate) natures: NanoArena<'tag, AttributeNode<'tag, Nature>>,
    //Ast Items
    pub(crate) expressions: TinyArena<'tag, Node<Expression<'tag>>>,
    pub(crate) attributes: TinyArena<'tag, Attribute<'tag>>,
    pub(crate) statements: TinyArena<'tag, Statement<'tag>>,
}
///this module contains copys of the definitions of tiny/small arena so we are able to acess internal fields for initialisation on the heap using pointers

impl<'tag> Hir<'tag> {
    /// # Safety
    /// You should never call this yourself. Lower an AST created using mk_ast! instead
    pub(crate) unsafe fn init<'lt>(ast: &'lt mut Ast<'tag>) -> Box<Self> {
        let layout = std::alloc::Layout::new::<Self>();
        #[allow(clippy::cast_ptr_alignment)]
        //the ptr cast below has the right alignment since we are allocation using the right layout
        let mut res: NonNull<Self> = NonNull::new(std::alloc::alloc(layout) as *mut Self)
            .unwrap_or_else(|| std::alloc::handle_alloc_error(layout));
        TinyArena::init(&mut res.as_mut().parameters);
        NanoArena::init(&mut res.as_mut().branches);
        TinyArena::init(&mut res.as_mut().nets);
        NanoArena::init(&mut res.as_mut().ports);
        TinyArena::init(&mut res.as_mut().variables);
        NanoArena::init(&mut res.as_mut().ports);
        NanoArena::init(&mut res.as_mut().functions);
        NanoArena::init(&mut res.as_mut().disciplines);
        NanoArena::init(&mut res.as_mut().natures);
        TinyArena::init(&mut res.as_mut().expressions);
        TinyArena::init(&mut res.as_mut().attributes);
        TinyArena::init(&mut res.as_mut().statements);
        Box::from_raw(res.as_ptr())
    }
}

impl_id_type!(BranchId in Hir::branches -> AttributeNode<'tag,BranchDeclaration<'tag>>);
impl<'tag> Write<BranchId<'tag>> for Hir<'tag> {
    type Data = AttributeNode<'tag, BranchDeclaration<'tag>>;
    fn write(&mut self, index: BranchId<'tag>, value: Self::Data) {
        unsafe {
            //this is save for types that dont implement drop
            self.branches
                .write(index.0, ::core::mem::MaybeUninit::new(value))
        }
    }
}
impl_id_type!(NetId in Hir::nets -> AttributeNode<'tag,Net<'tag>>);
impl<'tag> Write<NetId<'tag>> for Hir<'tag> {
    type Data = AttributeNode<'tag, Net<'tag>>;
    fn write(&mut self, index: NetId<'tag>, value: Self::Data) {
        unsafe {
            self.nets
                .write(index.0, ::core::mem::MaybeUninit::new(value))
        }
    }
}
impl_id_type!(PortId in Hir::ports -> Port<'tag>);
impl<'tag> Write<PortId<'tag>> for Hir<'tag> {
    type Data = Port<'tag>;
    fn write(&mut self, index: PortId<'tag>, value: Self::Data) {
        unsafe {
            self.ports
                .write(index.0, ::core::mem::MaybeUninit::new(value))
        }
    }
}
impl_id_type!(VariableId in Hir::variables ->  AttributeNode<'tag,Variable<'tag>>);
impl<'tag> Write<VariableId<'tag>> for Hir<'tag> {
    type Data = AttributeNode<'tag, Variable<'tag>>;
    fn write(&mut self, index: VariableId<'tag>, value: Self::Data) {
        unsafe {
            self.variables
                .write(index.0, ::core::mem::MaybeUninit::new(value))
        }
    }
}
impl_id_type!(ModuleId in Hir::modules -> AttributeNode<'tag,Module<'tag>>);
impl<'tag> Write<ModuleId<'tag>> for Hir<'tag> {
    type Data = AttributeNode<'tag, Module<'tag>>;
    fn write(&mut self, index: ModuleId<'tag>, value: Self::Data) {
        unsafe {
            self.modules
                .write(index.0, ::core::mem::MaybeUninit::new(value))
        }
    }
}
impl_id_type!(FunctionId in Hir::functions -> AttributeNode<'tag,Function<'tag>>);
impl_id_type!(DisciplineId in Hir::disciplines -> AttributeNode<'tag,Discipline<'tag>>);
impl<'tag> Write<DisciplineId<'tag>> for Hir<'tag> {
    type Data = AttributeNode<'tag, Discipline<'tag>>;
    fn write(&mut self, index: DisciplineId<'tag>, value: Self::Data) {
        unsafe {
            self.disciplines
                .write(index.0, ::core::mem::MaybeUninit::new(value))
        }
    }
}
impl_id_type!(ExpressionId in Hir::expressions -> Node<Expression<'tag>>);
impl_id_type!(AttributeId in Hir::attributes -> Attribute<'tag>);
impl_id_type!(StatementId in Hir::statements -> Statement<'tag>);
impl_id_type!(NatureId in Hir::natures -> AttributeNode<'tag,Nature>);
impl_id_type!(ParameterId in Hir::parameters -> AttributeNode<'tag,Parameter<'tag>>);

#[derive(Clone, Copy, Debug)]
pub struct Discipline<'tag> {
    pub name: Ident,
    pub flow_nature: NatureId<'tag>,
    pub potential_nature: NatureId<'tag>,
}

#[derive(Clone, Copy, Debug)]
pub struct Module<'hir> {
    pub name: Ident,
    pub port_list: SafeRange<PortId<'hir>>,
    //    pub parameter_list: Option<Range<ParameterId<'ast>>>
    pub analog: Block<'hir>,
}
pub type Block<'hir> = SafeRange<StatementId<'hir>>;
#[derive(Clone, Debug)]
pub struct Condition<'hir> {
    pub main_condition: ExpressionId<'hir>,
    pub main_condition_statements: Block<'hir>,
    pub else_ifs: Vec<(ExpressionId<'hir>, SafeRange<StatementId<'hir>>)>,
    pub else_statement: SafeRange<StatementId<'hir>>,
}
#[derive(Clone, Copy, Debug)]
pub struct Port<'tag> {
    pub input: bool,
    pub output: bool,
    pub net: NetId<'tag>,
}

#[derive(Clone, Copy, Debug)]
pub struct BranchDeclaration<'hir> {
    pub name: Ident,
    pub branch: Branch<'hir>,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Branch<'hir> {
    Port(PortId<'hir>),
    Nets(NetId<'hir>, NetId<'hir>),
}
#[derive(Clone, Copy, Debug)]
pub struct Net<'hir> {
    pub name: Ident,
    pub discipline: DisciplineId<'hir>,
    pub signed: bool,
    pub net_type: NetType,
}
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum DisciplineAccess {
    Potential,
    Flow,
}

#[derive(Clone, Debug)]
pub enum Statement<'hir> {
    Condition(AttributeNode<'hir, Condition<'hir>>),
    ConditionStart {
        condition_info_and_end: StatementId<'hir>,
    },
    Contribute(
        Attributes<'hir>,
        DisciplineAccess,
        BranchId<'hir>,
        ExpressionId<'hir>,
    ),
    //  TODO IndirectContribute(),
    Assignment(Attributes<'hir>, VariableId<'hir>, ExpressionId<'hir>),
    FunctionCall(Attributes<'hir>, FunctionId<'hir>, Vec<ExpressionId<'hir>>),
    BuiltInFunctionCall(AttributeNode<'hir, BuiltInFunctionCall<'hir>>),
}

#[derive(Clone, Debug)]
pub enum Expression<'hir> {
    BinaryOperator(ExpressionId<'hir>, Node<BinaryOperator>, ExpressionId<'hir>),
    UnaryOperator(Node<UnaryOperator>, ExpressionId<'hir>),
    Condtion(
        ExpressionId<'hir>,
        Span,
        ExpressionId<'hir>,
        Span,
        ExpressionId<'hir>,
    ),
    Primary(Primary<'hir>),
}
#[derive(Clone, Debug)]
pub enum Primary<'hir> {
    Integer(i64),
    UnsignedInteger(u32),
    Real(f64),
    VariableReference(VariableId<'hir>),
    NetReference(NetId<'hir>),
    PortReference(PortId<'hir>),
    ParameterReference(ParameterId<'hir>),
    FunctionCall(FunctionId<'hir>, Vec<ExpressionId<'hir>>),
    BranchAccess(DisciplineAccess, BranchId<'hir>),
    BuiltInFunctionCall(BuiltInFunctionCall<'hir>),
    SystemFunctionCall(Ident /*TODO args*/),
}