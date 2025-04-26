use super::{Compiler, Precedence};

#[macro_export]
macro_rules! parse_f {
    () => {
        Option<fn(& mut Compiler)>
    };
}
pub struct ParseRule {
    pub prefix: parse_f!(),
    pub infix: parse_f!(),
    pub prec: Precedence,
}
