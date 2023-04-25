use crate::errorkey::ErrorKey;
use crate::errors::error;
use crate::everything::Everything;
use crate::item::Item;
use crate::tables::datafunctions::Args;
use crate::token::Token;

pub use crate::tables::datafunctions::{
    lookup_function, lookup_global_function, lookup_global_promote, lookup_promote, Datatype,
    LookupResult,
};

#[derive(Clone, Debug)]
pub struct CodeChain {
    // "codes" is my name for the things separated by dots in gui functions.
    // They should be a series of "promotes" followed by a final "function",
    // each of which can possibly take arguments.
    pub codes: Vec<Code>,
}

// Most "codes" are just a name followed by another dot or by the end of the code section.
// Some have arguments between parentheses, which can be single-quoted strings, or other code chains.
#[derive(Clone, Debug)]
pub struct Code {
    pub name: Token,
    pub arguments: Vec<CodeArg>,
}

// Possibly the literal arguments can themselves contain [ ] code blocks.
// I'll have to test that.
// A literal argument can be a string that starts with a (datatype) in front
// of it, such as '(int32)0'.
#[derive(Clone, Debug)]
pub enum CodeArg {
    Chain(CodeChain),
    Literal(Token),
}

impl CodeChain {
    pub fn as_gameconcept(&self) -> Option<&Token> {
        if self.codes.len() == 1 && self.codes[0].arguments.is_empty() {
            Some(&self.codes[0].name)
        } else {
            None
        }
    }
}

fn validate_argument(arg: &CodeArg, _data: &Everything, expect_type: Datatype) {
    match arg {
        CodeArg::Chain(chain) => validate_datatypes(chain, _data, expect_type),
        CodeArg::Literal(token) => {
            if token.as_str().starts_with('(') {
                // TODO: parse datatype from string
            } else {
                if expect_type != Datatype::Unknown && expect_type != Datatype::CString {
                    error(
                        token,
                        ErrorKey::DataFunctions,
                        &format!("expected {}, got CString", expect_type),
                    );
                }
            }
        }
    }
}

pub fn validate_datatypes(chain: &CodeChain, data: &Everything, expect_type: Datatype) {
    let mut curtype = Datatype::Unknown;
    for (i, code) in chain.codes.iter().enumerate() {
        let is_first = i == 0;
        let is_last = i == chain.codes.len() - 1;
        let mut args = Args::NoArgs;
        let mut rtype = Datatype::Unknown;

        // The data_type logs include all game concepts as global functions.
        // We don't want them to match here, because those concepts often
        // overlap with passed-in scopes, which are not functions.
        let lookup_gf = if data.item_exists(Item::GameConcept, code.name.as_str()) {
            None
        } else {
            lookup_global_function(&code.name)
        };
        let lookup_gp = lookup_global_promote(&code.name);
        let lookup_f = lookup_function(&code.name, curtype);
        let lookup_p = lookup_promote(&code.name, curtype);

        let gf_found = lookup_gf.is_some();
        let gp_found = lookup_gp.is_some();
        let f_found = !matches!(lookup_f, LookupResult::NotFound);
        let p_found = !matches!(lookup_p, LookupResult::NotFound);

        let mut found = false;

        if is_first && is_last {
            if let Some((xargs, xrtype)) = lookup_gf {
                found = true;
                args = xargs;
                rtype = xrtype;
            }
        } else if is_first && !is_last {
            if let Some((xargs, xrtype)) = lookup_gp {
                found = true;
                args = xargs;
                rtype = xrtype;
            }
        } else if !is_first && !is_last {
            match lookup_p {
                LookupResult::Found(xargs, xrtype) => {
                    found = true;
                    args = xargs;
                    rtype = xrtype;
                }
                LookupResult::WrongType => {
                    error(
                        &code.name,
                        ErrorKey::DataFunctions,
                        &format!("{} can not follow a {} promote", code.name, curtype),
                    );
                    return;
                }
                LookupResult::NotFound => (),
            }
        } else if !is_first && is_last {
            match lookup_f {
                LookupResult::Found(xargs, xrtype) => {
                    found = true;
                    args = xargs;
                    rtype = xrtype;
                }
                LookupResult::WrongType => {
                    error(
                        &code.name,
                        ErrorKey::DataFunctions,
                        &format!("{} can not follow a {} promote", code.name, curtype),
                    );
                    return;
                }
                LookupResult::NotFound => (),
            }
        }

        if !found {
            // Properly reporting these errors is tricky because `code.name`
            // might be found in any or all of the functions and promotes tables.
            if is_first && (p_found || f_found) && !gp_found && !gf_found {
                error(
                    &code.name,
                    ErrorKey::DataFunctions,
                    &format!("{} can not be the first in a chain", code.name),
                );
                return;
            }
            if is_last && (gp_found || p_found) && !gf_found && !f_found {
                error(
                    &code.name,
                    ErrorKey::DataFunctions,
                    &format!("{} can not be last in a chain", code.name),
                );
                return;
            }
            if !is_first && (gp_found || gf_found) && !p_found && !f_found {
                error(
                    &code.name,
                    ErrorKey::DataFunctions,
                    &format!("{} must be the first in a chain", code.name),
                );
                return;
            }
            if !is_last && (gf_found || f_found) && !gp_found && !p_found {
                error(
                    &code.name,
                    ErrorKey::DataFunctions,
                    &format!("{} must be last in the chain", code.name),
                );
                return;
            }
            // A catch-all condition if none of the above match
            if gp_found || gf_found || p_found || f_found {
                error(
                    &code.name,
                    ErrorKey::DataFunctions,
                    &format!("{} is improperly used here", code.name),
                );
                return;
            } else {
                // If `code.name` is not found at all in the tables, then
                // it can be some passed-in scope. Unfortunately we don't
                // have a complete list of those, so accept them all.
                args = Args::NoArgs;
                // TODO: this could in theory be reduced to just the scope types
                rtype = Datatype::Unknown;
            }
        }

        if args.nargs() != code.arguments.len() {
            error(
                &code.name,
                ErrorKey::DataFunctions,
                &format!(
                    "{} takes {} arguments but was given {} here",
                    code.name,
                    args.nargs(),
                    code.arguments.len()
                ),
            );
        } else {
            match args {
                Args::NoArgs => (),
                Args::Arg(dt1) => validate_argument(&code.arguments[0], data, dt1),
                Args::Arg2(dt1, dt2) => {
                    validate_argument(&code.arguments[0], data, dt1);
                    validate_argument(&code.arguments[1], data, dt2);
                }
                Args::Arg3(dt1, dt2, dt3) => {
                    validate_argument(&code.arguments[0], data, dt1);
                    validate_argument(&code.arguments[1], data, dt2);
                    validate_argument(&code.arguments[2], data, dt3);
                }
                Args::Arg4(dt1, dt2, dt3, dt4) => {
                    validate_argument(&code.arguments[0], data, dt1);
                    validate_argument(&code.arguments[1], data, dt2);
                    validate_argument(&code.arguments[2], data, dt3);
                    validate_argument(&code.arguments[3], data, dt4);
                }
            }
        }

        curtype = rtype;

        if is_last
            && curtype != Datatype::Unknown
            && expect_type != Datatype::Unknown
            && curtype != expect_type
        {
            error(
                &code.name,
                ErrorKey::DataFunctions,
                &format!(
                    "{} returns {} but a {} is needed here",
                    code.name, curtype, expect_type
                ),
            );
            return;
        }
    }
}
