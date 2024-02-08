use crate::schema::{
    Assumption, BaseType, BinaryOp, Expr, Order, RcExpr, TreeProgram, Type, UnaryOp,
};

pub fn intt() -> Type {
    Type::Base(BaseType::IntT)
}

pub fn boolt() -> Type {
    Type::Base(BaseType::BoolT)
}

pub fn emptyt() -> Type {
    Type::TupleT(vec![])
}

pub fn tuplet_vec(types: Vec<BaseType>) -> Type {
    Type::TupleT(types)
}

/// Construct a tuple type from the child types
/// e.g. `tuple!(intt(), boolt())` becomes `Type::TupleT(vec![BaseType::IntT, BaseType::BoolT])`
#[macro_export]
macro_rules! tuplet {
    ($($x:expr),* $(,)?) => ($crate::ast::tuplet_vec(vec![$(std::rc::Rc::new($x)),*]))
}
pub use tuplet;

pub fn add(l: RcExpr, r: RcExpr) -> RcExpr {
    RcExpr::new(Expr::Bop(BinaryOp::Add, l, r))
}

pub fn sub(l: RcExpr, r: RcExpr) -> RcExpr {
    RcExpr::new(Expr::Bop(BinaryOp::Sub, l, r))
}

pub fn mul(l: RcExpr, r: RcExpr) -> RcExpr {
    RcExpr::new(Expr::Bop(BinaryOp::Mul, l, r))
}

pub fn less_than(l: RcExpr, r: RcExpr) -> RcExpr {
    RcExpr::new(Expr::Bop(BinaryOp::LessThan, l, r))
}

pub fn and(l: RcExpr, r: RcExpr) -> RcExpr {
    RcExpr::new(Expr::Bop(BinaryOp::And, l, r))
}

pub fn or(l: RcExpr, r: RcExpr) -> RcExpr {
    RcExpr::new(Expr::Bop(BinaryOp::Or, l, r))
}

pub fn not(e: RcExpr) -> RcExpr {
    RcExpr::new(Expr::Uop(UnaryOp::Not, e))
}

pub fn twrite(addr: RcExpr, val: RcExpr) -> RcExpr {
    RcExpr::new(Expr::Bop(BinaryOp::Write, addr, val))
}

pub fn tprint(e: RcExpr) -> RcExpr {
    RcExpr::new(Expr::Uop(UnaryOp::Print, e))
}

pub fn get(e: RcExpr, i: usize) -> RcExpr {
    RcExpr::new(Expr::Get(e, i))
}

pub fn first(e: RcExpr) -> RcExpr {
    get(e, 0)
}

pub fn second(e: RcExpr) -> RcExpr {
    get(e, 1)
}
pub fn write(ptr: RcExpr, val: RcExpr) -> RcExpr {
    RcExpr::new(Expr::Bop(BinaryOp::Write, ptr, val))
}

pub fn load(e: RcExpr) -> RcExpr {
    RcExpr::new(Expr::Uop(UnaryOp::Load, e))
}

pub fn ptradd(ptr: RcExpr, i: RcExpr) -> RcExpr {
    RcExpr::new(Expr::Bop(BinaryOp::PtrAdd, ptr, i))
}

pub fn alloc(e: RcExpr, ty: Type) -> RcExpr {
    RcExpr::new(Expr::Alloc(e, ty))
}

pub fn call(s: &str, e: RcExpr) -> RcExpr {
    RcExpr::new(Expr::Call(s.to_string(), e))
}
/// a macro that wraps the children in
/// a vec for program
/// e.g. `program!(main, f1, f2, f3)` becomes `TreeProgram { entry: main, functions: vec![f1, f2, f3] }`
#[macro_export]
macro_rules! program {
    ($main:expr, $($x:expr),* $(,)?) => ($crate::ast::program_vec($main, vec![$($x),*]))
}
pub use program;

pub fn program_vec(entry: RcExpr, functions: Vec<RcExpr>) -> TreeProgram {
    TreeProgram { entry, functions }
}

/// Create a switch given a predicate and a list of cases
/// e.g. `switch!(cond; case1, case2, case3)` becomes `switch_vec(cond, vec![case1, case2, case3])`
#[macro_export]
macro_rules! switch {
    ($arg:expr; $($x:expr),* $(,)?) => ($crate::ast::switch_vec($arg, vec![$($x),*]))
}
pub use switch;

pub fn switch_vec(cond: RcExpr, cases: Vec<RcExpr>) -> RcExpr {
    RcExpr::new(Expr::Switch(cond, cases))
}

pub fn empty() -> RcExpr {
    RcExpr::new(Expr::Empty)
}

pub fn single(e: RcExpr) -> RcExpr {
    RcExpr::new(Expr::Single(e))
}

pub fn cons_par(l: RcExpr, r: RcExpr) -> RcExpr {
    RcExpr::new(Expr::Concat(Order::Parallel, single(l), r))
}

pub fn push_par(l: RcExpr, r: RcExpr) -> RcExpr {
    RcExpr::new(Expr::Concat(Order::Parallel, r, single(l)))
}

pub fn push_seq(l: RcExpr, r: RcExpr) -> RcExpr {
    RcExpr::new(Expr::Concat(Order::Sequential, r, single(l)))
}

pub fn push_rev(l: RcExpr, r: RcExpr) -> RcExpr {
    RcExpr::new(Expr::Concat(Order::Reversed, r, single(l)))
}

pub fn concat_par(tuple: RcExpr, tuple2: RcExpr) -> RcExpr {
    RcExpr::new(Expr::Concat(Order::Parallel, tuple, tuple2))
}

pub fn concat_seq(tuple: RcExpr, tuple2: RcExpr) -> RcExpr {
    RcExpr::new(Expr::Concat(Order::Sequential, tuple, tuple2))
}

pub fn concat_rev(tuple: RcExpr, tuple2: RcExpr) -> RcExpr {
    RcExpr::new(Expr::Concat(Order::Reversed, tuple, tuple2))
}

/// Create a tuple where each element can be executed
/// in any order.
/// e.g. `parallel!(e1, e2, e3)` becomes `Concat(Order::Parallel, Concat(Order::Parallel, e1, e2), e3)`
#[macro_export]
macro_rules! parallel {
    ($($x:expr),* $(,)?) => ($crate::ast::parallel_vec(vec![$($x),*]))
}
pub use parallel;

pub fn parallel_vec(es: impl IntoIterator<Item = RcExpr>) -> RcExpr {
    es.into_iter().fold(empty(), |acc, x| push_par(x, acc))
}

pub fn tlet(lhs: RcExpr, rhs: RcExpr) -> RcExpr {
    RcExpr::new(Expr::Let(lhs, rhs))
}

pub fn arg() -> RcExpr {
    RcExpr::new(Expr::Arg)
}

pub fn getat(index: usize) -> RcExpr {
    get(arg(), index)
}

pub fn tif(cond: RcExpr, then_case: RcExpr, else_case: RcExpr) -> RcExpr {
    RcExpr::new(Expr::If(cond, then_case, else_case))
}

pub fn dowhile(inputs: RcExpr, pred_and_body: RcExpr) -> RcExpr {
    RcExpr::new(Expr::DoWhile(inputs, pred_and_body))
}

pub fn function(name: &str, arg_ty: Type, ret_ty: Type, body: RcExpr) -> RcExpr {
    RcExpr::new(Expr::Function(name.to_string(), arg_ty, ret_ty, body))
}

pub fn ttrue() -> RcExpr {
    RcExpr::new(Expr::Const(crate::schema::Constant::Bool(true)))
}

pub fn tfalse() -> RcExpr {
    RcExpr::new(Expr::Const(crate::schema::Constant::Bool(false)))
}

pub fn int(i: i64) -> RcExpr {
    RcExpr::new(Expr::Const(crate::schema::Constant::Int(i)))
}

pub fn inlet(e: RcExpr) -> Assumption {
    Assumption::InLet(e)
}

pub fn inloop(e1: RcExpr, e2: RcExpr) -> Assumption {
    Assumption::InLoop(e1, e2)
}

pub fn inif(is_then: bool, pred: RcExpr) -> Assumption {
    Assumption::InIf(is_then, pred)
}

pub fn infunc(name: &str) -> Assumption {
    Assumption::InFunc(name.to_string())
}

pub fn assume(assumption: Assumption, body: RcExpr) -> RcExpr {
    RcExpr::new(Expr::Assume(assumption, body))
}
