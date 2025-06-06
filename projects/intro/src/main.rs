use std::f32::consts::PI;

use ranim::{
    animation::{creation::WritingAnimSchedule, transform::GroupTransformAnimSchedule},
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
    fn construct(self, timeline: &RanimTimeline, camera: PinnedItem<CameraFrame>) {
        let frame_size = camera.data.frame_size();
        let logo_width = frame_size.y * 0.618;

        let mut logo = build_logo(logo_width)
            .map(|item| timeline.insert(item))
            .into_iter()
            .collect::<Group<_>>();

        timeline
            .play(logo.lagged_anim(0.0, |item| {
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
                timeline.play(
                    chunk
                        .transform(|data| {
                            data.scale_by_anchor(scale, anchor)
                                .scale_by_anchor(vec3(0.9, 0.9, 1.0), Anchor::origin())
                                .shift(vec3(0.0, frame_size.y / 9.0, 0.0));
                        })
                        .with_rate_func(smooth)
                        .apply(),
                );
            });

        let font_size = "50pt";
        let mut version_text = Group::<VItem>::from_svg(typst_svg!(
            format!(
                r#"
#align(center)[
    #text({font_size}, font: "LXGW Bright")[ranim v0.1.0]
    #text({font_size}, font: "LXGW Bright", fill: orange)[-alpha]
]"#
            )
            .as_str()
        ));
        version_text
            .shift(vec3(0.0, -frame_size.y * 2.5 / 8.0, 0.0))
            .scale(Vec3::splat(2.0));
        let r_bl = version_text.as_ref()[0].get_bounding_box_point(ivec3(-1, -1, 0));
        let r = version_text.get_bounding_box_point(ivec3(1, 0, 0)).x;
        let mut version_text = Group::<VItem>::from_svg(typst_svg!(
            format!(
                r#"
#align(center)[
    #text({font_size}, font: "LXGW Bright", fill: orange)[alpha]
]"#
            )
            .as_str()
        ));
        version_text
            .shift(
                vec3(r, r_bl.y, 0.0)
                    - version_text
                        .as_ref()
                        .last()
                        .unwrap()
                        .get_bounding_box_point(ivec3(1, -1, 0)),
            )
            .scale(Vec3::splat(2.0));
        let mut ranim_text = Group::<VItem>::from_svg(typst_svg!(
            format!(
                r#"
#align(center)[
    #text({font_size}, font: "LXGW Bright")[ranim v0.1.0]
]"#
            )
            .as_str()
        ));
        println!("ranim_text: {:?}", ranim_text.get_bounding_box());
        ranim_text
            .shift(
                (r_bl - ranim_text.as_ref()[0].get_bounding_box_point(ivec3(-1, -1, 0))).y
                    * Vec3::Y,
            )
            .scale(Vec3::splat(2.0));
        let ranim_left = ranim_text.get_bounding_box_point(ivec3(-1, 0, 0)).x;
        let mut ranim_text = ranim_text
            .into_iter()
            .map(|item| timeline.insert(item))
            .collect::<Group<_>>();
        timeline.play(
            ranim_text
                .lagged_anim(0.2, |item| item.write())
                .with_duration(2.0)
                .with_rate_func(linear),
        );
        timeline.sync();

        timeline.play(
            ranim_text
                .transform(|group| {
                    group.shift((r_bl.x - ranim_left) * Vec3::X);
                })
                .with_rate_func(smooth)
                .apply(),
        );
        let mut version_text = version_text
            .into_iter()
            .map(|item| timeline.insert(item))
            .collect::<Group<_>>();
        timeline.play(
            version_text
                .lagged_anim(0.2, |item| item.write())
                .with_total_duration(2.0)
                .with_rate_func(linear),
        );
        timeline.sync();

        timeline.forward(0.2);

        let mut all = logo
            .into_iter()
            .chain(version_text.into_iter())
            .chain(ranim_text.into_iter())
            .collect::<Group<_>>();
        timeline.play(all.lagged_anim(0.0, |item| {
            item.unwrite().with_duration(3.0).with_rate_func(smooth)
        }));
    }
}

#[scene]
struct TextScene;

impl TimelineConstructor for TextScene {
    fn construct<'t: 'r, 'r>(
        self,
        timeline: &'t RanimTimeline,
        camera: &'r mut Rabject<'t, CameraFrame>,
    ) {
        let frame_size = camera.data.frame_size();

        let mut rel = Group::<VItem>::from_svg(typst_svg!(
            r#"
#align(center)[
    #text(50pt, font: "LXGW Bright")[有趣]

    #text(50pt, font: "LXGW Bright")[确实是人用的]

    #text(80pt, font: "LXGW Bright", fill: orange)[I code my software]
]"#
        ));
        rel.scale(Vec3::splat(2.0));
        let l1_top_center = rel.get_bounding_box_point(ivec3(0, 1, 0));
        let l2_top_center = rel.get(3..).unwrap().get_bounding_box_point(ivec3(0, 1, 0));
        let l3_top_center = rel.get(9..).unwrap().get_bounding_box_point(ivec3(0, 1, 0));

        // let mut rel_group = rel
        //     .into_iter()
        //     .map(|item| timeline.insert(item))
        //     .collect::<Group<_>>();

        let mut text1 = Group::<VItem>::from_svg(typst_svg!(
            r#"
#align(center)[
    #text(50pt, font: "LXGW Bright")[有趣]
]"#
        ));
        text1.scale(Vec3::splat(2.0));

        let mut group1 = text1
            .into_iter()
            .map(|item| timeline.insert(item))
            .collect::<Group<_>>();
        timeline.play(
            group1
                .lagged_anim(0.2, |item| item.write())
                .with_duration(2.0),
        );

        timeline.sync();
        timeline.play(
            group1
                .transform(|data| {
                    data.put_anchor_on(Anchor::edge(0, 1, 0), l1_top_center);
                })
                .apply(),
        );
        let mut text2 = Group::<VItem>::from_svg(typst_svg!(
            r#"
#align(center)[
    #text(50pt, font: "LXGW Bright")[确实是人用的]
]"#
        ));
        text2.scale(Vec3::splat(2.0));

        let mut group2 = text2
            .into_iter()
            .map(|item| timeline.insert(item))
            .collect::<Group<_>>();
        timeline.play(
            group2
                .lagged_anim(0.2, |item| item.write())
                .with_duration(2.0),
        );

        timeline.sync();
        timeline.play(
            group2
                .transform(|data| {
                    data.put_anchor_on(Anchor::edge(0, 1, 0), l2_top_center);
                })
                .apply(),
        );
        let mut text3 = Group::<VItem>::from_svg(typst_svg!(
            r#"
#align(center)[
    #text(80pt, font: "LXGW Bright", fill: orange)[I code my software]
]"#
        ));
        text3.scale(Vec3::splat(2.0));
        text3.put_anchor_on(Anchor::edge(0, 1, 0), l3_top_center);

        let mut group3 = text3
            .into_iter()
            .map(|item| timeline.insert(item))
            .collect::<Group<_>>();
        timeline.play(
            group3
                .lagged_anim(0.2, |item| item.write())
                .with_duration(2.0),
        );
        timeline.sync();

        timeline.forward(1.0);

        let mut all = group1
            .into_iter()
            .chain(group2)
            .chain(group3)
            .collect::<Group<_>>();
        timeline.play(all.lagged_anim(0.0, |item| {
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
    //  #[cfg(debug_assertions)]
    // render_timeline(
    //     TextScene,
    //     &AppOptions {
    //         frame_size: (1280, 720),
    //         frame_rate: 20,
    //         ..Default::default()
    //     },
    // );
    // #[cfg(not(debug_assertions))]
    // render_timeline(
    //     TextScene,
    //     &AppOptions {
    //         frame_size: (3840, 2160),
    //         frame_rate: 60,
    //         ..Default::default()
    //     },
    // );
}
