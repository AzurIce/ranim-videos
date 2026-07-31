use ranim::anims::fading::FadingAnim;
use ranim::color::palettes::manim::{BLUE_C, RED_C, WHITE};
use ranim::core::traits::ScaleHint;
use ranim::glam::DVec3;
use ranim::items::vitem::svg::SvgItem;
use ranim::items::vitem::typst::typst_svg;
use ranim::items::vitem::{
    VItem,
    geometry::{Circle, Square},
};
use ranim::prelude::*;

const SVG_CAR: &str = include_str!("../assets/car.svg");

#[scene]
#[preview]
#[output]
fn ppt(r: &mut RanimScene) {
    let r_cam = r.insert(CameraFrame::new());
    let r_svg = r.insert(SvgItem::new(SVG_CAR).with(|x| {
        x.scale_to_with_stroke(ScaleHint::PorportionalY(6.0));
    }));
    // let r_svg =;
    // let r_cam = r.insert_and_show(CameraFrame::new());
    // let r_text = r.insert_and(
    //     Group::<VItem>::from(SvgItem::new(typst_svg("Transform"))).with(|x| {
    //         x.set_color(WHITE)
    //             .scale_to_with_stroke(ScaleHint::PorportionalY(0.5))
    //             .shift(DVec3::Y * 3.0);
    //     }),
    //     |t| {
    //         t.play_with(|x| x.fade_in().with_duration(0.4))
    //             .forward(0.4)
    //             .play_with(|x| x.fade_out().with_duration(0.4));
    //     },
    // );
    // let r_square = r.insert_and(
    //     VItem::from(Square::new(3.0)).with(|x| {
    //         x.set_color(BLUE_C);
    //     }),
    //     |t| {
    //         t.show()
    //             .forward(0.2)
    //             .play_with(|s| {
    //                 s.transform_to(VItem::from(Circle::new(1.5)).with(|x| {
    //                     x.set_color(RED_C);
    //                 }))
    //                 .with_duration(0.6)
    //             })
    //             .forward(0.2);
    //     },
    // );
}
