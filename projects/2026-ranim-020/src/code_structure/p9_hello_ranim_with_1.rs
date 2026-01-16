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
pub fn hello_ranim_chained(r: &mut RanimScene) {
    let _r_cam = r.insert(CameraFrame::default());

    let r_square = r.insert_empty();
    r.timeline_mut(r_square).play(
        Square::new(2.0)
            .with(|square| {
                square.set_color(manim::BLUE_C);
            })
            .fade_in(),
    );

    let circle = Circle::new(2.0).with(|circle| {
        circle
            .set_color(manim::RED_C)
            .rotate(-PI / 4.0 + PI, DVec3::Z);
    });

    let mut vitem = VItem::from(Square::new(2.0).with(|square| {
        square.set_color(manim::BLUE_C);
    }));
    r.timeline_mut(r_square)
        .play(vitem.transform_to(circle.into()))
        .forward(1.0)
        .play(vitem.clone().unwrite())
        .play(vitem.write())
        .play(vitem.fade_out());

    r.insert_time_mark(3.7, TimeMark::Capture("preview.png".to_string()));
}
