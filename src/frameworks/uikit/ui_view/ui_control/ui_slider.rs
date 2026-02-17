/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `UISlider`.

use crate::frameworks::core_graphics::CGRect;
use crate::objc::{id, msg_super, objc_classes, todo_objc_setter, ClassExports};

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation UISlider: UIControl

- (id)initWithFrame:(CGRect)frame {
    log!("[(UISlider*){:?} initWithFrame:{:?}] TODO: Implement UISlider. The control won't be rendered.", this, frame);
    let this: id = msg_super![env; this initWithFrame:frame];
    let color: id = msg_class![env; UIColor lightGrayColor];
    () = msg![env; this setBackgroundColor:color];
    this
}

// NSCoding implementation
- (id)initWithCoder:(id)coder {
    log!("[(UISlider*){:?} initWithCoder:{:?}] TODO: Implement UISlider. The control won't be rendered.", this, coder);
    let this: id = msg_super![env; this initWithCoder:coder];
    let color: id = msg_class![env; UIColor lightGrayColor];
    () = msg![env; this setBackgroundColor:color];
    this
}

- (())setMinimumValueImage:(id)img { // UIImage *
    todo_objc_setter!(this, img);
}
- (())setMaximumValueImage:(id)img { // UIImage *
    todo_objc_setter!(this, img);
}

- (f32)value {
    0.5
}
- (())setValue:(f32)val {
    todo_objc_setter!(this, val);
}

- (f32)minimumValue {
    0.0
}
- (())setMinimumValue:(f32)val {
    todo_objc_setter!(this, val);
}

- (f32)maximumValue {
    1.0
}
- (())setMaximumValue:(f32)val {
    todo_objc_setter!(this, val);
}

- (bool)isContinuous {
    true
}
- (())setContinuous:(bool)val {
    todo_objc_setter!(this, val);
}

@end

};
