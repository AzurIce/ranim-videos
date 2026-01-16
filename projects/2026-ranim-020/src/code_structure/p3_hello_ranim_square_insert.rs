use std::f64::consts::PI;

use ranim::{
    anims::{creation::WritingAnim, fading::FadingAnim, transform::TransformAnim},
    color::palettes::manim,
    glam::DVec3,
    items::vitem::{
        VItem,
        geometry::{Circle, Square},
    },
    prelude::*,
};

#[scene]
#[output(dir = "hello_ranim")]
pub fn hello_ranim(r: &mut RanimScene) {
    let _r_cam = r.insert(CameraFrame::default());

    let mut square = Square::new(2.0);
    square.set_color(manim::BLUE_C);

    let r_square = r.insert(square.clone());
    {
        let t = r.timeline_mut(r_square);
        t.forward(1.0);
    }
    // ...
}
