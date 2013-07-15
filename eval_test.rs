extern mod r5;

use std::io;
use r5::runtime::{Runtime};
use r5::parser::Parser;

fn eval_test(src: ~str, expected_src: ~str) {
    let expr =
    do io::with_str_reader(src) |rdr| {
        let mut parser = Parser(rdr);
        match parser.parse_datum() {
            Ok(v) => @v,
            Err(e) => fail!(e),
        }
    };

    let expected =
    do io::with_str_reader(expected_src) |rdr| {
        let mut parser = Parser(rdr);
        match parser.parse_datum() {
            Ok(v) => @v,
            Err(e) => fail!(e),
        }
    };

    let mut runtime = Runtime::new_std();

    let val = runtime.eval(expr);

    match val {
        Ok(v) => if v != expected {
            fail!(fmt!("expected %s, got %s", expected.to_str(), v.to_str()))
        },
        Err(e) => 
            fail!(e.to_str()),
    }
}

#[test]
fn add_test() {
    eval_test(~"(+ 1 2)", ~"3");
}

#[test]
fn mul_test() {
    eval_test(~"(* 1 2 3)", ~"6");
}

#[test]
fn quote_test() {
    eval_test(~"'(a b)", ~"(a b)");
}

#[test]
fn quote_macro_test() {
    eval_test(~"(quote (a b))", ~"(a b)");
}

#[test]
fn car_test() {
    eval_test(~"(car '(a b))", ~"a");
}

#[test]
fn cdr_test() {
    eval_test(~"(cdr '(a b))", ~"(b)");
}

#[test]
fn if_true_test() {
    eval_test(~"(if #t 1 0)", ~"1");
}

#[test]
fn if_false_test() {
    eval_test(~"(if #f 1 0)", ~"0");
}

#[test]
fn lambda_test() {
    eval_test(~"((lambda (x y) (+ x y)) 1 2)", ~"3");
}

#[test]
fn quasiquote_test() {
    eval_test(~"`(list ,(+ 1 2) 4)", ~"(list 3 4)");
}

#[test]
fn nested_quasiquote_test() {
    eval_test(~"`(a `(b ,(+ 1 2) ,(foo ,(+ 1 3) d) e) f)", ~"(a `(b ,(+ 1 2) ,(foo 4 d) e) f)");
}

#[test]
fn lexical_scoping_test() {
    /*
    (\y f -> f(2)) dyn ((\y -> (\x -> y)) lex)
    lexical scoping returns lex
    dynamic scoping returns dyn
    */

    let src = ~"((lambda (y f) (f 2)) 'dyn ((lambda (y) (lambda (x) y)) 'lex))";
    eval_test(src, ~"lex");
}

#[test]
fn eqv_test() {
    eval_test(~"(eqv? 3 (+ 1 2))", ~"#t");
}

#[test]
fn set_test() {
    eval_test(~"((lambda (x) (set! x 'mut) x) 'immut)", ~"mut");
}

#[test]
fn varargs_test() {
    eval_test(~"((lambda (a b . c) `(,(+ a b) . ,c)) 1 2 3 4)", ~"(3 3 4)");
}

#[test]
fn define_val_test() {
    eval_test(~"((lambda () (define a 2) (+ a 1)))", ~"3");
}

#[test]
fn define_fun_test() {
    eval_test(~"((lambda () (define (list . a) a) (list 1 2)))", ~"(1 2)");
}

#[test]
fn begin_test() {
    eval_test(~"(begin (+ 1 2) (+ 2 3))", ~"5");
}

#[test]
fn num_fun_test() {
    eval_test(~"(number? 3)", ~"#t");
    eval_test(~"(number? 3+4i)", ~"#t");
    eval_test(~"(real? 3)", ~"#t");
    eval_test(~"(real? -2.5+0.0i)", ~"#t");
    eval_test(~"(real? #e1e10)", ~"#t");
    eval_test(~"(integer? 3+0i)", ~"#t");
    eval_test(~"(integer? 3.0)", ~"#t");
    eval_test(~"(integer? 8/4)", ~"#t");
}

#[test]
fn num_cmp_test() {
    eval_test(~"(< 1 2 3)", ~"#t");
    eval_test(~"(< 1 2 2)", ~"#f");
    eval_test(~"(<= 1.0 2.0 3.0)", ~"#t");
    eval_test(~"(<= 1.0 2.0 2.0)", ~"#t");
    eval_test(~"(= #e2 #e4/2 #e2.0)", ~"#t");
    eval_test(~"(> 1 2 3)", ~"#f");
}
