use clippy_utils::diagnostics::span_lint;
use clippy_utils::return_ty;
use rustc_ast::Mutability;
use rustc_hir::{ImplItem, ImplItemKind, TraitItem, TraitItemKind};
use rustc_lint::LateContext;
use rustc_middle::ty::Ty;
use rustc_span::Ident;

use super::MISLEADING_MUT_METHOD_NAME;

pub(super) fn check_trait_item(cx: &LateContext<'_>, trait_item: &TraitItem<'_>) {
    if let TraitItemKind::Fn(_, _) = trait_item.kind {
        check(cx, trait_item.ident, return_ty(cx, trait_item.owner_id));
    }
}

pub(super) fn check_impl_item(cx: &LateContext<'_>, impl_item: &ImplItem<'_>, implements_trait: bool) {
    if !implements_trait && let ImplItemKind::Fn(_, _) = impl_item.kind {
        check(cx, impl_item.ident, return_ty(cx, impl_item.owner_id));
    }
}

fn check(cx: &LateContext<'_>, ident: Ident, ty: Ty<'_>) {
    let name = ident.name.as_str();
    if (name.starts_with("from_") || name.starts_with("is_"))
        || !(name.contains("_mut_") || name.ends_with("_mut"))
        || !(ty.is_primitive() || ty.ref_mutability() == Some(Mutability::Not))
    {
        return;
    }

    span_lint(
        cx,
        MISLEADING_MUT_METHOD_NAME,
        ident.span,
        "method name suggests that it returns a mutable value, but the return type is not mutable",
    );
}
