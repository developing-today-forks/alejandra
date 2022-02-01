pub fn rule(
    build_ctx: &crate::builder::BuildCtx,
    node: &rnix::SyntaxNode,
) -> std::collections::LinkedList<crate::builder::Step> {
    let mut steps = std::collections::LinkedList::new();

    let mut children = crate::children::Children::new(build_ctx, node);

    let layout = if children.has_comments() {
        &crate::config::Layout::Tall
    } else {
        build_ctx.config.layout()
    };

    // a
    let child = children.get_next().unwrap();
    let is_pattern_type =
        child.element.kind() == rnix::SyntaxKind::NODE_PATTERN;
    match layout {
        crate::config::Layout::Tall => {
            steps.push_back(crate::builder::Step::FormatWider(child.element));
        }
        crate::config::Layout::Wide => {
            steps.push_back(crate::builder::Step::Format(child.element));
        }
    }

    if let rnix::SyntaxKind::TOKEN_COMMENT =
        children.peek_next().unwrap().element.kind()
    {
        steps.push_back(crate::builder::Step::NewLine);
        steps.push_back(crate::builder::Step::Pad);
    }

    // /**/
    children.drain_comments(|text| {
        steps.push_back(crate::builder::Step::Comment(text));
        steps.push_back(crate::builder::Step::NewLine);
        steps.push_back(crate::builder::Step::Pad);
    });

    // :
    let child = children.get_next().unwrap();
    match layout {
        crate::config::Layout::Tall => {
            steps.push_back(crate::builder::Step::FormatWider(child.element));
        }
        crate::config::Layout::Wide => {
            steps.push_back(crate::builder::Step::Format(child.element));
        }
    }

    // /**/
    children.drain_comments(|text| {
        steps.push_back(crate::builder::Step::NewLine);
        steps.push_back(crate::builder::Step::Pad);
        steps.push_back(crate::builder::Step::Comment(text));
    });

    // c
    let child_prev = children.peek_prev().unwrap();
    let child = children.get_next().unwrap();
    match layout {
        crate::config::Layout::Tall => {
            if is_pattern_type {
                steps.push_back(crate::builder::Step::NewLine);
                steps.push_back(crate::builder::Step::Pad);
            } else if let rnix::SyntaxKind::TOKEN_COMMENT =
                child_prev.element.kind()
            {
                steps.push_back(crate::builder::Step::NewLine);
                steps.push_back(crate::builder::Step::Pad);
            } else if child.element.kind() == rnix::SyntaxKind::NODE_LAMBDA
                && child
                    .element
                    .clone()
                    .into_node()
                    .unwrap()
                    .children_with_tokens()
                    .next()
                    .unwrap()
                    .kind()
                    == rnix::SyntaxKind::NODE_IDENT
            {
                steps.push_back(crate::builder::Step::Whitespace);
            } else {
                steps.push_back(crate::builder::Step::NewLine);
                steps.push_back(crate::builder::Step::Pad);
            }
            steps.push_back(crate::builder::Step::FormatWider(child.element));
        }
        crate::config::Layout::Wide => {
            steps.push_back(crate::builder::Step::Whitespace);
            steps.push_back(crate::builder::Step::Format(child.element));
        }
    }

    steps
}
