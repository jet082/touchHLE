//! `UIGeometry.h`
//!
//! See also [crate::frameworks::core_graphics::cg_geometry].

use crate::dyld::{export_c_func, FunctionExports};
use crate::frameworks::core_graphics::{CGFloat, CGPoint, CGRect, CGSize};
use crate::frameworks::foundation::ns_string;
use crate::objc::{autorelease, id};
use crate::Environment;

// Apple's documentation says all of these return zeroes if the input is not
// well-formed.
pub fn CGPointFromString(env: &mut Environment, string: id) -> CGPoint {
    // TODO: avoid copy
    ns_string::to_rust_string(env, string)
        .parse()
        .unwrap_or_default()
}
pub fn CGSizeFromString(env: &mut Environment, string: id) -> CGSize {
    // TODO: avoid copy
    ns_string::to_rust_string(env, string)
        .parse()
        .unwrap_or_default()
}
pub fn CGRectFromString(env: &mut Environment, string: id) -> CGRect {
    // TODO: avoid copy
    ns_string::to_rust_string(env, string)
        .parse()
        .unwrap_or_default()
}

pub fn NSStringFromCGPoint(env: &mut Environment, point: CGPoint) -> id {
    let s = ns_string::from_rust_string(env, point.to_string());
    autorelease(env, s)
}
pub fn NSStringFromCGSize(env: &mut Environment, size: CGSize) -> id {
    let s = ns_string::from_rust_string(env, size.to_string());
    autorelease(env, s)
}
pub fn NSStringFromCGRect(env: &mut Environment, rect: CGRect) -> id {
    let s = ns_string::from_rust_string(env, rect.to_string());
    autorelease(env, s)
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct UIEdgeInsets {
    pub top: CGFloat,
    pub left: CGFloat,
    pub bottom: CGFloat,
    pub right: CGFloat,
}

impl std::fmt::Display for UIEdgeInsets {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{{}, {}, {}, {}}}",
            self.top, self.left, self.bottom, self.right
        )
    }
}

pub fn UIEdgeInsetsFromString(env: &mut Environment, string: id) -> UIEdgeInsets {
    // TODO: implement properly
    log!("TODO: UIEdgeInsetsFromString({:?})", string);
    UIEdgeInsets::default()
}

pub fn NSStringFromUIEdgeInsets(env: &mut Environment, insets: UIEdgeInsets) -> id {
    let s = ns_string::from_rust_string(env, insets.to_string());
    autorelease(env, s)
}

pub const FUNCTIONS: FunctionExports = &[
    export_c_func!(CGPointFromString(_)),
    export_c_func!(CGSizeFromString(_)),
    export_c_func!(CGRectFromString(_)),
    export_c_func!(NSStringFromCGPoint(_)),
    export_c_func!(NSStringFromCGSize(_)),
    export_c_func!(NSStringFromCGRect(_)),
    export_c_func!(UIEdgeInsetsFromString(_)),
    export_c_func!(NSStringFromUIEdgeInsets(_)),
];
