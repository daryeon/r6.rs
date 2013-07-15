#[deriving(Eq)]
pub enum PFunc {
    PEval,
    PBegin,
    PAdd,
    PSub,
    PMul,
    PDiv,
    PCar,
    PCdr,
    PEqv,
    PEqual,
    PNumber,
    PReal,
    PInteger,
    PExact,
    PInexact,
    PEQ,
    PGT,
    PLT,
    PGE,
    PLE,
}

pub fn prelude() -> ~[(@str, PFunc)] {
    do [PEval, PBegin, PAdd, PSub, PMul, PDiv, PCar, PCdr,
        PEqv, PEqual,
        PNumber, PReal, PInteger,
        PExact, PInexact,
        PEQ, PGT, PLT, PGE, PLE].map |prim| {
        (proc_to_str(prim).to_managed(), *prim)
    }
}

pub fn proc_to_str(&prim: &PFunc) -> ~str {
    match prim {
        PEval => ~"eval",
        PBegin => ~"begin",
        PAdd => ~"+",
        PSub => ~"-",
        PMul => ~"*",
        PDiv => ~"/",
        PCar => ~"car",
        PCdr => ~"cdr",
        PEqv => ~"eqv?",
        PEqual => ~"equal?",
        PNumber => ~"number?",
        PReal => ~"real?",
        PInteger => ~"integer?",
        PExact => ~"exact?",
        PInexact => ~"inexact?",
        PEQ => ~"=",
        PGT => ~">",
        PLT => ~"<",
        PGE => ~">=",
        PLE => ~"<=",
    }
}

impl ToStr for PFunc {
    fn to_str(&self) -> ~str {
        proc_to_str(self)
    }
}

#[deriving(Eq)]
pub enum PrimSyntax {
    SynIf,
    SynLambda,
    SynDefine,
    SynSet,
    SynQuote,
    SynQQuote,
    SynUnquote,
}

pub fn syntax_prelude() -> ~[(@str, PrimSyntax)] {
    do [SynIf, SynLambda, SynDefine, SynSet, SynQuote, SynQQuote, SynUnquote].map |syn| {
        (syntax_to_str(syn).to_managed(), *syn)
    }
}

pub fn syntax_to_str(&prim: &PrimSyntax) -> ~str {
    match prim {
        SynIf => ~"if",
        SynLambda => ~"lambda",
        SynDefine => ~"define",
        SynSet => ~"set!",
        SynQuote => ~"quote",
        SynQQuote => ~"quasiquote",
        SynUnquote => ~"unquote",
    }
}

impl ToStr for PrimSyntax {
    fn to_str(&self) -> ~str {
        syntax_to_str(self)
    }
}
