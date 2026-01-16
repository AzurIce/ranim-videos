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

    let r_main = r.insert_empty();
    {
        let t = r.timeline_mut(r_main);
        t.play(square.clone().fade_in());
    }

    let mut circle = Circle::new(2.0);
    circle
        .set_color(manim::RED_C)
        .rotate(-PI / 4.0 + PI, DVec3::Z);

    let mut vitem = VItem::from(square);
    {
        let t = r.timeline_mut(r_main);
        t.play(vitem.transform_to(circle.into()));
        t.forward(1.0);
        t.play(vitem.clone().unwrite());
        t.play(vitem.write());
        t.play(vitem.fade_out());
    };

    r.insert_time_mark(3.7, TimeMark::Capture("preview.png".to_string()));
}
