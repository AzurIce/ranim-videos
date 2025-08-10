use std::f32::consts::PI;

use ranim::{
    animation::{
        creation::WritingAnimSchedule,
        transform::{GroupTransformAnimSchedule, TransformAnimSchedule},
    },
    color::palettes::manim,
    components::Anchor,
    glam::{Vec3, ivec3, vec3},
    items::{
        group::Group,
        vitem::{Polygon, Rectangle, Square, VItem},
    },
    prelude::*,
    typst_svg,
    utils::rate_functions::{linear, smooth},
};

fn build_logo(logo_width: f32) -> [VItem; 6] {
    let mut red_bg_rect = Rectangle(logo_width / 2.0, logo_width).build();
    red_bg_rect
        .set_color(manim::RED_C.with_alpha(0.5))
        .put_center_on(vec3(-logo_width / 4.0, 0.0, 0.0));
    let mut red_rect = Rectangle(logo_width / 4.0, logo_width).build();
    red_rect
        .set_color(manim::RED_C)
        .put_anchor_on(Anchor::edge(1, 0, 0), vec3(-logo_width / 4.0, 0.0, 0.0));

    let mut green_bg_sq = Square(logo_width / 2.0).build();
    green_bg_sq
        .set_color(manim::GREEN_C.with_alpha(0.5))
        .put_center_on(vec3(logo_width / 4.0, logo_width / 4.0, 0.0));
    let mut green_triangle = Polygon(vec![
        vec3(0.0, logo_width / 2.0, 0.0),
        vec3(logo_width / 2.0, logo_width / 2.0, 0.0),
        vec3(logo_width / 2.0, 0.0, 0.0),
    ])
    .build(); // ◥
    green_triangle.set_color(manim::GREEN_C);

    let mut blue_bg_sq = Square(logo_width / 2.0).build();
    blue_bg_sq
        .set_color(manim::BLUE_C.with_alpha(0.5))
        .put_center_on(vec3(logo_width / 4.0, -logo_width / 4.0, 0.0));
    let mut blue_triangle = green_triangle.clone();
    blue_triangle
        .set_color(manim::BLUE_C)
        .rotate(PI, Vec3::Z)
        .shift(Vec3::NEG_Y * logo_width / 2.0); // ◣

    [
        red_bg_rect,
        red_rect,
        green_bg_sq,
        green_triangle,
        blue_bg_sq,
        blue_triangle,
    ]
}

#[scene]
struct LogoScene;

impl TimelineConstructor for LogoScene {
    fn construct<'t: 'r, 'r>(
        self,
        timeline: &'t RanimTimeline,
        camera: &'r mut Rabject<'t, CameraFrame>,
    ) {
        let frame_size = camera.data.frame_size();
        let logo_width = frame_size.y * 0.618;

        let mut logo = build_logo(logo_width)
            .map(|item| timeline.insert(item))
            .into_iter()
            .collect::<Group<_>>();

        timeline
            .play_group(logo.lagged_anim(0.0, |item| {
                item.write().with_duration(3.0).with_rate_func(smooth)
            }))
            .sync();

        let gap_ratio = 1.0 / 60.0;
        let gap = logo_width * gap_ratio;
        let scale = (logo_width - gap * 2.0) / logo_width;
        let scale = [
            vec3(scale, 1.0, 1.0),
            vec3(scale, scale, 1.0),
            vec3(scale, scale, 1.0),
        ];
        let anchor = [
            Anchor::edge(-1, 0, 0),
            Anchor::edge(1, 1, 0),
            Anchor::edge(1, -1, 0),
        ];
        logo.chunks_mut(2)
            .zip(scale.into_iter().zip(anchor))
            .for_each(|(chunk, (scale, anchor))| {
                timeline.play_group(
                    chunk
                        .transform(|data| {
                            data.scale_by_anchor(scale, anchor)
                                .scale_by_anchor(vec3(0.9, 0.9, 1.0), Anchor::origin())
                                .shift(vec3(0.0, frame_size.y / 9.0, 0.0));
                        })
                        .into_iter()
                        .map(|schedule| schedule.with_rate_func(smooth).apply())
                        .collect(),
                );
            });

        let mut version_text = Group::<VItem>::from_svg(typst_svg!(
            r#"
#align(center)[
    #text(50pt, font: "LXGW Bright")[ranim v0.1.0]
    #text(50pt, font: "LXGW Bright", fill: orange)[-alpha]
]"#
        ));
        version_text.shift(vec3(0.0, -frame_size.y * 2.5 / 8.0, 0.0));
        let r_bl = version_text.as_ref()[0].get_bounding_box_point(ivec3(-1, -1, 0));
        let r = version_text.get_bounding_box_point(ivec3(1, 0, 0)).x;
        let mut version_text = Group::<VItem>::from_svg(typst_svg!(
            r#"
#align(center)[
    #text(50pt, font: "LXGW Bright", fill: orange)[alpha]
]"#
        ));
        version_text.shift(
            vec3(r, r_bl.y, 0.0)
                - version_text
                    .as_ref()
                    .last()
                    .unwrap()
                    .get_bounding_box_point(ivec3(1, -1, 0)),
        );
        let mut ranim_text = Group::<VItem>::from_svg(typst_svg!(
            r#"
#align(center)[
    #text(50pt, font: "LXGW Bright")[ranim v0.1.0]
]"#
        ));
        println!("ranim_text: {:?}", ranim_text.get_bounding_box());
        ranim_text.shift(
            (r_bl - ranim_text.as_ref()[0].get_bounding_box_point(ivec3(-1, -1, 0))).y * Vec3::Y,
        );
        let ranim_left = ranim_text.get_bounding_box_point(ivec3(-1, 0, 0)).x;
        let mut ranim_text = ranim_text
            .into_iter()
            .map(|item| timeline.insert(item))
            .collect::<Group<_>>();
        let len = ranim_text.as_ref().len() as f32;
        let dur = 2.0 / (1.0 + (len - 1.0) * 0.2);
        timeline.play_group(ranim_text.lagged_anim(0.2, |item| {
            item.write().with_duration(dur).with_rate_func(linear)
        }));
        timeline.sync();

        timeline.play_group(ranim_text.lagged_anim(0.0, |item| {
            item.transform(|data| {
                data.shift((r_bl.x - ranim_left) * Vec3::X);
            })
            .with_rate_func(smooth)
            .apply()
        }));
        let mut version_text = version_text
            .into_iter()
            .map(|item| timeline.insert(item))
            .collect::<Group<_>>();
        let len = version_text.as_ref().len() as f32;
        let dur = 2.0 / (1.0 + (len - 1.0) * 0.2);
        timeline.play_group(version_text.lagged_anim(0.2, |item| {
            item.write().with_duration(dur).with_rate_func(linear)
        }));
        timeline.sync();

        timeline.forward(1.0);

        let mut all = logo
            .into_iter()
            .chain(version_text.into_iter())
            .chain(ranim_text.into_iter())
            .collect::<Group<_>>();
        timeline.play_group(all.lagged_anim(0.0, |item| {
            item.unwrite().with_duration(3.0).with_rate_func(smooth)
        }));
    }
}

fn main() {
    #[cfg(debug_assertions)]
    render_timeline(
        LogoScene,
        &AppOptions {
            frame_size: (1280, 720),
            frame_rate: 20,
            ..Default::default()
        },
    );
    #[cfg(not(debug_assertions))]
    render_timeline(
        LogoScene,
        &AppOptions {
            frame_size: (3840, 2160),
            frame_rate: 60,
            ..Default::default()
        },
    );
}