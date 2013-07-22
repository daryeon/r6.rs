use numeric::LNumeric;
use std::io;
use std::vec;

#[deriving(Eq)]
pub enum LDatum<T> {
    LIdent(@str),
    LString(~str),
    LChar(char),
    LBool(bool),
    LNum(LNumeric),
    LCons(@LDatum<T>, @LDatum<T>),
    LNil,
    LVector(~[@LDatum<T>]),
    LExt(T),
}

#[deriving(Eq)]
pub enum Quotation
{
    Quote,
    QuasiQuote,
    Unquote,
}

impl<T: ToStr> ToStr for LDatum<T> {
    fn to_str(&self) -> ~str {
        do io::with_str_writer |wr| {
            write_ldatum(wr, self)
        }
    }
}

impl<T> LDatum<T> {
    pub fn to_list(&self) -> Option<~[@LDatum<T>]> {
        match *self {
            LCons(h, t) => datum_to_list(h, t),
            LNil => Some(~[]),
            _ => None,
        }
    }
}

impl<T: ToStr> LDatum<T> {
    pub fn write(&self, wr: @io::Writer) {
        write_ldatum(wr, self)
    }
}

priv fn datum_to_list<T>(head: @LDatum<T>, tail: @LDatum<T>) -> Option<~[@LDatum<T>]> {
    let mut x: @LDatum<T> = tail;
    let mut list_flag = false;

    let list =
    do vec::build |push| {
        push(head);
        loop {
            match *x {
                LCons(h, t) => {
                    push(h);
                    x = t;
                },
                LNil => break,
                _ => {
                    list_flag = true;
                    break
                },
            }
        }
    };

    if list_flag {
        None
    } else {
        Some(list)
    }
}

priv fn write_ldatum<T: ToStr>(wr: @io::Writer, &v: &LDatum<T>) {
    match v {
        LIdent(s) => wr.write_str(s),
        LString(s) => {
            wr.write_char('"');
            wr.write_str(s);
            wr.write_char('"');
        },
        LChar(' ') => wr.write_str("#\\space"),
        LChar('\n') => wr.write_str("#\\newline"),
        LChar(c) => {
            wr.write_str("#\\");
            wr.write_char(c);
        },
        LBool(true) => wr.write_str("#t"),
        LBool(false) => wr.write_str("#f"),
        LNum(f) => wr.write_str(f.to_str()),
        LCons(ref head, ref tail) => {
            match is_quote(head, tail) {
                Some((name, v)) => {
                    let ch = match name {
                        Quote => '\'',
                        Unquote => ',',
                        QuasiQuote => '`',
                    };
                    wr.write_char(ch);
                    write_ldatum(wr, v);
                },
                None => {
                    wr.write_char('(');
                    write_ldatum(wr, *head);
                    write_list(wr, *tail);
                },
            }
        },
        LNil => wr.write_str("()"),
        LVector(v) => {
            wr.write_str("#(");
            match v {
                [] => (),
                [head, .. tail] => {
                    write_ldatum(wr, head);
                    for tail.each |&x| {
                        wr.write_char(' ');
                        write_ldatum(wr, x);
                    }
                }
            }
            wr.write_char(')');
        }
        LExt(v) => {
            wr.write_str(v.to_str());
        },
    }
}

pub fn is_quote<T>(&head: &@LDatum<T>, &tail: &@LDatum<T>) -> Option<(Quotation, @LDatum<T>)> {
    match *head {
        LIdent(name) =>
            match *tail {
                LCons(v, @LNil) => if name == @"quote" {
                        Some((Quote, v))
                    } else if name == @"quasiquote" {
                        Some((QuasiQuote, v))
                    } else if name == @"unquote" {
                        Some((Unquote, v))
                    } else {
                        None
                    },
                _ => None
            },
        _ =>
            None,
    }
}

priv fn write_list<T: ToStr>(wr: @io::Writer, &v: &LDatum<T>) {
    match v {
        LCons(head, tail) => {
            wr.write_char(' ');
            write_ldatum(wr, head);
            write_list(wr, tail);
        },
        LNil => wr.write_char(')'),
        _ => {
            wr.write_str(" . ");
            write_ldatum(wr, &v);
            wr.write_char(')');
        },
    }
}
