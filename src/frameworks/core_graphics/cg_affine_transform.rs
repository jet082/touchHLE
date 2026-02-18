/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `CGAffineTransform.h`

use std::ops::{Add, Mul, Sub};

use super::{CGFloat, CGPoint, CGRect, CGSize};
use crate::abi::{impl_GuestRet_for_large_struct, GuestArg};
use crate::dyld::{export_c_func, ConstantExports, FunctionExports, HostConstant};
use crate::matrix::Matrix;
use crate::mem::SafeRead;
use crate::Environment;

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(C, packed)]
/// Apple documents this as representing a 3-by-3 matrix type where the columns
/// are `[a, c, tx]`, `[b, d, ty]`,`[0, 0, 1]`, and which can then be used to
/// transform points with (p × M) where p is a row vector representing the
/// point and M is the matrix.
pub struct CGAffineTransform {
    pub a: CGFloat,
    pub b: CGFloat,
    pub c: CGFloat,
    pub d: CGFloat,
    pub tx: CGFloat,
    pub ty: CGFloat,
}
unsafe impl SafeRead for CGAffineTransform {}

impl std::fmt::Display for CGAffineTransform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let a = self.a;
        let b = self.b;
        let c = self.c;
        let d = self.d;
        let tx = self.tx;
        let ty = self.ty;
        write!(f, "[{}, {}, {}, {}, {}, {}]", a, b, c, d, tx, ty)
    }
}

impl std::str::FromStr for CGAffineTransform {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if !s.starts_with('[') || !s.ends_with(']') {
            return Err(());
        }
        let parts: Vec<&str> = s[1..s.len() - 1].split(',').map(|p| p.trim()).collect();
        if parts.len() != 6 {
            return Err(());
        }
        Ok(CGAffineTransform {
            a: parts[0].parse().map_err(|_| ())?,
            b: parts[1].parse().map_err(|_| ())?,
            c: parts[2].parse().map_err(|_| ())?,
            d: parts[3].parse().map_err(|_| ())?,
            tx: parts[4].parse().map_err(|_| ())?,
            ty: parts[5].parse().map_err(|_| ())?,
        })
    }
}

impl GuestArg for CGAffineTransform {
    const REG_COUNT: usize = 6;

    fn from_regs(regs: &[u32]) -> Self {
        CGAffineTransform {
            a: GuestArg::from_regs(&regs[0..1]),
            b: GuestArg::from_regs(&regs[1..2]),
            c: GuestArg::from_regs(&regs[2..3]),
            d: GuestArg::from_regs(&regs[3..4]),
            tx: GuestArg::from_regs(&regs[4..5]),
            ty: GuestArg::from_regs(&regs[5..6]),
        }
    }
    fn to_regs(self, regs: &mut [u32]) {
        self.a.to_regs(&mut regs[0..1]);
        self.b.to_regs(&mut regs[1..2]);
        self.c.to_regs(&mut regs[2..3]);
        self.d.to_regs(&mut regs[3..4]);
        self.tx.to_regs(&mut regs[4..5]);
        self.ty.to_regs(&mut regs[5..6]);
    }
}
impl_GuestRet_for_large_struct!(CGAffineTransform);

// These conversions allow sharing code with the touchHLE Matrix type.
// Note that they transpose the matrix relative to what Apple documents (see
// the doc comment on the struct above), because our Matrix type is built on
// OpenGL-style (M × v) column-vector multiplication for transformations, vs.
// CGAffineTransform's (v × M) row-vector multiplication.
impl TryFrom<Matrix<3>> for CGAffineTransform {
    type Error = ();

    fn try_from(value: Matrix<3>) -> Result<CGAffineTransform, ()> {
        let columns = value.columns();
        if columns[0][2] == 0.0 && columns[1][2] == 0.0 && columns[2][2] == 1.0 {
            Ok(affine_transform_from_matrix_unchecked(value))
        } else {
            Err(())
        }
    }
}

// Implemented to aid animation code.
// These are the operations needed for the interpolation.
impl Mul<f32> for CGAffineTransform {
    type Output = CGAffineTransform;

    fn mul(self, rhs: f32) -> Self::Output {
        CGAffineTransform {
            a: self.a * rhs,
            b: self.b * rhs,
            c: self.c * rhs,
            d: self.d * rhs,
            tx: self.tx * rhs,
            ty: self.ty * rhs,
        }
    }
}
impl Add<CGAffineTransform> for CGAffineTransform {
    type Output = CGAffineTransform;

    fn add(self, rhs: CGAffineTransform) -> Self::Output {
        CGAffineTransform {
            a: self.a + rhs.a,
            b: self.b + rhs.b,
            c: self.c + rhs.c,
            d: self.d + rhs.d,
            tx: self.tx + rhs.tx,
            ty: self.ty + rhs.ty,
        }
    }
}
impl Sub<CGAffineTransform> for CGAffineTransform {
    type Output = CGAffineTransform;

    fn sub(self, rhs: CGAffineTransform) -> Self::Output {
        CGAffineTransform {
            a: self.a - rhs.a,
            b: self.b - rhs.b,
            c: self.c - rhs.c,
            d: self.d - rhs.d,
            tx: self.tx - rhs.tx,
            ty: self.ty - rhs.ty,
        }
    }
}
fn affine_transform_from_matrix_unchecked(matrix: Matrix<3>) -> CGAffineTransform {
    let columns = matrix.columns();
    CGAffineTransform {
        a: columns[0][0],
        b: columns[0][1],
        c: columns[1][0],
        d: columns[1][1],
        tx: columns[2][0],
        ty: columns[2][1],
    }
}
impl From<CGAffineTransform> for Matrix<3> {
    fn from(value: CGAffineTransform) -> Matrix<3> {
        let CGAffineTransform { a, b, c, d, tx, ty } = value;
        Matrix::<3>::from_columns([[a, b, 0.0], [c, d, 0.0], [tx, ty, 1.0]])
    }
}
impl From<CGAffineTransform> for Matrix<4> {
    fn from(value: CGAffineTransform) -> Matrix<4> {
        let CGAffineTransform { a, b, c, d, tx, ty } = value;
        Matrix::<4>::from_columns([
            [a, b, 0.0, 0.0],
            [c, d, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [tx, ty, 0.0, 1.0],
        ])
    }
}

#[rustfmt::skip]
pub const CGAffineTransformIdentity: CGAffineTransform = CGAffineTransform {
    a: 1.0, c: 0.0, tx: 0.0,
    b: 0.0, d: 1.0, ty: 0.0,
    // 0.0, 0.0, 1.0,
};

pub const CONSTANTS: ConstantExports = &[(
    "_CGAffineTransformIdentity",
    HostConstant::Custom(|env| {
        env.mem
            .alloc_and_write(CGAffineTransformIdentity)
            .cast()
            .cast_const()
    }),
)];

// The CGAffineTransform* functions are implemented as wrappers around these
// methods so that host code has the option of calling them without needing to
// provide a &mut Environment and with more convenient syntax. Every method has
// a straightforward mapping to a CGAffineTransform* function.
impl CGAffineTransform {
    pub fn is_identity(self) -> bool {
        self == CGAffineTransformIdentity
    }
    pub fn make_rotation(angle: CGFloat) -> Self {
        if angle.is_nan() {
            return CGAffineTransformIdentity;
        }
        Matrix::<3>::from(&Matrix::<2>::z_rotation(angle))
            .try_into()
            .unwrap_or(CGAffineTransformIdentity)
    }
    pub fn make_scale(x: CGFloat, y: CGFloat) -> Self {
        if x.is_nan() || y.is_nan() {
            return CGAffineTransformIdentity;
        }
        Matrix::<3>::from(&Matrix::<2>::scale_2d(x, y))
            .try_into()
            .unwrap_or(CGAffineTransformIdentity)
    }
    pub fn make_translation(x: CGFloat, y: CGFloat) -> Self {
        if x.is_nan() || y.is_nan() {
            return CGAffineTransformIdentity;
        }
        Matrix::<3>::translate_2d(x, y)
            .try_into()
            .unwrap_or(CGAffineTransformIdentity)
    }
    pub fn concat(self, other: Self) -> Self {
        if self.a.is_nan() || self.b.is_nan() || self.c.is_nan() || self.d.is_nan() || self.tx.is_nan() || self.ty.is_nan() ||
           other.a.is_nan() || other.b.is_nan() || other.c.is_nan() || other.d.is_nan() || other.tx.is_nan() || other.ty.is_nan() {
            return CGAffineTransformIdentity;
        }
        Matrix::<3>::multiply(&self.into(), &other.into())
            .try_into()
            .unwrap_or(CGAffineTransformIdentity)
    }
    pub fn rotate(self, angle: CGFloat) -> Self {
        Self::make_rotation(angle).concat(self)
    }
    pub fn scale(self, x: CGFloat, y: CGFloat) -> Self {
        Self::make_scale(x, y).concat(self)
    }
    pub fn translate(self, x: CGFloat, y: CGFloat) -> Self {
        Self::make_translation(x, y).concat(self)
    }
    pub fn invert(self) -> Self {
        if self.a.is_nan() || self.b.is_nan() || self.c.is_nan() || self.d.is_nan() || self.tx.is_nan() || self.ty.is_nan() {
            return CGAffineTransformIdentity;
        }
        let self_3x3: Matrix<3> = self.into();
        if let Some(inverse) = Matrix::<3>::from(&self_3x3).inverse() {
            // Matrix inversion sometimes produces values in the last column
            // that are slightly off due to accumulated floating-point error.
            // The TryFrom check causes crashes in that case, and it is a waste
            // of energy to begin with as the result of inverting an affine
            // transformation matrix will also be affine.
            let res = affine_transform_from_matrix_unchecked(inverse);
            if res.a.is_nan() || res.b.is_nan() || res.c.is_nan() || res.d.is_nan() || res.tx.is_nan() || res.ty.is_nan() {
                CGAffineTransformIdentity
            } else {
                res
            }
        } else {
            self
        }
    }

    pub fn apply_to_point(self, point: CGPoint) -> CGPoint {
        // z = 1 makes the translation (in homogenous co-ordinates) be applied
        let [x, y, _] = Matrix::<3>::transform(&self.into(), [point.x, point.y, 1.0]);
        CGPoint { x, y }
    }
    pub fn apply_to_size(self, size: CGSize) -> CGSize {
        // z = 0 makes the translation (in homogenous co-ordinates) be ignored
        let [width, height, _] =
            Matrix::<3>::transform(&self.into(), [size.width, size.height, 0.0]);
        CGSize { width, height }
    }
    pub fn apply_to_rect(self, rect: CGRect) -> CGRect {
        // Affine transforms applied to a rectangle don't necessarily return a
        // rectangle (just a quadrilateral), so CGRectApplyAffineTransform
        // essentially returns the bounding box of the points.

        let corner1 = rect.origin;
        let corner2 = CGPoint {
            x: rect.origin.x + rect.size.width,
            y: rect.origin.y,
        };
        let corner3 = CGPoint {
            x: rect.origin.x,
            y: rect.origin.y + rect.size.height,
        };
        let corner4 = CGPoint {
            x: rect.origin.x + rect.size.width,
            y: rect.origin.y + rect.size.height,
        };

        let point1 = self.apply_to_point(corner1);
        let point2 = self.apply_to_point(corner2);
        let point3 = self.apply_to_point(corner3);
        let point4 = self.apply_to_point(corner4);

        let x1 = point1.x.min(point2.x).min(point3.x).min(point4.x);
        let x2 = point1.x.max(point2.x).max(point3.x).max(point4.x);
        let y1 = point1.y.min(point2.y).min(point3.y).min(point4.y);
        let y2 = point1.y.max(point2.y).max(point3.y).max(point4.y);

        CGRect {
            origin: CGPoint { x: x1, y: y1 },
            size: CGSize {
                width: x2 - x1,
                height: y2 - y1,
            },
        }
    }
}

fn CGAffineTransformIsIdentity(_env: &mut Environment, transform: CGAffineTransform) -> bool {
    transform.is_identity()
}

fn CGAffineTransformEqualToTransform(
    _env: &mut Environment,
    a: CGAffineTransform,
    b: CGAffineTransform,
) -> bool {
    a == b
}

fn CGAffineTransformMake(
    _env: &mut Environment,
    a: CGFloat,
    b: CGFloat,
    c: CGFloat,
    d: CGFloat,
    tx: CGFloat,
    ty: CGFloat,
) -> CGAffineTransform {
    CGAffineTransform { a, b, c, d, tx, ty }
}

fn CGAffineTransformMakeRotation(_env: &mut Environment, angle: CGFloat) -> CGAffineTransform {
    CGAffineTransform::make_rotation(angle)
}
fn CGAffineTransformMakeScale(_env: &mut Environment, x: CGFloat, y: CGFloat) -> CGAffineTransform {
    CGAffineTransform::make_scale(x, y)
}
fn CGAffineTransformMakeTranslation(
    _env: &mut Environment,
    x: CGFloat,
    y: CGFloat,
) -> CGAffineTransform {
    CGAffineTransform::make_translation(x, y)
}

fn CGAffineTransformConcat(
    _env: &mut Environment,
    a: CGAffineTransform,
    b: CGAffineTransform,
) -> CGAffineTransform {
    a.concat(b)
}

pub fn CGAffineTransformRotate(
    _env: &mut Environment,
    existing: CGAffineTransform,
    angle: CGFloat,
) -> CGAffineTransform {
    existing.rotate(angle)
}
pub fn CGAffineTransformScale(
    _env: &mut Environment,
    existing: CGAffineTransform,
    x: CGFloat,
    y: CGFloat,
) -> CGAffineTransform {
    existing.scale(x, y)
}
pub fn CGAffineTransformTranslate(
    _env: &mut Environment,
    existing: CGAffineTransform,
    x: CGFloat,
    y: CGFloat,
) -> CGAffineTransform {
    existing.translate(x, y)
}
pub fn CGAffineTransformInvert(
    _env: &mut Environment,
    existing: CGAffineTransform,
) -> CGAffineTransform {
    existing.invert()
}

fn CGPointApplyAffineTransform(
    _env: &mut Environment,
    point: CGPoint,
    transform: CGAffineTransform,
) -> CGPoint {
    transform.apply_to_point(point)
}
fn CGSizeApplyAffineTransform(
    _env: &mut Environment,
    size: CGSize,
    transform: CGAffineTransform,
) -> CGSize {
    transform.apply_to_size(size)
}
pub fn CGRectApplyAffineTransform(
    _env: &mut Environment,
    rect: CGRect,
    transform: CGAffineTransform,
) -> CGRect {
    transform.apply_to_rect(rect)
}

pub const FUNCTIONS: FunctionExports = &[
    export_c_func!(CGAffineTransformIsIdentity(_)),
    export_c_func!(CGAffineTransformEqualToTransform(_, _)),
    export_c_func!(CGAffineTransformMake(_, _, _, _, _, _)),
    export_c_func!(CGAffineTransformMakeRotation(_)),
    export_c_func!(CGAffineTransformMakeScale(_, _)),
    export_c_func!(CGAffineTransformMakeTranslation(_, _)),
    export_c_func!(CGAffineTransformConcat(_, _)),
    export_c_func!(CGAffineTransformRotate(_, _)),
    export_c_func!(CGAffineTransformScale(_, _, _)),
    export_c_func!(CGAffineTransformTranslate(_, _, _)),
    export_c_func!(CGAffineTransformInvert(_)),
    export_c_func!(CGPointApplyAffineTransform(_, _)),
    export_c_func!(CGSizeApplyAffineTransform(_, _)),
    export_c_func!(CGRectApplyAffineTransform(_, _)),
];
