pub(crate) mod apply;
pub(crate) mod assert_or_with;
pub(crate) mod attr_set;
pub(crate) mod bin_op_and_or_default;
pub(crate) mod dynamic;
pub(crate) mod if_else;
pub(crate) mod inherit;
pub(crate) mod key_value;
pub(crate) mod lambda;
pub(crate) mod let_in;
pub(crate) mod list;
pub(crate) mod paren;
pub(crate) mod pat_bind;
pub(crate) mod pat_entry;
pub(crate) mod pattern;
pub(crate) mod root;
pub(crate) mod select;
pub(crate) mod string;
pub(crate) mod string_interpol;

pub(crate) fn default(
    _: &crate::builder::BuildCtx,
    node: &rnix::SyntaxNode,
) -> std::collections::LinkedList<crate::builder::Step> {
    node.children_with_tokens()
        .map(|child| crate::builder::Step::Format(child.into()))
        .collect()
}
