use crate::ast::{Net, NetType, VariableType};
use crate::diagnostic::MultiDiagnostic;
use crate::ir::ast::{Ast, Variable};
use crate::ir::ids::{NetId, VariableId};
use crate::ir::{Attributes, Node};
use crate::parser::error::Error;
use crate::parser::error::Error::AlreadyDeclaredInThisScope;
use crate::symbol::Ident;
use crate::symbol_table::{SymbolDeclaration, SymbolTable};

pub struct SymbolTableBuilder(SymbolTable, Vec<SymbolTable>);

impl SymbolTableBuilder {
    pub fn new() -> Self {
        Self(SymbolTable::with_capacity(64), Vec::new())
    }
    pub fn exit_scope(&mut self) -> SymbolTable {
        self.1.pop().expect("Tried to exit root scope")
    }

    pub fn enter_scope(&mut self) {
        self.1.push(SymbolTable::with_capacity(32))
    }

    pub fn enter_function_scope(&mut self, ident: Ident, ast: &mut Ast) -> VariableId {
        let return_var = Variable {
            ident,
            var_type: VariableType::Real,
            default: None,
        };
        let return_var = ast.variables.push(Node {
            attributes: Attributes::EMPTY,
            span: ident.span,
            contents: return_var,
        });

        let mut scope = SymbolTable::with_capacity(32);
        scope.insert(ident.name, return_var.into());
        self.1.push(scope);
        return_var
    }

    pub fn finish(self) -> SymbolTable {
        debug_assert!(self.1.is_empty());
        self.0
    }

    fn current_table(&self) -> &SymbolTable {
        self.1.last().unwrap_or(&self.0)
    }

    fn current_table_mut(&mut self) -> &mut SymbolTable {
        self.1.last_mut().unwrap_or(&mut self.0)
    }

    #[inline]
    pub fn insert(
        &mut self,
        ast: &Ast,
        errors: &mut MultiDiagnostic<Error>,
        declaration: impl Into<SymbolDeclaration>,
    ) {
        let declaration = declaration.into();
        let ident = declaration.ident(ast);

        if let Some(old_declaration) = self.current_table_mut().insert(ident.name, declaration) {
            errors.add(AlreadyDeclaredInThisScope {
                declaration: ident.span,
                other_declaration: old_declaration.ident(ast).span,
                name: ident.name,
            });
        }
    }

    #[inline]
    pub fn insert_net(
        &mut self,
        ast: &mut Ast,
        errors: &mut MultiDiagnostic<Error>,
        declaration: Node<Net>,
    ) -> Option<NetId> {
        if let Some(old_declaration) = self.current_table().get(&declaration.contents.ident.name) {
            if let SymbolDeclaration::Port(id) = *old_declaration {
                let id = ast[id].net;
                if ast[id].contents.net_type == NetType::UNDECLARED
                    && ast[id].contents.discipline.is_none()
                //TODO range
                {
                    ast[id].contents.net_type = declaration.contents.net_type;
                    ast[id].contents.discipline = declaration.contents.discipline;
                    return Some(id);
                }
            }

            let error = AlreadyDeclaredInThisScope {
                declaration: declaration.span,
                other_declaration: old_declaration.span(ast),
                name: declaration.contents.ident.name,
            };

            errors.add(error);
            None
        } else {
            let id = ast.nets.push(declaration);
            self.current_table_mut()
                .insert(declaration.contents.ident.name, SymbolDeclaration::Net(id));
            Some(id)
        }
    }
}

impl Default for SymbolTableBuilder {
    fn default() -> Self {
        Self::new()
    }
}
