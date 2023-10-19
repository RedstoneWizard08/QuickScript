use anyhow::Result;
use codegen::{
    entity::EntityRef,
    ir::{types, AbiParam, Block, InstBuilder},
    settings::Configurable,
};

use cranelift::prelude::{codegen, settings, FunctionBuilderContext, IntCC, Value, Variable};
use cranelift_frontend::FunctionBuilder;
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{DataDescription, Linkage, Module};
use std::collections::HashMap;
use std::slice;

use crate::{
    ast::{parser, Expr, Function},
    util::random_string,
};

pub struct Compiler {
    builder_context: FunctionBuilderContext,
    ctx: codegen::Context,
    data_description: DataDescription,
    module: JITModule,
}

impl Compiler {
    pub fn new() -> Self {
        let mut flag_builder = settings::builder();

        flag_builder.set("use_colocated_libcalls", "false").unwrap();
        flag_builder.set("is_pic", "false").unwrap();

        let isa_builder = cranelift_native::builder().unwrap_or_else(|msg| {
            panic!("host machine is not supported: {}", msg);
        });

        let isa = isa_builder
            .finish(settings::Flags::new(flag_builder))
            .unwrap();

        let builder = JITBuilder::with_isa(isa, cranelift_module::default_libcall_names());
        let module = JITModule::new(builder);

        Self {
            builder_context: FunctionBuilderContext::new(),
            ctx: module.make_context(),
            data_description: DataDescription::new(),
            module,
        }
    }

    pub fn compile(&mut self, input: &str) -> Result<Vec<*const u8>> {
        let input = format!(
            "{}\n",
            input
                .split('\n')
                .filter(|v| v != &"")
                .collect::<Vec<&str>>()
                .join("\n")
        );

        let functions = parser::root(&input)?;

        let mut ret = Vec::new();

        for function in functions {
            ret.push(self.compile_function(function)?);
        }

        Ok(ret)
    }

    pub fn compile_function(
        &mut self,
        Function(name, params, the_return, stmts): Function,
    ) -> Result<*const u8> {
        self.translate(params, the_return, stmts)?;

        let id = self
            .module
            .declare_function(&name, Linkage::Export, &self.ctx.func.signature)?;

        self.module.define_function(id, &mut self.ctx)?;

        self.module.clear_context(&mut self.ctx);
        self.module.finalize_definitions().unwrap();

        let code = self.module.get_finalized_function(id);

        Ok(code)
    }

    pub fn create_data(&mut self, name: &str, contents: Vec<u8>) -> Result<&[u8], String> {
        self.data_description.define(contents.into_boxed_slice());

        let id = self
            .module
            .declare_data(name, Linkage::Export, true, false)
            .map_err(|e| e.to_string())?;

        self.module
            .define_data(id, &self.data_description)
            .map_err(|e| e.to_string())?;

        self.data_description.clear();
        self.module.finalize_definitions().unwrap();

        let buffer = self.module.get_finalized_data(id);

        Ok(unsafe { slice::from_raw_parts(buffer.0, buffer.1) })
    }

    fn translate(
        &mut self,
        params: Vec<(String, Expr)>,
        the_return: String,
        stmts: Vec<Expr>,
    ) -> Result<()> {
        let int = self.module.target_config().pointer_type();
        let any = self.module.target_config().pointer_type();

        for _p in &params {
            self.ctx.func.signature.params.push(AbiParam::new(int));
        }

        self.ctx.func.signature.returns.push(AbiParam::new(int));

        let mut builder = FunctionBuilder::new(&mut self.ctx.func, &mut self.builder_context);
        let entry_block = builder.create_block();

        builder.append_block_params_for_function_params(entry_block);
        builder.switch_to_block(entry_block);
        builder.seal_block(entry_block);

        let variables =
            declare_variables(int, &mut builder, &params, &the_return, &stmts, entry_block);

        let mut trans = FunctionTranslator {
            any,
            int,
            builder,
            variables,
            module: &mut self.module,
        };

        for expr in stmts {
            trans.translate_expr(expr);
        }

        let return_variable = trans.variables.get(&the_return).unwrap();
        let return_value = trans.builder.use_var(*return_variable);

        trans.builder.ins().return_(&[return_value]);
        trans.builder.finalize();

        Ok(())
    }
}

/// A collection of state used for translating from toy-language AST nodes
/// into Cranelift IR.
struct FunctionTranslator<'a> {
    any: types::Type,
    int: types::Type,
    builder: FunctionBuilder<'a>,
    variables: HashMap<String, Variable>,
    module: &'a mut JITModule,
}

impl<'a> FunctionTranslator<'a> {
    fn translate_expr(&mut self, expr: Expr) -> Value {
        match expr {
            Expr::Literal(literal) => {
                let val = literal.parse::<i32>();

                if let Ok(val) = val {
                    return self.builder.ins().iconst(self.int, i64::from(val));
                }

                let val = literal.parse::<f32>();

                if let Ok(val) = val {
                    return self.builder.ins().f32const(val);
                }

                let mut data_description = DataDescription::new();

                data_description.define(literal.as_bytes().to_vec().into_boxed_slice());

                let name = random_string(8);

                let id = self
                    .module
                    .declare_data(&name, Linkage::Export, true, false)
                    .unwrap();

                self.module
                    .define_data(id, &data_description)
                    .map_err(|e| e.to_string())
                    .unwrap();

                data_description.clear();

                self.module.finalize_definitions().unwrap();

                self.translate_global_data_addr(name)
            }

            Expr::Add(lhs, rhs) => {
                let lhs = self.translate_expr(*lhs);
                let rhs = self.translate_expr(*rhs);
                self.builder.ins().iadd(lhs, rhs)
            }

            Expr::Sub(lhs, rhs) => {
                let lhs = self.translate_expr(*lhs);
                let rhs = self.translate_expr(*rhs);
                self.builder.ins().isub(lhs, rhs)
            }

            Expr::Mul(lhs, rhs) => {
                let lhs = self.translate_expr(*lhs);
                let rhs = self.translate_expr(*rhs);
                self.builder.ins().imul(lhs, rhs)
            }

            Expr::Div(lhs, rhs) => {
                let lhs = self.translate_expr(*lhs);
                let rhs = self.translate_expr(*rhs);
                self.builder.ins().udiv(lhs, rhs)
            }

            Expr::Eq(lhs, rhs) => self.translate_icmp(IntCC::Equal, *lhs, *rhs),
            Expr::Ne(lhs, rhs) => self.translate_icmp(IntCC::NotEqual, *lhs, *rhs),
            Expr::Lt(lhs, rhs) => self.translate_icmp(IntCC::SignedLessThan, *lhs, *rhs),
            Expr::Le(lhs, rhs) => self.translate_icmp(IntCC::SignedLessThanOrEqual, *lhs, *rhs),
            Expr::Gt(lhs, rhs) => self.translate_icmp(IntCC::SignedGreaterThan, *lhs, *rhs),
            Expr::Ge(lhs, rhs) => self.translate_icmp(IntCC::SignedGreaterThanOrEqual, *lhs, *rhs),
            Expr::Call(name, args) => self.translate_call(name, args),
            Expr::GlobalDataAddr(name) => self.translate_global_data_addr(name),

            Expr::Identifier(name) => {
                let variable = self.variables.get(&name).expect("variable not defined");
                self.builder.use_var(*variable)
            }

            Expr::Assign(name, expr) => self.translate_assign(name, *expr),

            Expr::IfElse(condition, then_body, else_body) => {
                self.translate_if_else(*condition, then_body, else_body)
            }

            Expr::WhileLoop(condition, loop_body) => {
                self.translate_while_loop(*condition, loop_body)
            }

            Expr::ForEachLoop(item_name, array, loop_body) => self.translate_for_each_loop(
                match *item_name {
                    Expr::Identifier(name) => name,
                    _ => panic!("Variable name must be an identifier!"),
                },
                match *array {
                    Expr::Identifier(name) => name,
                    _ => panic!("Variable name must be an identifier!"),
                },
                loop_body,
            ),

            Expr::Tuple(_types) => self.builder.ins().null(self.any),

            Expr::Type(_name, _generics) => self.builder.ins().null(self.any),

            Expr::Variable(name, mutable, type_, value) => {
                self.translate_variable(name, mutable, *type_, *value)
            }
        }
    }

    fn translate_assign(&mut self, name: String, expr: Expr) -> Value {
        let new_value = self.translate_expr(expr);
        let variable = self.variables.get(&name).unwrap();
        self.builder.def_var(*variable, new_value);
        new_value
    }

    fn translate_variable(
        &mut self,
        name: String,
        _mutable: bool,
        _type: Option<Expr>,
        expr: Option<Expr>,
    ) -> Value {
        let var = Variable::new(self.variables.capacity());

        self.builder.declare_var(var, self.any);

        self.variables.insert(name, var);

        if let Some(expr) = expr {
            let new_value = self.translate_expr(expr);

            self.builder.def_var(var, new_value);

            return new_value;
        }

        self.builder.ins().null(self.any)
    }

    fn translate_icmp(&mut self, cmp: IntCC, lhs: Expr, rhs: Expr) -> Value {
        let lhs = self.translate_expr(lhs);
        let rhs = self.translate_expr(rhs);
        self.builder.ins().icmp(cmp, lhs, rhs)
    }

    fn translate_if_else(
        &mut self,
        condition: Expr,
        then_body: Vec<Expr>,
        else_body: Vec<Expr>,
    ) -> Value {
        let condition_value = self.translate_expr(condition);

        let then_block = self.builder.create_block();
        let else_block = self.builder.create_block();
        let merge_block = self.builder.create_block();

        self.builder.append_block_param(merge_block, self.int);

        self.builder
            .ins()
            .brif(condition_value, then_block, &[], else_block, &[]);

        self.builder.switch_to_block(then_block);
        self.builder.seal_block(then_block);

        let mut then_return = self.builder.ins().iconst(self.int, 0);

        for expr in then_body {
            then_return = self.translate_expr(expr);
        }

        self.builder.ins().jump(merge_block, &[then_return]);
        self.builder.switch_to_block(else_block);
        self.builder.seal_block(else_block);

        let mut else_return = self.builder.ins().iconst(self.int, 0);

        for expr in else_body {
            else_return = self.translate_expr(expr);
        }

        self.builder.ins().jump(merge_block, &[else_return]);
        self.builder.switch_to_block(merge_block);
        self.builder.seal_block(merge_block);

        let phi = self.builder.block_params(merge_block)[0];

        phi
    }

    fn translate_while_loop(&mut self, condition: Expr, loop_body: Vec<Expr>) -> Value {
        let header_block = self.builder.create_block();
        let body_block = self.builder.create_block();
        let exit_block = self.builder.create_block();

        self.builder.ins().jump(header_block, &[]);
        self.builder.switch_to_block(header_block);

        let condition_value = self.translate_expr(condition);

        self.builder
            .ins()
            .brif(condition_value, body_block, &[], exit_block, &[]);

        self.builder.switch_to_block(body_block);
        self.builder.seal_block(body_block);

        for expr in loop_body {
            self.translate_expr(expr);
        }

        self.builder.ins().jump(header_block, &[]);
        self.builder.switch_to_block(exit_block);
        self.builder.seal_block(header_block);
        self.builder.seal_block(exit_block);
        self.builder.ins().iconst(self.int, 0)
    }

    fn translate_for_each_loop(
        &mut self,
        _item: String,
        var_name: String,
        loop_body: Vec<Expr>,
    ) -> Value {
        let header_block = self.builder.create_block();
        let body_block = self.builder.create_block();
        let exit_block = self.builder.create_block();

        self.builder.ins().jump(header_block, &[]);
        self.builder.switch_to_block(header_block);

        let arr = self
            .builder
            .use_var(*self.variables.get(&var_name).unwrap());

        let var = Variable::new(0);
        let val = self.builder.ins().iconst(self.int, 0);
        let one = self.builder.ins().iconst(self.int, 1);

        self.builder.declare_var(var, self.int);
        self.builder.def_var(var, val);

        let condition_value = self.builder.ins().icmp(IntCC::SignedLessThan, val, arr);

        self.builder
            .ins()
            .brif(condition_value, body_block, &[], exit_block, &[]);

        self.builder.switch_to_block(body_block);
        self.builder.seal_block(body_block);

        for expr in loop_body {
            self.translate_expr(expr);
        }

        self.builder.ins().iadd(val, one);

        self.builder.ins().jump(header_block, &[]);
        self.builder.switch_to_block(exit_block);
        self.builder.seal_block(header_block);
        self.builder.seal_block(exit_block);
        self.builder.ins().iconst(self.int, 0)
    }

    fn translate_call(&mut self, name: String, args: Vec<Expr>) -> Value {
        let mut sig = self.module.make_signature();

        // Add a parameter for each argument.
        for _arg in &args {
            sig.params.push(AbiParam::new(self.int));
        }

        // For simplicity for now, just make all calls return a single I64.
        sig.returns.push(AbiParam::new(self.int));

        // TODO: Streamline the API here?
        let callee = self
            .module
            .declare_function(&name, Linkage::Import, &sig)
            .expect("problem declaring function");
        let local_callee = self.module.declare_func_in_func(callee, self.builder.func);

        let mut arg_values = Vec::new();
        for arg in args {
            arg_values.push(self.translate_expr(arg))
        }
        let call = self.builder.ins().call(local_callee, &arg_values);
        self.builder.inst_results(call)[0]
    }

    fn translate_global_data_addr(&mut self, name: String) -> Value {
        let sym = self
            .module
            .declare_data(&name, Linkage::Export, true, false)
            .expect("problem declaring data object");
        let local_id = self.module.declare_data_in_func(sym, self.builder.func);

        let pointer = self.module.target_config().pointer_type();
        self.builder.ins().symbol_value(pointer, local_id)
    }
}

fn declare_variables(
    int: types::Type,
    builder: &mut FunctionBuilder,
    params: &[(String, Expr)],
    the_return: &String,
    stmts: &[Expr],
    entry_block: Block,
) -> HashMap<String, Variable> {
    let mut variables = HashMap::new();
    let mut index = 0;

    for (i, (name, _)) in params.iter().enumerate() {
        let val = builder.block_params(entry_block)[i];
        let var = declare_variable(int, builder, &mut variables, &mut index, name);

        builder.def_var(var, val);
    }

    let zero = builder.ins().iconst(int, 0);

    let return_variable = declare_variable(int, builder, &mut variables, &mut index, the_return);

    builder.def_var(return_variable, zero);

    for expr in stmts {
        declare_variables_in_stmt(int, builder, &mut variables, &mut index, expr);
    }

    variables
}

/// Recursively descend through the AST, translating all implicit
/// variable declarations.
fn declare_variables_in_stmt(
    int: types::Type,
    builder: &mut FunctionBuilder,
    variables: &mut HashMap<String, Variable>,
    index: &mut usize,
    expr: &Expr,
) {
    match *expr {
        Expr::Assign(ref name, _) => {
            declare_variable(int, builder, variables, index, name);
        }

        Expr::Variable(ref name, _, _, _) => {
            declare_variable(int, builder, variables, index, name);
        }

        Expr::IfElse(ref _condition, ref then_body, ref else_body) => {
            for stmt in then_body {
                declare_variables_in_stmt(int, builder, variables, index, stmt);
            }
            for stmt in else_body {
                declare_variables_in_stmt(int, builder, variables, index, stmt);
            }
        }

        Expr::WhileLoop(ref _condition, ref loop_body) => {
            for stmt in loop_body {
                declare_variables_in_stmt(int, builder, variables, index, stmt);
            }
        }
        _ => (),
    }
}

/// Declare a single variable declaration.
fn declare_variable(
    int: types::Type,
    builder: &mut FunctionBuilder,
    variables: &mut HashMap<String, Variable>,
    index: &mut usize,
    name: &String,
) -> Variable {
    let var = Variable::new(*index);
    if !variables.contains_key(name) {
        variables.insert(name.into(), var);
        builder.declare_var(var, int);
        *index += 1;
    }
    var
}
