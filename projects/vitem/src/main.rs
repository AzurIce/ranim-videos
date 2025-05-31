use std::{f64::consts::PI, ops::{Deref, DerefMut}};

use itertools::Itertools;
use ranim::{
    animation::{creation::WritingAnim, fading::FadingAnim, transform::TransformAnim}, color::palettes::manim, components::ScaleHint, glam::{dvec3, DVec3}, items::{
        vitem::{geometry::{Circle, Square}, svg::SvgItem, typst::typst_svg, VItem}, Group
    }, prelude::{scene, *}, render::primitives::{vitem::VItemPrimitive, Extract}, render_scene, timeline::{TimelineTrait, TimelinesFunc}, traits::{Color, FillColor, Shift, StrokeColor, With}, AppOptions, TimelineConstructor
};

#[derive(Clone)]
pub struct VisualVItem(VItem);

// impl Deref for VisualVItem {
//     type Target = VItem;
//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

// impl DerefMut for VisualVItem {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.0
//     }
// }

impl Interpolatable for VisualVItem {
    fn lerp(&self, target: &Self, t: f64) -> Self {
        Self(self.0.lerp(&target.0, t))
    }
}

impl Alignable for VisualVItem {
    fn is_aligned(&self, other: &Self) -> bool {
        self.0.is_aligned(&other.0)
    }
    fn align_with(&mut self, other: &mut Self) {
        self.0.align_with(&mut other.0);
    }
}

impl Partial for VisualVItem {
    fn get_partial(&self, range: std::ops::Range<f64>) -> Self {
        Self(self.0.get_partial(range))
    }
    fn get_partial_closed(&self, range: std::ops::Range<f64>) -> Self {
        Self(self.0.get_partial_closed(range))
    }
}

impl Opacity for VisualVItem {
    fn set_opacity(&mut self, opacity: f32) -> &mut Self {
        self.0.set_opacity(opacity);
        self
    }
}

impl StrokeColor for VisualVItem {
    fn set_stroke_color(&mut self, color: color::AlphaColor<color::Srgb>) -> &mut Self {
        self.0.set_stroke_color(color);
        self
    }
    fn set_stroke_opacity(&mut self, opacity: f32) -> &mut Self {
        self.0.set_stroke_opacity(opacity);
        self
    }
    fn stroke_color(&self) -> color::AlphaColor<color::Srgb> {
        self.0.stroke_color()
    }
}

impl FillColor for VisualVItem {
    fn set_fill_color(&mut self, color: color::AlphaColor<color::Srgb>) -> &mut Self {
        self.0.set_fill_color(color);
        self
    }
    fn set_fill_opacity(&mut self, opacity: f32) -> &mut Self {
        self.0.set_fill_opacity(opacity);
        self
    }
    fn fill_color(&self) -> color::AlphaColor<color::Srgb> {
        self.0.fill_color()
    }
}

impl StrokeWidth for VisualVItem {
    fn apply_stroke_func(&mut self, f: impl for<'a> Fn(&'a mut [ranim::components::width::Width])) -> &mut Self {
        self.0.apply_stroke_func(f);
        self
    }
    fn set_stroke_width(&mut self, width: f32) -> &mut Self {
        self.0.set_stroke_width(width);
        self
    }
}

impl Extract for VisualVItem {
    type Target = Vec<VItemPrimitive>;
    fn extract(&self) -> Self::Target {
        let mut points = Vec::with_capacity(self.0.vpoints.len());
        self.0.vpoints.iter().enumerate().for_each(|(idx, p)| {
            points.push(if idx % 2 == 0 {
                Circle::new(0.06).with(|circle| {
                    circle
                        .put_center_on(*p)
                        .set_color(manim::BLUE_C)
                        .set_fill_opacity(0.6);
                })
            } else {
                Circle::new(0.04).with(|circle| {
                    circle
                        .put_center_on(*p)
                        .set_color(manim::WHITE)
                        .set_stroke_opacity(0.8)
                        .set_fill_opacity(0.4);
                })
            });
        });
        let mut lines = Vec::with_capacity(self.0.vpoints.len());
        self.0
            .vpoints
            .iter()
            .step_by(2)
            .zip(self.0.vpoints.iter().skip(1).step_by(2))
            .zip(self.0.vpoints.iter().skip(2).step_by(2))
            .for_each(|((p0, p1), p2)| {
                if p0 != p1 {
                    lines.extend_from_slice(&[
                        VItem::from_vpoints(vec![*p0, (p0 + p1) / 2.0, *p1]),
                        VItem::from_vpoints(vec![*p1, (p1 + p2) / 2.0, *p2]),
                    ]);
                }
            });
        [self.0.extract()]
            .into_iter()
            .chain(lines.into_iter().map(|x| {
                x.with(|item| {
                    item.set_stroke_width(0.015);
                })
                .extract()
            }))
            .chain(points.into_iter().map(|x| x.extract()))
            .collect()
    }
}

impl Empty for VisualVItem {
    fn empty() -> Self {
        Self(VItem::empty())
    }
}

#[scene]
struct VItemScene;

impl TimelineConstructor for VItemScene {
    fn construct(self, r: &mut RanimScene, r_cam: TimelineId<CameraFrame>) {
        let text = SvgItem::new(typst_svg("Ranim")).with(|item| {
            item.set_fill_color(manim::WHITE)
                .set_fill_opacity(0.5)
                .scale_to_with_stroke(ScaleHint::PorportionalY(3.6))
                .put_center_on(DVec3::ZERO);
        });
        let r_texts = Group::<VItem>::from(text)
            .into_iter()
            .map(VisualVItem)
            .map(|item| r.init_timeline(item))
            .collect::<Vec<_>>();
        for r_text in &r_texts {
            r.timeline_mut(r_text).show();
        }
        let default_cam = r.timeline(&r_cam).state().clone();
        r.timelines_mut().forward(1.0);
        r.timeline_mut(&r_cam).play_with(|cam| {
            cam.transform(|cam| {
                cam.scale = 0.3;
                cam.up = DVec3::NEG_X;
                cam.pos.shift(DVec3::NEG_X * 6.0);
            })
        });
        r.timelines_mut().forward(1.0);
        r.timeline_mut(&r_cam).play_with(|cam| {
            cam.transform(|cam| {
                cam.pos.shift(DVec3::X * 12.0);
            })
            .with_duration(7.0)
        });
        r.timelines_mut().forward(1.0);
        r.timeline_mut(&r_cam)
            .play_with(|cam| cam.transform_to(default_cam));

        // r.timelines_mut().forward(1.0);
    }
}

#[scene]
struct VItemHelloScene;

impl TimelineConstructor for VItemHelloScene {
    fn construct(self, r: &mut RanimScene, r_cam: TimelineId<CameraFrame>) {
        let square = VisualVItem(VItem::from(Square::new(2.0).with(|square| {
            square.set_color(manim::BLUE_C);
        })));
        let r_vitem = r.init_timeline(square);

        let circle = VisualVItem(VItem::from(Circle::new(2.0).with(|circle| {
            circle
                .set_color(manim::RED_C)
                .rotate(- PI / 4.0 + PI, DVec3::Z);
        })));

        {
            let timeline = r.timeline_mut(&r_vitem);
            timeline.play_with(|item| item.transform_to(circle));
            timeline.forward(1.0);
            let circle = timeline.state().clone();
            timeline.play_with(|circle| circle.unwrite());
            timeline.play(circle.write());
            timeline.play_with(|circle| circle.fade_out());
        }
        r.timelines_mut().sync();
    }
}


fn main() {
    // render_scene(
    //     VItemScene,
    //     &AppOptions {
    //         pixel_size: (2560, 1440),
    //         ..Default::default()
    //     },
    // );
    render_scene(
        VItemHelloScene,
        &AppOptions {
            pixel_size: (2560, 1440),
            ..Default::default()
        },
    );
}
