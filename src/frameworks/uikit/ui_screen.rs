/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `UIScreen`.

use crate::frameworks::core_graphics::{CGFloat, CGPoint, CGRect, CGSize};
use crate::objc::{id, msg, objc_classes, ClassExports, TrivialHostObject};

#[derive(Default)]
pub struct State {
    main_screen: Option<id>,
}

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

// For now this is a singleton (the only instance is returned by mainScreen),
// so there are hardcoded assumptions related to that.
@implementation UIScreen: NSObject

+ (id)mainScreen {
    if let Some(screen) = env.framework_state.uikit.ui_screen.main_screen {
        screen
    } else {
        let new = env.objc.alloc_static_object(
            this,
            Box::new(TrivialHostObject),
            &mut env.mem
        );
        env.framework_state.uikit.ui_screen.main_screen = Some(new);
        new
   }
}
- (id)retain { this }
- (())release {}
- (id)autorelease { this }

// TODO: more accessors

- (CGRect)bounds {
    // While Apple's documentation says this changes with the interface
    // orientation, https://useyourloaf.com/blog/uiscreen-bounds-in-ios-8/ says
    // ths wasn't the case prior to iOS 8.
    // However, some Universal apps might expect it to reflect the actual size
    // provided by the emulator.
    let (width, height) = env.window().size_unrotated_unscaled();
    use crate::window::DeviceOrientation;
    let (width, height) = match env.window().current_rotation() {
        DeviceOrientation::Portrait => (width, height),
        DeviceOrientation::LandscapeLeft | DeviceOrientation::LandscapeRight => (height, width),
    };
    CGRect {
        origin: CGPoint { x: 0.0, y: 0.0 },
        size: CGSize { width: width as f32, height: height as f32 },
    }
}

- (CGRect)applicationFrame {
    use crate::window::DeviceOrientation;
    let (width, height) = env.window().device_family().portrait_size();
    let (width, height) = (width as f32, height as f32);

    let mut rect = match env.window().current_rotation() {
        DeviceOrientation::Portrait => {
            let mut r = CGRect {
                origin: CGPoint { x: 0.0, y: 0.0 },
                size: CGSize { width, height },
            };
            if !env.framework_state.uikit.ui_application.status_bar_hidden {
                r.origin.y += 20.0;
                r.size.height -= 20.0;
            }
            r
        }
        DeviceOrientation::LandscapeLeft => {
            let mut r = CGRect {
                origin: CGPoint { x: 0.0, y: 0.0 },
                size: CGSize { width, height },
            };
            if !env.framework_state.uikit.ui_application.status_bar_hidden {
                r.origin.x += 20.0;
                r.size.width -= 20.0;
            }
            r
        }
        DeviceOrientation::LandscapeRight => {
            let mut r = CGRect {
                origin: CGPoint { x: 0.0, y: 0.0 },
                size: CGSize { width, height },
            };
            if !env.framework_state.uikit.ui_application.status_bar_hidden {
                r.size.width -= 20.0;
            }
            r
        }
    };
    rect
}

- (CGFloat)scale {
    // TODO: support retina
    1.0
}

- (id)currentMode {
    let mode: id = msg_class![env; UIScreenMode alloc];
    autorelease(env, msg![env; mode init])
}

@end

@implementation UIScreenMode: NSObject

- (CGSize)size {
    let (width, height) = env.window().device_family().portrait_size();
    CGSize { width: width as f32, height: height as f32 }
}

- (CGFloat)pixelAspectRatio {
    1.0
}

@end

};
