/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `UIScrollView`.

pub mod ui_text_view;
use crate::frameworks::core_graphics::{CGFloat, CGPoint, CGRect, CGSize};
use crate::frameworks::foundation::NSInteger;
use crate::frameworks::uikit::ui_geometry::UIEdgeInsets;
use crate::objc::{
    id, impl_HostObject_with_superclass, msg, nil, objc_classes, todo_objc_setter, ClassExports,
    NSZonePtr, SEL,
};

type UIScrollViewIndicatorStyle = NSInteger;

pub struct UIScrollViewHostObject {
    superclass: super::UIViewHostObject,
    /// UIScrollViewDelegate, weak reference
    delegate: id,
    scroll_enabled: bool,
    content_offset: CGPoint,
    content_size: CGSize,
    content_inset: UIEdgeInsets,
    scroll_indicator_insets: UIEdgeInsets,
    bounces: bool,
    directional_lock_enabled: bool,
    paging_enabled: bool,
    shows_horizontal_scroll_indicator: bool,
    shows_vertical_scroll_indicator: bool,
    scrolls_to_top: bool,
}
impl_HostObject_with_superclass!(UIScrollViewHostObject);
impl Default for UIScrollViewHostObject {
    fn default() -> Self {
        UIScrollViewHostObject {
            superclass: Default::default(),
            delegate: nil,
            scroll_enabled: true,
            content_offset: CGPoint { x: 0.0, y: 0.0 },
            content_size: CGSize {
                width: 0.0,
                height: 0.0,
            },
            content_inset: UIEdgeInsets::default(),
            scroll_indicator_insets: UIEdgeInsets::default(),
            bounces: true,
            directional_lock_enabled: false,
            paging_enabled: false,
            shows_horizontal_scroll_indicator: true,
            shows_vertical_scroll_indicator: true,
            scrolls_to_top: true,
        }
    }
}

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation UIScrollView: UIView

+ (id)allocWithZone:(NSZonePtr)_zone {
    let host_object = Box::<UIScrollViewHostObject>::default();
    env.objc.alloc_object(this, host_object, &mut env.mem)
}

- (id)initWithCoder:(id)coder {
    this = msg_super![env; this initWithCoder:coder];

    let key_ns_string = get_static_str(env, "UIContentSize");
    if msg![env; coder containsValueForKey:key_ns_string] {
        let size: CGSize = msg![env; coder decodeCGSizeForKey:key_ns_string];
        () = msg![env; this setContentSize:size];
    }

    let key_ns_string = get_static_str(env, "UIContentOffset");
    if msg![env; coder containsValueForKey:key_ns_string] {
        let offset: CGPoint = msg![env; coder decodeCGPointForKey:key_ns_string];
        () = msg![env; this setContentOffset:offset];
    }

    let key_ns_string = get_static_str(env, "UIContentInset");
    if msg![env; coder containsValueForKey:key_ns_string] {
        let inset: UIEdgeInsets = msg![env; coder decodeUIEdgeInsetsForKey:key_ns_string];
        () = msg![env; this setContentInset:inset];
    }

    let key_ns_string = get_static_str(env, "UIBounces");
    if msg![env; coder containsValueForKey:key_ns_string] {
        let bounces: bool = msg![env; coder decodeBoolForKey:key_ns_string];
        () = msg![env; this setBounces:bounces];
    }

    let key_ns_string = get_static_str(env, "UIScrollEnabled");
    if msg![env; coder containsValueForKey:key_ns_string] {
        let enabled: bool = msg![env; coder decodeBoolForKey:key_ns_string];
        () = msg![env; this setScrollEnabled:enabled];
    }

    let key_ns_string = get_static_str(env, "UIPagingEnabled");
    if msg![env; coder containsValueForKey:key_ns_string] {
        let enabled: bool = msg![env; coder decodeBoolForKey:key_ns_string];
        () = msg![env; this setPagingEnabled:enabled];
    }

    let key_ns_string = get_static_str(env, "UIShowsHorizontalScrollIndicator");
    if msg![env; coder containsValueForKey:key_ns_string] {
        let shows: bool = msg![env; coder decodeBoolForKey:key_ns_string];
        () = msg![env; this setShowsHorizontalScrollIndicator:shows];
    }

    let key_ns_string = get_static_str(env, "UIShowsVerticalScrollIndicator");
    if msg![env; coder containsValueForKey:key_ns_string] {
        let shows: bool = msg![env; coder decodeBoolForKey:key_ns_string];
        () = msg![env; this setShowsVerticalScrollIndicator:shows];
    }

    this
}

- (id)delegate {
    env.objc.borrow::<UIScrollViewHostObject>(this).delegate
}
- (())setDelegate:(id)delegate {
    env.objc.borrow_mut::<UIScrollViewHostObject>(this).delegate = delegate;
}

- (bool)delaysContentTouches {
    true
}
- (())setDelaysContentTouches:(bool)delays {
    todo_objc_setter!(this, delays);
}

- (bool)bounces {
    env.objc.borrow::<UIScrollViewHostObject>(this).bounces
}
- (())setBounces:(bool)bounces {
    env.objc.borrow_mut::<UIScrollViewHostObject>(this).bounces = bounces;
}

- (bool)scrollEnabled {
    env.objc.borrow::<UIScrollViewHostObject>(this).scroll_enabled
}
- (())setScrollEnabled:(bool)scroll_enabled {
    env.objc.borrow_mut::<UIScrollViewHostObject>(this).scroll_enabled = scroll_enabled;
}

- (CGPoint)contentOffset {
    env.objc.borrow::<UIScrollViewHostObject>(this).content_offset
}
- (())setContentOffset:(CGPoint)offset {
    env.objc.borrow_mut::<UIScrollViewHostObject>(this).content_offset = offset;
    // Bounds origin should be equals to the content offset
    let mut bounds: CGRect = msg![env; this bounds];
    bounds.origin = offset;
    () = msg![env; this setBounds:bounds];
    () = msg![env; this setNeedsDisplay];
}

- (CGSize)contentSize {
    env.objc.borrow::<UIScrollViewHostObject>(this).content_size
}
- (())setContentSize:(CGSize)size {
    env.objc.borrow_mut::<UIScrollViewHostObject>(this).content_size = size;
}

- (UIEdgeInsets)contentInset {
    env.objc.borrow::<UIScrollViewHostObject>(this).content_inset
}
- (())setContentInset:(UIEdgeInsets)inset {
    env.objc.borrow_mut::<UIScrollViewHostObject>(this).content_inset = inset;
}

- (UIEdgeInsets)scrollIndicatorInsets {
    env.objc.borrow::<UIScrollViewHostObject>(this).scroll_indicator_insets
}
- (())setScrollIndicatorInsets:(UIEdgeInsets)inset {
    env.objc.borrow_mut::<UIScrollViewHostObject>(this).scroll_indicator_insets = inset;
}

- (())setIndicatorStyle:(UIScrollViewIndicatorStyle)style {
    todo_objc_setter!(this, style);
}

- (bool)isDirectionalLockEnabled {
    env.objc.borrow::<UIScrollViewHostObject>(this).directional_lock_enabled
}
- (())setDirectionalLockEnabled:(bool)enabled {
    env.objc.borrow_mut::<UIScrollViewHostObject>(this).directional_lock_enabled = enabled;
}

- (bool)showsHorizontalScrollIndicator {
    env.objc.borrow::<UIScrollViewHostObject>(this).shows_horizontal_scroll_indicator
}
- (())setShowsHorizontalScrollIndicator:(bool)shows {
    env.objc.borrow_mut::<UIScrollViewHostObject>(this).shows_horizontal_scroll_indicator = shows;
}

- (bool)showsVerticalScrollIndicator {
    env.objc.borrow::<UIScrollViewHostObject>(this).shows_vertical_scroll_indicator
}
- (())setShowsVerticalScrollIndicator:(bool)shows {
    env.objc.borrow_mut::<UIScrollViewHostObject>(this).shows_vertical_scroll_indicator = shows;
}

- (bool)isPagingEnabled {
    env.objc.borrow::<UIScrollViewHostObject>(this).paging_enabled
}
- (())setPagingEnabled:(bool)enabled {
    env.objc.borrow_mut::<UIScrollViewHostObject>(this).paging_enabled = enabled;
}

- (bool)alwaysBounceVertical {
    false
}
- (())setAlwaysBounceVertical:(bool)enabled {
    todo_objc_setter!(this, enabled);
}

- (bool)alwaysBounceHorizontal {
    false
}
- (())setAlwaysBounceHorizontal:(bool)enabled {
    todo_objc_setter!(this, enabled);
}

- (bool)canCancelContentTouches {
    true
}
- (())setCanCancelContentTouches:(bool)enabled {
    todo_objc_setter!(this, enabled);
}

- (bool)scrollsToTop {
    env.objc.borrow::<UIScrollViewHostObject>(this).scrolls_to_top
}
- (())setScrollsToTop:(bool)enabled {
    env.objc.borrow_mut::<UIScrollViewHostObject>(this).scrolls_to_top = enabled;
}

- (bool)bouncesZoom {
    true
}
- (())setBouncesZoom:(bool)enabled {
    todo_objc_setter!(this, enabled);
}

- (CGFloat)minimumZoomScale {
    1.0
}
- (())setMinimumZoomScale:(CGFloat)scale {
    todo_objc_setter!(this, scale);
}

- (CGFloat)maximumZoomScale {
    1.0
}
- (())setMaximumZoomScale:(CGFloat)scale {
    todo_objc_setter!(this, scale);
}

- (CGFloat)zoomScale {
    1.0
}
- (())setZoomScale:(CGFloat)scale {
    todo_objc_setter!(this, scale);
}

- (())flashScrollIndicators {
    log!("TODO: [(UIScrollView*){:?} flashScrollIndicators]", this);
}

- (())scrollRectToVisible:(CGRect)rect animated:(bool)animated {
    log!("TODO: [(UIScrollView*){:?} scrollRectToVisible:{:?} animated:{}]", this, rect, animated);
}

- (())touchesMoved:(id)touches // NSSet* of UITouch*
         withEvent:(id)_event { // UIEvent*
    let scroll_enabled: bool = msg![env; this scrollEnabled];
    if !scroll_enabled {
        return;
    }

    let touch_arr: id = msg![env; touches allObjects];
    // Assume single finger touches for now
    let touch: id = msg![env; touch_arr objectAtIndex:0u32];
    let bounds: CGRect = msg![env; this bounds];

    let prev_location: CGPoint = msg![env; touch previousLocationInView:this];
    let prev_x = prev_location.x;
    let prev_y = prev_location.y;

    let new_location: CGPoint = msg![env; touch locationInView:this];
    let y = new_location.y;
    let x = new_location.x;

    let delta_y = y - prev_y;
    let delta_x = x - prev_x;

    let offset: CGPoint = msg![env; this contentOffset];
    let content_size: CGSize = msg![env; this contentSize];

    // Very rudimentary scrolling.
    // We emulate sliding up to scroll down like on the real iPhone.
    let mut new_content_offset: CGPoint = CGPoint { x: offset.x - delta_x, y: offset.y - delta_y };

    // Update content offset within bounds
    new_content_offset.y = new_content_offset.y.min(content_size.height - bounds.size.height).max(0.0);
    new_content_offset.x = new_content_offset.x.min(content_size.width - bounds.size.width).max(0.0);

    // Trigger rerender only if required.
    log_dbg!("content offset: old {:?}, new {:?}", offset, new_content_offset);
    if new_content_offset != offset {
        () = msg![env; this setContentOffset:new_content_offset];

        let delegate: id = msg![env; this delegate];
        let sel: SEL = env
            .objc
            .register_host_selector("scrollViewDidScroll:".to_string(), &mut env.mem);
        let responds: bool = msg![env; delegate respondsToSelector:sel];
        if responds {
            () = msg![env; delegate scrollViewDidScroll:this];
        }
    }
}

@end

};
