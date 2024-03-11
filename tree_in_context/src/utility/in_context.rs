#[cfg(test)]
use crate::{egglog_test, interpreter::Value, schema::Constant};

#[test]
fn test_in_context_in_func() -> crate::Result {
    use crate::ast::*;
    let expr = function("main", base(intt()), base(intt()), int(2)).func_with_arg_types();
    let expected = function(
        "main",
        base(intt()),
        base(intt()),
        in_context(infunc("main"), int(2)),
    )
    .func_with_arg_types();
    egglog_test(
        &format!("{expr}"),
        &format!("(check (= {expr} {expected}))"),
        vec![
            expr.to_program(emptyt(), base(intt())),
            expected.to_program(emptyt(), base(intt())),
        ],
        Value::Tuple(vec![]),
        Value::Const(Constant::Int(2)),
        vec![],
    )
}

#[test]
fn test_in_context_two_lets() -> crate::Result {
    use crate::ast::*;
    let expr = function(
        "main",
        base(intt()),
        base(intt()),
        tlet(int(1), tlet(add(iarg(), iarg()), mul(iarg(), int(2)))),
    )
    .func_with_arg_types();
    let int1 = in_context(infunc("main"), int(1)).with_arg_types(base(intt()), base(intt()));
    let arg1 = in_context(inlet(int1.clone()), iarg());
    let addarg1 = add(arg1.clone(), arg1.clone());
    let int2 = in_context(inlet(addarg1.clone()), int_ty(2, base(intt())));
    let arg2 = in_context(inlet(addarg1.clone()), iarg());
    let expr2 = function(
        "main",
        base(intt()),
        base(intt()),
        tlet(
            int1,
            tlet(
                add(arg1.clone(), arg1.clone()),
                mul(arg2.clone(), int2.clone()),
            ),
        ),
    )
    .func_with_arg_types();

    egglog_test(
        &format!("{expr}"),
        &format!("(check (= {expr} {expr2}))"),
        vec![
            expr.to_program(emptyt(), base(intt())),
            expr2.to_program(emptyt(), base(intt())),
        ],
        Value::Tuple(vec![]),
        Value::Const(Constant::Int(4)),
        vec![],
    )
}

#[test]
fn test_switch_contexts() -> crate::Result {
    use crate::ast::*;
    let expr = function(
        "main",
        base(intt()),
        base(intt()),
        tif(ttrue(), int(1), int(2)),
    )
    .func_with_arg_types();
    let pred = in_context(infunc("main"), ttrue_ty(base(intt())));
    let expr2 = function(
        "main",
        base(intt()),
        base(intt()),
        tif(
            pred.clone(),
            in_context(inif(true, pred.clone()), int(1)),
            in_context(inif(false, pred.clone()), int(2)),
        ),
    )
    .func_with_arg_types();
    egglog_test(
        &format!("{expr}"),
        &format!("(check (= {expr} {expr2}))"),
        vec![
            expr.to_program(emptyt(), base(intt())),
            expr2.to_program(emptyt(), base(intt())),
        ],
        Value::Tuple(vec![]),
        Value::Const(Constant::Int(1)),
        vec![],
    )
}

#[test]
fn test_dowhile_cycle_in_context() -> crate::Result {
    use crate::ast::*;
    // loop runs one iteration and returns 3
    let myloop = dowhile(arg(), parallel!(tfalse(), int(3)));
    let expr = function("main", tuplet!(intt()), tuplet!(intt()), myloop).func_with_arg_types();
    let int3func =
        function("main", tuplet!(intt()), tuplet!(intt()), single(int(3))).func_with_arg_types();

    let fargincontext = in_context(infunc("main"), arg_ty(tuplet!(intt())));
    let inner_in_context = inloop(
        fargincontext.clone(),
        parallel!(tfalse(), int(3)).with_arg_types(tuplet!(intt()), tuplet!(boolt(), intt())),
    );
    let expr_intermediate = function(
        "main",
        tuplet!(intt()),
        tuplet!(intt()),
        dowhile(
            fargincontext.clone(),
            in_context(inner_in_context.clone(), parallel!(tfalse(), int(3))),
        ),
    )
    .func_with_arg_types();
    let expr2 = function(
        "main",
        tuplet!(intt()),
        tuplet!(intt()),
        dowhile(
            fargincontext.clone(),
            parallel!(
                in_context(inner_in_context.clone(), tfalse()),
                in_context(inner_in_context.clone(), int(3)),
            ),
        ),
    )
    .func_with_arg_types();

    egglog_test(
        &format!("{expr}",),
        &format!(
            "
(check (ContextLess {expr}))
(check (= {expr} {expr_intermediate}))
(check (= {expr} {expr2}))
(check (= {expr} {int3func}))"
        ),
        vec![
            expr.to_program(tuplet!(intt()), tuplet!(intt())),
            expr2.to_program(tuplet!(intt()), tuplet!(intt())),
        ],
        Value::Tuple(vec![Value::Const(Constant::Int(3))]),
        Value::Tuple(vec![Value::Const(Constant::Int(3))]),
        vec![],
    )
}
