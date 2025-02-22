use oxc_allocator::Vec;
#[allow(clippy::wildcard_imports)]
use oxc_ast::ast::*;

use crate::{doc::Doc, hardline, indent, ss, Prettier};

use super::statement;

pub(super) fn print_block<'a>(
    p: &mut Prettier<'a>,
    stmts: &Vec<'a, Statement<'a>>,
    directives: Option<&Vec<'a, Directive>>,
    is_static_block: bool,
) -> Doc<'a> {
    let mut parts = p.vec();
    if is_static_block {
        parts.push(ss!("static "));
    }
    parts.push(ss!("{"));
    if let Some(doc) = print_block_body(p, stmts, directives, true, false) {
        parts.push(indent![p, hardline!(), doc]);
        parts.push(hardline!());
    }
    parts.push(ss!("}"));
    Doc::Array(parts)
}

pub(super) fn print_block_body<'a>(
    p: &mut Prettier<'a>,
    stmts: &Vec<'a, Statement<'a>>,
    directives: Option<&Vec<'a, Directive>>,
    remove_last_statement_hardline: bool,
    is_program: bool,
) -> Option<Doc<'a>> {
    let has_directives = directives.is_some_and(|directives| !directives.is_empty());
    let has_body = stmts.iter().any(|stmt| !matches!(stmt, Statement::EmptyStatement(_)));

    if !has_body && !has_directives {
        return None;
    }

    let mut parts = p.vec();

    if has_directives {
        if let Some(directives) = directives {
            parts.extend(statement::print_statement_sequence(p, directives, false));
        }
    }

    if !stmts.is_empty() {
        parts.extend(statement::print_statement_sequence(p, stmts, remove_last_statement_hardline));
    }

    if is_program {
        parts.push(hardline!());
    }

    Some(Doc::Array(parts))
}
