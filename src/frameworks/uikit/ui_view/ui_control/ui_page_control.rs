/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `UIPageControl`.

use crate::frameworks::core_graphics::CGSize;
use crate::frameworks::foundation::NSInteger;
use crate::objc::{
    id, impl_HostObject_with_superclass, msg, nil, objc_classes, todo_objc_setter, ClassExports,
    NSZonePtr,
};

#[derive(Default)]
struct UIPageControlHostObject {
    superclass: super::UIControlHostObject,
    number_of_pages: NSInteger,
    current_page: NSInteger,
    hides_for_single_page: bool,
    defers_current_page_display: bool,
}
impl_HostObject_with_superclass!(UIPageControlHostObject);

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation UIPageControl: UIControl

+ (id)allocWithZone:(NSZonePtr)_zone {
    let host_object = Box::<UIPageControlHostObject>::default();
    env.objc.alloc_object(this, host_object, &mut env.mem)
}

- (NSInteger)numberOfPages {
    env.objc.borrow::<UIPageControlHostObject>(this).number_of_pages
}
- (())setNumberOfPages:(NSInteger)number {
    env.objc.borrow_mut::<UIPageControlHostObject>(this).number_of_pages = number;
    () = msg![env; this setNeedsDisplay];
}

- (NSInteger)currentPage {
    env.objc.borrow::<UIPageControlHostObject>(this).current_page
}
- (())setCurrentPage:(NSInteger)page {
    env.objc.borrow_mut::<UIPageControlHostObject>(this).current_page = page;
    () = msg![env; this setNeedsDisplay];
}

- (bool)hidesForSinglePage {
    env.objc.borrow::<UIPageControlHostObject>(this).hides_for_single_page
}
- (())setHidesForSinglePage:(bool)hides {
    env.objc.borrow_mut::<UIPageControlHostObject>(this).hides_for_single_page = hides;
    () = msg![env; this setNeedsDisplay];
}

- (bool)defersCurrentPageDisplay {
    env.objc.borrow::<UIPageControlHostObject>(this).defers_current_page_display
}
- (())setDefersCurrentPageDisplay:(bool)defers {
    env.objc.borrow_mut::<UIPageControlHostObject>(this).defers_current_page_display = defers;
}

- (())updateCurrentPageDisplay {
    () = msg![env; this setNeedsDisplay];
}

- (CGSize)sizeForNumberOfPages:(NSInteger)page_count {
    // Basic estimation
    CGSize {
        width: page_count as f32 * 20.0,
        height: 20.0,
    }
}

- (id)pageIndicatorTintColor {
    nil
}
- (())setPageIndicatorTintColor:(id)color {
    todo_objc_setter!(this, color);
}

- (id)currentPageIndicatorTintColor {
    nil
}
- (())setCurrentPageIndicatorTintColor:(id)color {
    todo_objc_setter!(this, color);
}

@end

};
