use ranim::{
    animation::fading::FadingAnim,
    color::palettes::manim,
    components::ScaleHint,
    glam::{dvec3, DVec3},
    items::{
        vitem::{geometry::Rectangle, svg::SvgItem, typst::typst_svg, VItem}, Group
    },
    prelude::*,
    timeline::{AnimationInfo, RabjectTimelineInfo, TimelineTrait}, utils::rate_functions::linear,
};

pub enum AnimType {
    Static,
    Dynamic,
}

#[scene]
pub struct TimelineScene;

impl TimelineConstructor for TimelineScene {
    fn construct(self, r: &mut RanimScene, r_cam: TimelineId<CameraFrame>) {
        let text = Group::<VItem>::from(
            SvgItem::new(typst_svg(
                r#"#text(font: "JetBrainsMono NFM")[timeline.insert(square)]"#,
            ))
            .with(|text| {
                text.set_fill_color(manim::WHITE)
                    .scale_to_with_stroke(ScaleHint::PorportionalY(0.6))
                    .put_center_on(DVec3::Y * 3.5);
            }),
        );
        let r_text = r.init_timeline(text);
        // r.timelines_mut().forward(0.5);

        let timeline1 = Rectangle::new(0.1, 1.0).with(|rect| {
            rect.put_center_on(dvec3(-5.0, 2.3, 0.0));
        });
        let r_timeline1 = r.init_timeline(timeline1);
        // r.timelines_mut().sync();

        r.timeline_mut(&r_text)
            .play_with(|text| text.fade_in().with_duration(1.0).with_rate_func(linear));
        r.timeline_mut(&r_timeline1)
            .play_with(|rect| rect.fade_in().with_duration(1.0).with_rate_func(linear));

        r.timelines_mut().sync();
    }
}

fn main() {
    render_scene(TimelineScene, &AppOptions::default());
}
