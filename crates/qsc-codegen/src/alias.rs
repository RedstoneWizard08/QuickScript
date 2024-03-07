use cranelift_codegen::ir;
use cranelift_module::{FuncId, Linkage, Module, ModuleResult};
use object::{
    write::{Symbol, SymbolSection},
    SymbolFlags, SymbolKind,
};
use qsc_jit::JITModule;
use qsc_object::{
    backend::{translate_linkage, validate_symbol},
    ObjectModule,
};

pub trait DeclareAliasedFunction {
    fn declare_aliased_function(
        &mut self,
        name: &str,
        real_name: &str,
        linkage: Linkage,
        signature: &ir::Signature,
    ) -> ModuleResult<FuncId>;
}

impl<M: Module + DeclareAliasedFunction> DeclareAliasedFunction for &mut M {
    fn declare_aliased_function(
        &mut self,
        name: &str,
        real_name: &str,
        linkage: Linkage,
        signature: &ir::Signature,
    ) -> ModuleResult<FuncId> {
        (**self).declare_aliased_function(name, real_name, linkage, signature)
    }
}

impl DeclareAliasedFunction for ObjectModule {
    fn declare_aliased_function(
        &mut self,
        name: &str,
        real_name: &str,
        linkage: Linkage,
        signature: &ir::Signature,
    ) -> ModuleResult<FuncId> {
        validate_symbol(name)?;

        let (id, linkage) = self
            .declarations
            .declare_function(name, linkage, signature)?;

        let (scope, weak) = translate_linkage(linkage);

        if let Some((function, _defined)) = self.functions[id] {
            let symbol = self.object.symbol_mut(function);

            symbol.scope = scope;
            symbol.weak = weak;
        } else {
            let symbol_id = self.object.add_symbol(Symbol {
                name: real_name.as_bytes().to_vec(),
                value: 0,
                size: 0,
                kind: SymbolKind::Text,
                scope,
                weak,
                section: SymbolSection::Undefined,
                flags: SymbolFlags::None,
            });

            self.functions[id] = Some((symbol_id, false));
        }

        Ok(id)
    }
}

impl DeclareAliasedFunction for JITModule {
    fn declare_aliased_function(
        &mut self,
        name: &str,
        real_name: &str,
        linkage: Linkage,
        signature: &ir::Signature,
    ) -> ModuleResult<FuncId> {
        let (id, linkage) = self
            .declarations
            .declare_function(name, linkage, signature)?;

        if self.function_got_entries[id].is_none() && self.isa.flags().is_pic() {
            // FIXME populate got entries with a null pointer when defined
            let val = if linkage == Linkage::Import {
                self.lookup_symbol(real_name).unwrap_or(std::ptr::null())
            } else {
                std::ptr::null()
            };

            self.new_func_plt_entry(id, val);
        }

        Ok(id)
    }
}
