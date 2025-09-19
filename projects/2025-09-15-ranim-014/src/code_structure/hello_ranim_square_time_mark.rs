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
#[output(dir = "ranim-014")]
fn hello_ranim(r: &mut RanimScene) {
    let _r_cam = r.insert_and_show(CameraFrame::default());

    let mut square = Square::new(2.0);
    square.set_color(manim::BLUE_C).set_fill_opacity(0.8);

    let r_square = r.insert(square);
    {
        let t = r.timeline_mut(&r_square);
        t.play_with(|square| square.fade_in());
    }
    // ...
    r.insert_time_mark(0.5, TimeMark::Capture("fade-half.png".to_string()));
}
