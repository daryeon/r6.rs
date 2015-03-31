use std::iter::FromIterator;
use std::num::Float;
use std::cmp::PartialOrd;
use std::ops::Deref;

use num_trait::Number;
use real::Real;
use datum::{Datum, DatumType};
use error::{RuntimeError, RuntimeErrorKind};
use runtime::DatumCast;

pub trait PrimFunc {
    fn call(&self, Vec<Datum>) -> Result<Datum, RuntimeError>;
}

pub struct Fold<P> {
    fold: fn(Vec<P>) -> P
}

pub struct Fold1<P> {
    fold1: fn(P, Vec<P>) -> P
}

pub struct Fold2<P, R> {
    fold2: fn(P, P, Vec<P>) -> R
}

pub struct FoldR2<P, R> {
    fold_r2: fn(&P, &P, &[P]) -> R
}

pub struct Fold1Err<P> {
    fold1: fn(P, Vec<P>) -> Result<P, RuntimeError>
}

pub struct F1<T0, R> {
    f1: fn(T0) -> R
}

pub struct R1<T0, R> {
    r1: fn(&T0) -> R
}

impl<T> PrimFunc for Fold<T> where T: DatumCast {
    fn call(&self, args: Vec<Datum>) -> Result<Datum, RuntimeError> {
        let p_args:Result<Vec<T>, RuntimeError> = args.into_iter().map(DatumCast::unwrap).collect();
        let f = self.fold;
        p_args.map(|v| f(v).wrap())
    }
}

impl PrimFunc for Fold<Datum> {
    fn call(&self, args: Vec<Datum>) -> Result<Datum, RuntimeError> {
        let f = self.fold;
        Ok(f(args))
    }
}

impl<T> PrimFunc for Fold1<T> where T: DatumCast {
    fn call(&self, args: Vec<Datum>) -> Result<Datum, RuntimeError> {
        let p_args:Result<Vec<T>, RuntimeError> = args.into_iter().map(DatumCast::unwrap).collect();
        let f = self.fold1;
        p_args.and_then(|mut vs|
            if vs.len() < 1 {
                Err(RuntimeError {
                    kind: RuntimeErrorKind::NumArgs,
                    desc: "Expected at least 1 arguments, received 0".to_string()
                })
            } else {
                let v0 = vs.remove(0);
                Ok(f(v0, vs).wrap())
            }
        )
    }
}

impl<T, R> PrimFunc for Fold2<T, R> where T: DatumCast, R: DatumCast {
    fn call(&self, args: Vec<Datum>) -> Result<Datum, RuntimeError> {
        let p_args:Result<Vec<T>, RuntimeError> = args.into_iter().map(DatumCast::unwrap).collect();
        let f = self.fold2;
        p_args.and_then(|mut vs|
            if vs.len() < 2 {
                Err(RuntimeError {
                    kind: RuntimeErrorKind::NumArgs,
                    desc: format!("Expected at least 1 arguments, received {}", vs.len())
                })
            } else {
                let rems = vs.split_off(2);
                let v1 = vs.remove(1);
                let v0 = vs.remove(0);
                Ok(f(v0, v1, rems).wrap())
            }
        )
    }
}

impl<T> PrimFunc for Fold1Err<T> where T: DatumCast {
    fn call(&self, args: Vec<Datum>) -> Result<Datum, RuntimeError> {
        let p_args:Result<Vec<T>, RuntimeError> = args.into_iter().map(DatumCast::unwrap).collect();
        let f = self.fold1;
        p_args.and_then(|mut vs|
            if vs.len() < 1 {
                Err(RuntimeError {
                    kind: RuntimeErrorKind::NumArgs,
                    desc: "Expected at least 1 arguments, received 0".to_string()
                })
            } else {
                let v0 = vs.remove(0);
                f(v0, vs).map(|res| res.wrap())
            }
        )
    }
}

impl<P: DatumCast, R: DatumCast> PrimFunc for FoldR2<P, R> {
    fn call(&self, args: Vec<Datum>) -> Result<Datum, RuntimeError> {
        if args.len() < 2 {
            return Err(RuntimeError {
                kind: RuntimeErrorKind::NumArgs,
                desc: format!("Expected 2 or more argument, received {:?}", args.len())
            });
        }

        let vs: Vec<P> = try!(args.into_iter().map(DatumCast::unwrap).collect());
        let f = self.fold_r2;
        Ok(DatumCast::wrap(f(&vs[0], &vs[1], &vs[2..])))
    }
}

impl<R: DatumCast> PrimFunc for R1<Datum, R> {
    fn call(&self, mut args: Vec<Datum>) -> Result<Datum, RuntimeError> {
        if args.len() != 1 {
            return Err(RuntimeError {
                kind: RuntimeErrorKind::NumArgs,
                desc: format!("Expected 1 argument, received {:?}", args.len())
            });
        }
        let f = self.r1;

        Ok(f(&args.remove(0)).wrap())
    }
}

impl<T0: DatumCast, R: DatumCast> PrimFunc for R1<T0, R> {
    fn call(&self, mut args: Vec<Datum>) -> Result<Datum, RuntimeError> {
        if args.len() != 1 {
            return Err(RuntimeError {
                kind: RuntimeErrorKind::NumArgs,
                desc: format!("Expected 1 argument, received {:?}", args.len())
            });
        }
        let f = self.r1;

        DatumCast::unwrap(args.remove(0)).map(|v| f(&v).wrap())
    }
}

impl<T0: DatumCast> PrimFunc for F1<T0, Datum> {
    fn call(&self, mut args: Vec<Datum>) -> Result<Datum, RuntimeError> {
        if args.len() != 1 {
            return Err(RuntimeError {
                kind: RuntimeErrorKind::NumArgs,
                desc: format!("Expected 1 argument, received {:?}", args.len())
            });
        }
        DatumCast::unwrap(args.pop().unwrap()).map(self.f1)
    }
}

impl<T0: DatumCast, R: DatumCast> PrimFunc for F1<T0, R> {
    fn call(&self, mut args: Vec<Datum>) -> Result<Datum, RuntimeError> {
        if args.len() != 1 {
            return Err(RuntimeError {
                kind: RuntimeErrorKind::NumArgs,
                desc: format!("Expected 1 argument, received {:?}", args.len())
            });
        }
        let f = self.f1;
        DatumCast::unwrap(args.remove(0)).map(|v| f(v).wrap())
    }
}

fn add(args: Vec<Box<Number>>) -> Box<Number> {
    let mut sum:Box<Number> = box Real::Fixnum(0);
    for a in args.into_iter() {
        sum = sum.deref() + a.deref();
    }
    return sum;
}

/// `(+ n0 n1 ...)`
pub static PRIM_ADD:Fold<Box<Number>> = Fold { fold: add };

fn mul(args: Vec<Box<Number>>) -> Box<Number> {
    let mut product:Box<Number> = box Real::Fixnum(1);
    for a in args.iter() {
        product = product.deref() * a.deref();
    }
    return product;
}

/// `(* n0 n1 ...)`
pub static PRIM_MUL:Fold<Box<Number>> = Fold { fold: mul };

fn sub(arg0: Box<Number>, args: Vec<Box<Number>>) -> Box<Number> {
    if args.len() == 0 {
        let zero = Real::Fixnum(0);
        let ref_zero: &Number = &zero;
        return ref_zero - arg0.deref();
    }
    let mut sum:Box<Number> = arg0;
    for a in args.into_iter() {
        sum = sum.deref() - a.deref();
    }
    return sum;
}

/// `(- n0 n1 ...)`
pub static PRIM_SUB:Fold1<Box<Number>> = Fold1 { fold1: sub };

fn div(arg0: Box<Number>, args: Vec<Box<Number>>) -> Result<Box<Number>, RuntimeError> {
    if args.len() == 0 {
        let arg_ref = arg0.deref();
        if arg_ref.is_exact() && arg_ref.num_is_zero() {
            return Err(RuntimeError {
                kind: RuntimeErrorKind::DivideByZero,
                desc: "Tried to divied by 0".to_string()
            });
        }
        let one = Real::Fixnum(1);
        let one_ref: &Number = &one;
        return Ok(one_ref / arg_ref);
    }

    let mut product:Box<Number> = arg0;
    for a in args.into_iter() {
        let arg_ref = a.deref();
        if arg_ref.is_exact() && arg_ref.num_is_zero() {
            return Err(RuntimeError {
                kind: RuntimeErrorKind::DivideByZero,
                desc: "Tried to divied by 0".to_string()
            });
        }
        product = product.deref() / arg_ref;
    }

    return Ok(product);
}

/// `(/ n0 n1 ...)`
pub static PRIM_DIV:Fold1Err<Box<Number>> = Fold1Err { fold1: div };

fn list(args: Vec<Datum>) -> Datum {
    debug!("list: args = {:?}", args);
    FromIterator::from_iter(args.into_iter())
}

/// `(list a0 a1 ...)`
pub static PRIM_LIST:Fold<Datum> = Fold { fold: list };

fn car(arg: (Datum, Datum)) -> Datum {
    arg.0
}

/// `(car x)`
pub static PRIM_CAR: F1<(Datum, Datum), Datum> = F1 { f1: car };

fn cdr(arg: (Datum, Datum)) -> Datum {
    arg.1
}

/// `(cdr x)`
pub static PRIM_CDR: F1<(Datum, Datum), Datum> = F1 { f1: cdr };

fn is_zero(arg: Box<Number>) -> bool {
    arg.deref().num_is_zero()
}

/// `(zero? x)`
pub static PRIM_ZERO: F1<Box<Number>, bool> = F1 { f1: is_zero };

fn is_real(arg: &Datum) -> bool {
    if let &Datum::Ptr(ref ptr) = arg {
        ptr.get_number().and_then(Number::get_real).is_some()
    } else {
        false
    }
}

/// `(real? x)`
pub static PRIM_REAL: R1<Datum, bool> = R1 { r1: is_real };

fn is_rational(arg: &Datum) -> bool {
    if let &Datum::Ptr(ref ptr) = arg {
        match ptr.get_number().and_then(Number::get_real) {
            Some(&Real::Flonum(f)) => f.is_finite(),
            Some(_) => true,
            None => false
        }
    } else {
        false
    }
}

/// `(rational? x)`
pub static PRIM_RATIONAL: R1<Datum, bool> = R1 { r1: is_rational };

fn is_integer(arg: &Datum) -> bool {
    if let &Datum::Ptr(ref ptr) = arg {
        if let Some(r) = ptr.get_number().and_then(Number::get_real) {
            return r.is_integer();
        }
    }

    return false;
}

/// `(integer? x)`
pub static PRIM_INTEGER: R1<Datum, bool> = R1 { r1: is_integer };

fn num_eq(arg0: Box<Number>, arg1: Box<Number>, args: Vec<Box<Number>>) -> bool {
    if &arg0 != &arg1 {
        return false;
    }
    let mut last_arg = arg1;
    for arg in args.into_iter() {
        if &last_arg != &arg {
            return false;
        }
        last_arg = arg;
    }
    return true;
}

pub static PRIM_NUM_EQ: Fold2<Box<Number>, bool> = Fold2 { fold2: num_eq };

macro_rules! impl_num_comp {
    ($type_name:ty, $static_name:ident, $func_name:ident, $op:ident) => (
        fn $func_name(arg0: &$type_name, arg1: &$type_name, args: &[$type_name]) -> bool {
            if !arg0.$op(arg1) {
                return false;
            }
            let mut last_arg = arg1;
            for arg in args.iter() {
                if !last_arg.$op(arg) {
                    return false;
                }
                last_arg = arg;
            }
            return true;
        }

        pub static $static_name: FoldR2<$type_name, bool> = FoldR2 { fold_r2: $func_name };
    )
}

impl_num_comp!(Real, PRIM_LT, real_lt, lt);
impl_num_comp!(Real, PRIM_GT, real_gt, gt);
impl_num_comp!(Real, PRIM_LE, real_le, le);
impl_num_comp!(Real, PRIM_GE, real_ge, ge);

macro_rules! impl_typecheck {
    ($static_name:ident, $func_name:ident, $type_name:ident) => (
        fn $func_name(arg: &Datum) -> bool {
            DatumType::get_type(arg) == DatumType::$type_name
        }

        pub static $static_name: R1<Datum, bool> = R1 { r1: $func_name };
    )
}

impl_typecheck!(PRIM_BOOLEAN, is_boolean, Bool);
impl_typecheck!(PRIM_PAIR, is_pair, Pair);
impl_typecheck!(PRIM_SYMBOL, is_symbol, Sym);
impl_typecheck!(PRIM_NUMBER, is_number, Num);
impl_typecheck!(PRIM_CHAR, is_char, Char);
impl_typecheck!(PRIM_STRING, is_string, String);
impl_typecheck!(PRIM_VECTOR, is_vector, Vector);
impl_typecheck!(PRIM_PROCEDURE, is_procedure, Callable);
impl_typecheck!(PRIM_NULL, is_null, Null);

/// Lists all primitive functions with its name
pub fn libprimitive() -> Vec<(&'static str, &'static (PrimFunc + 'static))> {
    vec![
        ("+", &PRIM_ADD),
        ("-", &PRIM_SUB),
        ("*", &PRIM_MUL),
        ("/", &PRIM_DIV),
        ("list", &PRIM_LIST),
        ("boolean?", &PRIM_BOOLEAN),
        ("pair?", &PRIM_PAIR),
        ("symbol?", &PRIM_SYMBOL),
        ("number?", &PRIM_NUMBER),
        ("char?", &PRIM_CHAR),
        ("string?", &PRIM_STRING),
        ("vector?", &PRIM_VECTOR),
        ("procedure?", &PRIM_PROCEDURE),
        ("null?", &PRIM_NULL),
        ("car", &PRIM_CAR),
        ("cdr", &PRIM_CDR),
        ("zero?", &PRIM_ZERO),
        // complex? is synonym to number?
        ("complex?", &PRIM_NUMBER),
        ("real?", &PRIM_REAL),
        ("rational?", &PRIM_RATIONAL),
        ("integer?", &PRIM_INTEGER),
        ("=", &PRIM_NUM_EQ),
        ("<", &PRIM_LT),
        (">", &PRIM_GT),
        ("<=", &PRIM_LE),
        (">=", &PRIM_GE)
    ]
}
