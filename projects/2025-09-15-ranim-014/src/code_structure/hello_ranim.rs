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

    let mut circle = Circle::new(2.0);
    circle
        .set_color(manim::RED_C)
        .rotate(-PI / 4.0 + PI, DVec3::Z);

    let r_vitem = r.map(r_square, VItem::from);
    {
        let t = r.timeline_mut(&r_vitem);
        t.play_with(|state| state.transform_to(circle.into()));
        t.forward(1.0);
        let circle = t.snapshot();
        t.play_with(|circle| circle.unwrite());
        t.play(circle.write());
        t.play_with(|circle| circle.fade_out());
    };

    r.insert_time_mark(3.7, TimeMark::Capture("preview.png".to_string()));
}
