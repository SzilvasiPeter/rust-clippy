use clippy_utils::diagnostics::span_lint;
use clippy_utils::return_ty;
use rustc_hir::{ImplItem, Mutability, OwnerId, TraitItem};
use rustc_lint::LateContext;
use rustc_middle::ty::{self, Ty};
use rustc_span::Ident;

use super::MISLEADING_MUT_METHOD_NAME;

pub(super) fn check_impl_item(cx: &LateContext<'_>, impl_item: &ImplItem<'_>) {
    check_method(cx, impl_item.ident, impl_item.owner_id);
}

pub(super) fn check_trait_item(cx: &LateContext<'_>, trait_item: &TraitItem<'_>) {
    check_method(cx, trait_item.ident, trait_item.owner_id);
}

fn check_method(cx: &LateContext<'_>, ident: Ident, owner_id: OwnerId) {
    let name = ident.name.as_str();
    if !name.ends_with("_mut") || name.starts_with("from_") {
        return;
    }

    let ret_ty = return_ty(cx, owner_id);
    if ret_ty.is_unit() || contains_mut_ref(cx, ret_ty, 0) {
        return;
    }
    span_lint(
        cx,
        MISLEADING_MUT_METHOD_NAME,
        ident.span,
        "method name suggests that it returns a mutable value, but the return type is not mutable",
    );
}

fn contains_mut_ref<'tcx>(cx: &LateContext<'tcx>, ty: Ty<'tcx>, depth: usize) -> bool {
    if !cx.tcx.recursion_limit().value_within_limit(depth) {
        return true;
    }
    match *ty.kind() {
        ty::Ref(_, inner, Mutability::Not) | ty::Slice(inner) | ty::Array(inner, _) => {
            contains_mut_ref(cx, inner, depth + 1)
        },
        ty::Tuple(args) => args.iter().any(|arg| contains_mut_ref(cx, arg, depth + 1)),
        ty::Adt(adt, args) => {
            args.types().any(|arg| contains_mut_ref(cx, arg, depth + 1))
                || adt
                    .all_fields()
                    .any(|field| contains_mut_ref(cx, field.ty(cx.tcx, args).skip_norm_wip(), depth + 1))
        },
        ty::Bool | ty::Char | ty::Int(_) | ty::Uint(_) | ty::Float(_) | ty::Str | ty::Never | ty::FnPtr(..) => false,
        _ => true,
    }
}
