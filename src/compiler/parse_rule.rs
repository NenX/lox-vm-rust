use super::{Compiler, Precedence};

#[macro_export]
macro_rules! ph {
    () => {
        Option<fn(& mut Compiler)>
    };
}
pub struct ParseRule {
    pub prefix: ph!(),
    pub infix: ph!(),
    pub prec: Precedence,
}
