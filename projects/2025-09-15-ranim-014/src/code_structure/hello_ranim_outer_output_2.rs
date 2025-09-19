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
#[output(dir = "ranim-014", fps = 30)]
#[output] // dir = ".", pixel_size = (1920, 1080), fps = 60, save_frames = false
#[output(dir = "ranim-014", pixel_size = (2560, 1440))]
#[output(dir = "ranim-014", pixel_size = (3840, 2160), fps = 1, save_frames = true)]
fn hello_ranim(r: &mut RanimScene) {
    let _r_cam = r.insert_and_show(CameraFrame::default());
    // ...
}
