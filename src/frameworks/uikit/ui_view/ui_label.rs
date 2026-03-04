/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `UILabel`.

use crate::frameworks::core_graphics::cg_context::CGContextSetRGBFillColor;
use crate::frameworks::core_graphics::{CGFloat, CGPoint, CGRect, CGSize};
use crate::frameworks::foundation::ns_string::get_static_str;
use crate::frameworks::foundation::NSInteger;
use crate::frameworks::uikit::ui_color;
use crate::frameworks::uikit::ui_font::{
    UILineBreakMode, UILineBreakModeTailTruncation, UITextAlignment, UITextAlignmentCenter,
    UITextAlignmentLeft, UITextAlignmentRight,
};
use crate::frameworks::uikit::ui_graphics::UIGraphicsGetCurrentContext;
use crate::objc::{
    id, impl_HostObject_with_superclass, msg, msg_class, msg_super, nil, objc_classes, release,
    retain, todo_objc_setter, ClassExports, NSZonePtr,
};

pub struct UILabelHostObject {
    superclass: super::UIViewHostObject,
    /// `NSString*`
    text: id,
    /// `UIFont*`
    font: id,
    /// `UIColor*`
    text_color: id,
    text_alignment: UITextAlignment,
    line_break_mode: UILineBreakMode,
    number_of_lines: NSInteger,
    adjusts_font_size_to_fit_width: bool,
    baseline_adjustment: NSInteger,
    /// `UIColor*`
    shadow_color: id,
    shadow_offset: CGSize,
}
impl_HostObject_with_superclass!(UILabelHostObject);
impl Default for UILabelHostObject {
    fn default() -> Self {
        UILabelHostObject {
            superclass: Default::default(),
            text: nil,
            font: nil,
            text_color: nil,
            text_alignment: UITextAlignmentLeft,
            line_break_mode: UILineBreakModeTailTruncation,
            number_of_lines: 1,
            adjusts_font_size_to_fit_width: false,
            baseline_adjustment: 0, // UIBaselineAdjustmentAlignBaselines
            shadow_color: nil,
            shadow_offset: CGSize {
                width: 0.0,
                height: -1.0,
            },
        }
    }
}

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation UILabel: UIView

+ (id)allocWithZone:(NSZonePtr)_zone {
    let host_object = Box::<UILabelHostObject>::default();
    env.objc.alloc_object(this, host_object, &mut env.mem)
}

- (id)initWithCoder:(id)coder {
    let this: id = msg_super![env; this initWithCoder:coder];

    // TODO: Decode other property values from the coder
    () = msg![env; this setFont:nil];

    let key_ns_string = get_static_str(env, "UIText");
    let text: id = msg![env; coder decodeObjectForKey:key_ns_string];
    () = msg![env; this setText:text];

    let key_ns_string = get_static_str(env, "UITextColor");
    let text_color: id = msg![env; coder decodeObjectForKey:key_ns_string];
    () = msg![env; this setTextColor:text_color];

    let key_ns_string = get_static_str(env, "UIFont");
    let font: id = msg![env; coder decodeObjectForKey:key_ns_string];
    if font != nil {
        () = msg![env; this setFont:font];
    }

    let key_ns_string = get_static_str(env, "UITextAlignment");
    if msg![env; coder containsValueForKey:key_ns_string] {
        let alignment: i32 = msg![env; coder decodeIntForKey:key_ns_string];
        () = msg![env; this setTextAlignment:(alignment as UITextAlignment)];
    }

    let key_ns_string = get_static_str(env, "UILineBreakMode");
    if msg![env; coder containsValueForKey:key_ns_string] {
        let mode: i32 = msg![env; coder decodeIntForKey:key_ns_string];
        () = msg![env; this setLineBreakMode:(mode as UILineBreakMode)];
    }

    let key_ns_string = get_static_str(env, "UINumberOfLines");
    if msg![env; coder containsValueForKey:key_ns_string] {
        let lines: i32 = msg![env; coder decodeIntForKey:key_ns_string];
        () = msg![env; this setNumberOfLines:(lines as NSInteger)];
    }

    let key_ns_string = get_static_str(env, "UIShadowColor");
    let shadow_color: id = msg![env; coder decodeObjectForKey:key_ns_string];
    if shadow_color != nil {
        () = msg![env; this setShadowColor:shadow_color];
    }

    let key_ns_string = get_static_str(env, "UIShadowOffset");
    if msg![env; coder containsValueForKey:key_ns_string] {
        let offset: CGSize = msg![env; coder decodeCGSizeForKey:key_ns_string];
        () = msg![env; this setShadowOffset:offset];
    }

    let key_ns_string = get_static_str(env, "UIBackgroundColor");
    let bg_color: id = msg![env; coder decodeObjectForKey:key_ns_string];
    let bg_color = if bg_color == nil {
        // Setting nil to the background color will fall back
        // to a white color, but testing with BoD screens,
        // it seems to use the transparent one
        msg_class![env; UIColor clearColor]
    } else {
        bg_color
    };
    () = msg![env; this setBackgroundColor:bg_color];

    // Built-in views don't have user-controlled opaqueness.
    () = msg_super![env; this setOpaque:false];
    this
}

- (id)initWithFrame:(CGRect)frame {
    let this: id = msg_super![env; this initWithFrame:frame];
    // These aren't redundant, the setters fetch the real defaults.
    () = msg![env; this setFont:nil];
    () = msg![env; this setTextColor:nil];
    () = msg![env; this setBackgroundColor:nil];
    // Built-in views don't have user-controlled opaqueness.
    () = msg_super![env; this setOpaque:false];
    this
}

- (())dealloc {
    let &UILabelHostObject {
        superclass: _,
        text,
        font,
        text_color,
        text_alignment: _,
        line_break_mode: _,
        number_of_lines: _,
        adjusts_font_size_to_fit_width: _,
        baseline_adjustment: _,
        shadow_color,
        shadow_offset: _,
    } = env.objc.borrow(this);
    release(env, text);
    release(env, font);
    release(env, text_color);
    release(env, shadow_color);
    msg_super![env; this dealloc]
}

// TODO: initWithCoder:

- (id)text {
    env.objc.borrow::<UILabelHostObject>(this).text
}
- (())setText:(id)new_text { // NSString*
    let new_text: id = msg![env; new_text copy];
    let old_text = std::mem::replace(
        &mut env.objc.borrow_mut::<UILabelHostObject>(this).text,
        new_text
    );
    release(env, old_text);

    () = msg![env; this setNeedsDisplay];
}

- (id)font {
    env.objc.borrow::<UILabelHostObject>(this).font
}
- (())setFont:(id)new_font { // UIFont*
    let new_font: id = if new_font == nil {
        // reset to default
        let size: CGFloat = 17.0;
        msg_class![env; UIFont systemFontOfSize:size]
    } else {
        new_font
    };

    let old_font = std::mem::replace(
        &mut env.objc.borrow_mut::<UILabelHostObject>(this).font,
        new_font
    );
    retain(env, new_font);
    release(env, old_font);

    () = msg![env; this setNeedsDisplay];
}

- (bool)adjustsFontSizeToFitWidth {
    env.objc.borrow::<UILabelHostObject>(this).adjusts_font_size_to_fit_width
}
- (())setAdjustsFontSizeToFitWidth:(bool)adjusts {
    env.objc.borrow_mut::<UILabelHostObject>(this).adjusts_font_size_to_fit_width = adjusts;
    () = msg![env; this setNeedsDisplay];
}

- (CGFloat)minimumFontSize {
    0.0 // TODO: store it
}
- (())setMinimumFontSize:(CGFloat)size {
    todo_objc_setter!(this, size);
}

- (id)textColor {
    env.objc.borrow::<UILabelHostObject>(this).text_color
}
- (())setTextColor:(id)new_text_color { // UIFont*
    let new_text_color: id = if new_text_color == nil {
        msg_class![env; UIColor blackColor]
    } else {
        new_text_color
    };

    let old_text_color = std::mem::replace(
        &mut env.objc.borrow_mut::<UILabelHostObject>(this).text_color,
        new_text_color
    );
    retain(env, new_text_color);
    release(env, old_text_color);

    () = msg![env; this setNeedsDisplay];
}

- (())setBackgroundColor:(id)color { // UIColor*
    // This overrides the standard setBackgroundColor: accessor on UIView.
    // UILabel seems to default to white, and setting the background color to
    // nil also just gives white, rather than the normal transparency. I don't
    // know how or why it does that, but overriding this setter seems like a
    // reasonable way to match that behavior.
    let color: id = if color == nil {
        msg_class![env; UIColor whiteColor]
    } else {
        color
    };
    msg_super![env; this setBackgroundColor:color]
}

- (id)shadowColor {
    env.objc.borrow::<UILabelHostObject>(this).shadow_color
}
- (())setShadowColor:(id)color { // UIColor*
    let old_color = std::mem::replace(
        &mut env.objc.borrow_mut::<UILabelHostObject>(this).shadow_color,
        color
    );
    retain(env, color);
    release(env, old_color);
    () = msg![env; this setNeedsDisplay];
}
- (CGSize)shadowOffset {
    env.objc.borrow::<UILabelHostObject>(this).shadow_offset
}
- (())setShadowOffset:(CGSize)value {
    env.objc.borrow_mut::<UILabelHostObject>(this).shadow_offset = value;
    () = msg![env; this setNeedsDisplay];
}

- (())setOpaque:(bool)_opaque {
    // Built-in views don't have user-controlled opaqueness.
}

- (UITextAlignment)textAlignment {
    env.objc.borrow::<UILabelHostObject>(this).text_alignment
}
- (())setTextAlignment:(UITextAlignment)text_alignment { // UIFont*
    env.objc.borrow_mut::<UILabelHostObject>(this).text_alignment = text_alignment;
    () = msg![env; this setNeedsDisplay];
}

- (UILineBreakMode)lineBreakMode {
    env.objc.borrow::<UILabelHostObject>(this).line_break_mode
}
- (())setLineBreakMode:(UILineBreakMode)line_break_mode { // UIFont*
    env.objc.borrow_mut::<UILabelHostObject>(this).line_break_mode = line_break_mode;
    () = msg![env; this setNeedsDisplay];
}

- (NSInteger)numberOfLines {
    env.objc.borrow::<UILabelHostObject>(this).number_of_lines
}
- (())setNumberOfLines:(NSInteger)number {
    env.objc.borrow_mut::<UILabelHostObject>(this).number_of_lines = number;
    if number != 0 && number != 1 {
        log!("TODO: UILabel numberOfLines > 1 (label {:?})", this);
    }
    () = msg![env; this setNeedsDisplay];
}

- (CGSize)sizeThatFits:(CGSize)size {
    let host_obj = env.objc.borrow::<UILabelHostObject>(this);
    let text = host_obj.text;
    let font = host_obj.font;
    let line_break_mode = host_obj.line_break_mode;
    let number_of_lines = host_obj.number_of_lines;

    if text == nil {
        return CGSize { width: 0.0, height: 0.0 };
    }

    let mut res: CGSize = if number_of_lines == 1 {
        msg![env; text sizeWithFont:font]
    } else {
        msg![env; text sizeWithFont:font
                  constrainedToSize:size
                      lineBreakMode:line_break_mode]
    };
    if res.width.is_nan() { res.width = 0.0; }
    if res.height.is_nan() { res.height = 0.0; }
    res
}

- (NSInteger)baselineAdjustment {
    env.objc.borrow::<UILabelHostObject>(this).baseline_adjustment
}
- (())setBaselineAdjustment:(NSInteger)adjustment {
    env.objc.borrow_mut::<UILabelHostObject>(this).baseline_adjustment = adjustment;
    () = msg![env; this setNeedsDisplay];
}

- (bool)isEnabled {
    true
}
- (())setEnabled:(bool)enabled {
    todo_objc_setter!(this, enabled);
}

- (bool)isHighlighted {
    false
}
- (())setHighlighted:(bool)highlighted {
    todo_objc_setter!(this, highlighted);
}

- (())drawRect:(CGRect)_rect {
    let bounds: CGRect = msg![env; this bounds];
    let context = UIGraphicsGetCurrentContext(env);

    let &mut UILabelHostObject {
        superclass: _,
        text,
        font,
        text_color,
        text_alignment,
        line_break_mode,
        number_of_lines,
        shadow_color,
        shadow_offset,
        ..
    } = env.objc.borrow_mut(this);

    if text == nil {
        return;
    }

    // TODO: handle line counts other than 0 and 1 properly. 0 = unlimited
    // (note the log message in setNumberOfLines:)
    let single_line = number_of_lines == 1;

    let mut calculated_size: CGSize = if single_line {
        msg![env; text sizeWithFont:font]
    } else {
        msg![env; text sizeWithFont:font
                  constrainedToSize:(bounds.size)
                      lineBreakMode:line_break_mode]
    };
    if calculated_size.width.is_nan() { calculated_size.width = 0.0; }
    if calculated_size.height.is_nan() { calculated_size.height = 0.0; }

    // UILabel always vertically centers text
    let mut origin_y = bounds.origin.y + (bounds.size.height - calculated_size.height) / 2.0;
    if origin_y.is_nan() { origin_y = bounds.origin.y; }

    let rect = CGRect {
        origin: CGPoint {
            x: bounds.origin.x,
            y: origin_y,
        },
        size: CGSize {
            width: bounds.size.width,
            height: calculated_size.height,
        },
    };

    let x_offset_mult = match text_alignment {
        UITextAlignmentLeft => 0.0,
        UITextAlignmentCenter => 0.5,
        UITextAlignmentRight => 1.0,
        _ => unimplemented!(),
    };

    if shadow_color != nil {
        let (r, g, b, a) = ui_color::get_rgba(&env.objc, shadow_color);
        CGContextSetRGBFillColor(env, context, r, g, b, a);

        let shadow_rect = CGRect {
            origin: CGPoint {
                x: rect.origin.x + shadow_offset.width,
                y: rect.origin.y + shadow_offset.height,
            },
            size: rect.size,
        };

        if single_line {
            let point = CGPoint {
                x: shadow_rect.origin.x + x_offset_mult * (bounds.size.width - calculated_size.width),
                y: shadow_rect.origin.y
            };
            let _: CGSize = msg![env; text drawAtPoint:point
                                              withFont:font];
        } else {
            let _: CGSize = msg![env; text drawInRect:shadow_rect
                                             withFont:font
                                        lineBreakMode:line_break_mode
                                            alignment:text_alignment];
        }
    }

    let (r, g, b, a) = ui_color::get_rgba(&env.objc, text_color);
    CGContextSetRGBFillColor(env, context, r, g, b, a);

    let _size: CGSize = if single_line {
        let point = CGPoint {
            x: rect.origin.x + x_offset_mult * (bounds.size.width - calculated_size.width),
            y: rect.origin.y
        };
        msg![env; text drawAtPoint:point
                          withFont:font]
    } else {
        msg![env; text drawInRect:rect
                         withFont:font
                    lineBreakMode:line_break_mode
                        alignment:text_alignment]
    };
}

@end

};
