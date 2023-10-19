/// The AST node for expressions.
pub enum Expr {
    /// A literal.
    /// - The literal name.
    Literal(String),

    /// An identifier.
    /// - The identifier.
    Identifier(String),

    /// An assignment operator.
    /// - The variable.
    /// - The value.
    Assign(String, Box<Expr>),

    /// An equality expression (a == b).
    /// - Lhs operand.
    /// - Rhs operand.
    Eq(Box<Expr>, Box<Expr>),

    /// An inequality expression (a != b).
    /// - Lhs operand.
    /// - Rhs operand.
    Ne(Box<Expr>, Box<Expr>),

    /// A less than expression (a < b).
    /// - Lhs operand.
    /// - Rhs operand.
    Lt(Box<Expr>, Box<Expr>),

    /// A less than or equal to expression (a <= b).
    /// - Lhs operand.
    /// - Rhs operand.
    Le(Box<Expr>, Box<Expr>),

    /// A greater than expression (a > b).
    /// - Lhs operand.
    /// - Rhs operand.
    Gt(Box<Expr>, Box<Expr>),

    /// A greather than or equal to expression (a >= b).
    /// - Lhs operand.
    /// - Rhs operand.
    Ge(Box<Expr>, Box<Expr>),

    /// An addition operation (a + b).
    /// - Lhs operand.
    /// - Rhs operand.
    Add(Box<Expr>, Box<Expr>),

    /// A subtraction operation (a - b).
    /// - Lhs operand.
    /// - Rhs operand.
    Sub(Box<Expr>, Box<Expr>),

    /// A multiplication operation (a * b).
    /// - Lhs operand.
    /// - Rhs operand.
    Mul(Box<Expr>, Box<Expr>),

    /// A division operation (a / b).
    /// - Lhs operand.
    /// - Rhs operand.
    Div(Box<Expr>, Box<Expr>),

    /// An if/else statement.
    /// - The condition.
    /// - The statement(s) to be evaluated if true.
    /// - The statement(s) to be evaluated if false.
    IfElse(Box<Expr>, Vec<Expr>, Vec<Expr>),

    /// A while loop.
    /// - The condition.
    /// - The statement(s) to be evaluated.
    WhileLoop(Box<Expr>, Vec<Expr>),

    /// A foreach loop.
    /// - The item identifier.
    /// - The container identifier.
    /// - The statement(s) to be evaluated.
    ForEachLoop(Box<Expr>, Box<Expr>, Vec<Expr>),

    /// A function call.
    /// - The function name.
    /// - Any arguments.
    Call(String, Vec<Expr>),

    /// A reference to global data.
    /// - The global data address.
    GlobalDataAddr(String),

    /// A variable definition.
    /// - The variable name.
    /// - Mutability status.
    /// - The optional type (otherwise auto-detected).
    /// - The optional initial value.
    Variable(String, bool, Box<Option<Expr>>, Box<Option<Expr>>),

    /// A type name.
    /// - The type name.
    /// - Any generic type(s).
    Type(Box<Expr>, Box<Option<Vec<Expr>>>),

    /// A tuple type.
    /// - The inner types.
    Tuple(Vec<Expr>),
}

/// A function.
/// - The function name.
/// - The function's arguments.
/// - The return type.
/// - The content.
pub struct Function(pub String, pub Vec<(String, Expr)>, pub String, pub Vec<Expr>);

peg::parser!(pub grammar parser() for str {
    pub rule function() -> Function
        = [' ' | '\t' | '\n']* "fn" _ name:identifier() _
        "(" params:((_ i:identifier() _ ":" _ t:type_() _ {(i, t)}) ** ",") ")" _
        "->" _
        returns:identifier() _
        "{" _ "\n"
        stmts:statements()
        _ "}" _ "\n" _
        { Function(name, params, returns, stmts) }
    
    pub rule root() -> Vec<Function>
        = f:function() *
        { f }

    pub rule statements() -> Vec<Expr>
        = s:(statement()*) { s }

    pub rule statement() -> Expr
        = _ e:expression() _ "\n" { e }

    pub rule type_() -> Expr
        = type_name()
        / tuple_type()

    pub rule type_name() -> Expr
        = _ i:identifier() _ g:("<" _ g:((_ g:type_() _ {g}) ** ",") _ ">" {g})?
        { Expr::Type(Box::new(Expr::Identifier(i)), Box::new(g)) }
    
    pub rule tuple_type() -> Expr
        = "(" _ ty:(t:type_() ** "," {t}) _ ")"
        { Expr::Tuple(ty) }

    pub rule expression() -> Expr
        = if_else()
        / while_loop()
        / for_each_loop()
        / assignment()
        / binary_op()

    pub rule if_else() -> Expr
        = "if" _ "(" _ e:expression() _ ")" _ "{" _ "\n"
        then_body:statements() _ "}" _ "else" _ "{" _ "\n"
        else_body:statements() _ "}"
        { Expr::IfElse(Box::new(e), then_body, else_body) }

    pub rule while_loop() -> Expr
        = "while" _ "(" _ e:expression() _ ")" _ "{" _ "\n"
        loop_body:statements() _ "}"
        { Expr::WhileLoop(Box::new(e), loop_body) }
    
    pub rule for_each_loop() -> Expr
        = "foreach" _ "(" _ n:expression() _ "in" _ c:expression() ")" _ "{" _ "\n"
        loop_body:statements() _ "}"
        { Expr::ForEachLoop(Box::new(n), Box::new(c), loop_body) }
    
    pub rule variable() -> Expr
        = "let" _ m:"mut"? _ i:identifier() _ t:(":" _ t:type_() {t})? _ e:("=" _ e:expression() {e})? _ ";"
        { Expr::Variable(i, m.is_some(), Box::new(t), Box::new(e)) }

    pub rule assignment() -> Expr
        = i:identifier() _ "=" _ e:expression() _ ";" {Expr::Assign(i, Box::new(e))}

    pub rule binary_op() -> Expr = precedence!{
        a:@ _ "==" _ b:(@) { Expr::Eq(Box::new(a), Box::new(b)) }
        a:@ _ "!=" _ b:(@) { Expr::Ne(Box::new(a), Box::new(b)) }
        a:@ _ "<"  _ b:(@) { Expr::Lt(Box::new(a), Box::new(b)) }
        a:@ _ "<=" _ b:(@) { Expr::Le(Box::new(a), Box::new(b)) }
        a:@ _ ">"  _ b:(@) { Expr::Gt(Box::new(a), Box::new(b)) }
        a:@ _ ">=" _ b:(@) { Expr::Ge(Box::new(a), Box::new(b)) }
        --
        a:@ _ "+" _ b:(@) { Expr::Add(Box::new(a), Box::new(b)) }
        a:@ _ "-" _ b:(@) { Expr::Sub(Box::new(a), Box::new(b)) }
        --
        a:@ _ "*" _ b:(@) { Expr::Mul(Box::new(a), Box::new(b)) }
        a:@ _ "/" _ b:(@) { Expr::Div(Box::new(a), Box::new(b)) }
        --
        i:identifier() _ "(" args:((_ e:expression() _ {e}) ** ",") ")" _ ";" { Expr::Call(i, args) }
        i:identifier() { Expr::Identifier(i) }
        l:literal() { l }
    }

    pub rule identifier() -> String
        = quiet!{ n:$(['a'..='z' | 'A'..='Z' | '_']['a'..='z' | 'A'..='Z' | '0'..='9' | '_']*) { n.to_owned() } }
        / expected!("identifier")

    pub rule literal() -> Expr
        = n:$(['0'..='9']+) { Expr::Literal(n.to_owned()) }
        / "&" i:identifier() { Expr::GlobalDataAddr(i) }
        / "\"" _ v:$([^'"']+) _ "\"" { Expr::Literal(v.to_string()) }

    rule _() =  quiet!{[' ' | '\t']*}
});
