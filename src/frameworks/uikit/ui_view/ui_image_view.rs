/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `UIImageView`.

use crate::frameworks::core_graphics::cg_image::CGImageRef;
use crate::frameworks::core_graphics::{CGPoint, CGRect, CGSize};
use crate::frameworks::foundation::ns_string::get_static_str;
use crate::frameworks::foundation::{NSTimeInterval, NSUInteger};
use crate::objc::{
    id, impl_HostObject_with_superclass, msg, msg_super, nil, objc_classes, release, retain,
    ClassExports, NSZonePtr,
};

#[derive(Default)]
struct UIImageViewHostObject {
    superclass: super::UIViewHostObject,
    /// `UIImage*`
    image: id,
    /// `NSArray<UIImage *>*`
    animation_images: id,
    animation_duration: NSTimeInterval,
    is_animating: bool,
}
impl_HostObject_with_superclass!(UIImageViewHostObject);

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation UIImageView: UIView

+ (id)allocWithZone:(NSZonePtr)_zone {
    let host_object = Box::<UIImageViewHostObject>::default();
    env.objc.alloc_object(this, host_object, &mut env.mem)
}

- (id)initWithFrame:(CGRect)frame {
    let this: id = msg_super![env; this initWithFrame:frame];
    // Not sure if UIImageView does this unconditionally, or only for images
    // with alpha channels.
    () = msg![env; this setOpaque:false];
    this
}

- (())dealloc {
    let &UIImageViewHostObject {
        superclass: _,
        image,
        animation_images,
        ..
    } = env.objc.borrow(this);
    release(env, image);
    release(env, animation_images);
    msg_super![env; this dealloc]
}

// NSCoding implementation
- (id)initWithCoder:(id)coder {
    let this: id = msg_super![env; this initWithCoder:coder];

    let key_ns_string = get_static_str(env, "UIImage");
    let image: id = msg![env; coder decodeObjectForKey:key_ns_string];

    () = msg![env; this setImage:image];

    this
}

- (id)initWithImage:(id)image { // UIImage*
    let size: CGSize = msg![env; image size];
    let frame = CGRect {
        origin: CGPoint { x: 0.0, y: 0.0 },
        size
    };
    let this = msg_super![env; this initWithFrame:frame];
    () = msg![env; this setImage:image];
    // Not sure if UIImageView does this unconditionally, or only for images
    // with alpha channels.
    () = msg![env; this setOpaque:false];
    this
}

- (id)image {
    env.objc.borrow::<UIImageViewHostObject>(this).image
}

- (())setImage:(id)new_image { // UIImage*
    let host_obj = env.objc.borrow_mut::<UIImageViewHostObject>(this);
    let old_image = std::mem::replace(&mut host_obj.image, new_image);
    retain(env, new_image);
    release(env, old_image);

    let layer: id = msg![env; this layer];
    let cg_image: CGImageRef = msg![env; new_image CGImage];
    () = msg![env; layer setContents:cg_image];
}

- (id)animationImages {
    env.objc.borrow::<UIImageViewHostObject>(this).animation_images
}
- (())setAnimationImages:(id)images { // NSArray<UIImage *>*
    let old_images = std::mem::replace(
        &mut env.objc.borrow_mut::<UIImageViewHostObject>(this).animation_images,
        images
    );
    retain(env, images);
    release(env, old_images);

    // If we're not currently showing a static image, show the first frame.
    let current_image = env.objc.borrow::<UIImageViewHostObject>(this).image;
    if current_image == nil && images != nil {
        let count: NSUInteger = msg![env; images count];
        if count > 0 {
            let first_image: id = msg![env; images objectAtIndex:0u32];
            () = msg![env; this setImage:first_image];
            // setImage just updated host_obj.image, but we want it to stay nil
            // if we're just showing the first frame of an animation.
            env.objc.borrow_mut::<UIImageViewHostObject>(this).image = nil;
        }
    }
}

- (NSTimeInterval)animationDuration {
    env.objc.borrow::<UIImageViewHostObject>(this).animation_duration
}
- (())setAnimationDuration:(NSTimeInterval)duration {
    env.objc.borrow_mut::<UIImageViewHostObject>(this).animation_duration = duration;
}

- (bool)isAnimating {
    env.objc.borrow::<UIImageViewHostObject>(this).is_animating
}
- (())startAnimating {
    env.objc.borrow_mut::<UIImageViewHostObject>(this).is_animating = true;
    log!("TODO: [(UIImageView*) {:?} startAnimating] (not yet fully implemented)", this);
}

- (())stopAnimating {
    env.objc.borrow_mut::<UIImageViewHostObject>(this).is_animating = false;
}

@end

};
