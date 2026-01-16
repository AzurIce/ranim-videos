use std::{cmp::Ordering, ops::Range};

use ranim::{
    RanimScene,
    anims::transform::TransformAnim,
    color::palettes::manim,
    core::{Extract, animation::AnimationCell, core_item::CoreItem, timeline::Timeline},
    glam::{DVec3, dvec3, ivec3},
    items::vitem::{
        VItem,
        geometry::{Rectangle, Square},
    },
    prelude::*,
};

#[derive(Default, Clone, Debug)]
pub struct SingleTimeline {
    pub show_times: Vec<Range<f64>>,
}

impl SingleTimeline {
    pub fn max_time(&self) -> f64 {
        self.show_times
            .iter()
            .map(|t| t.end)
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))
            .unwrap_or(0.0)
    }
    pub fn min_time(&self) -> f64 {
        self.show_times
            .iter()
            .map(|t| t.start)
            .min_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))
            .unwrap_or(0.0)
    }
}

impl SingleTimeline {
    pub fn update_from(&mut self, scene: &RanimScene, timeline: &Timeline) {
        let animation_infos = timeline.get_animation_infos();
        self.show_times = animation_infos
            .iter()
            .map(|info| info.range.clone())
            .collect();
    }
}

#[derive(Default, Clone, Debug)]
pub struct VisualTimeline {
    pub timelines: Vec<SingleTimeline>,
    pub top_left: DVec3,
    pub width: f64,
    pub height: f64,
}

impl VisualTimeline {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_height(mut self, height: f64) -> Self {
        self.height = height;
        self
    }
    pub fn with_width(mut self, width: f64) -> Self {
        self.width = width;
        self
    }
}

impl VisualTimeline {
    pub fn update_inner_from(&mut self, scene: &RanimScene) {
        if self.timelines.len() < scene.timelines().len() {
            self.timelines
                .resize(scene.timelines().len(), Default::default());
        }
        self.timelines
            .iter_mut()
            .zip(scene.timelines())
            .for_each(|(t, timeline)| {
                t.update_from(scene, timeline);
            });
    }
    pub fn update_inner_transform_from(
        mut self,
        t_id: TimelineId,
        r: &mut RanimScene,
        src_scene: &RanimScene,
        transform_anim_modifier: impl Fn(AnimationCell<Vec<VItem>>) -> AnimationCell<Vec<VItem>>,
    ) {
        let prev = self.clone();
        let new = self.with(|x| x.update_inner_from(src_scene));

        r.timeline_mut(t_id).play(transform_anim_modifier(
            Vec::<VItem>::from(prev).transform_to(new.into()),
        ));
    }
    pub fn update_inner_transform(
        mut self,
        t_id: TimelineId,
        r: &mut RanimScene,
        update_fn: impl Fn(&mut Self),
        transform_anim_modifier: impl Fn(AnimationCell<Vec<VItem>>) -> AnimationCell<Vec<VItem>>,
    ) {
        let prev = self.clone();
        let new = self.with(update_fn);

        r.timeline_mut(t_id).play(transform_anim_modifier(
            Vec::<VItem>::from(prev).transform_to(new.into()),
        ));
    }
}

impl BoundingBox for VisualTimeline {
    fn get_bounding_box(&self) -> [DVec3; 3] {
        let start = self.top_left;
        let end = self.top_left + dvec3(self.width, self.height, 0.0);
        [start, (start + end) / 2.0, end]
    }
}

impl Shift for VisualTimeline {
    fn shift(&mut self, shift: DVec3) -> &mut Self {
        self.top_left.shift(shift);
        self
    }
}

impl VisualTimeline {
    pub fn max_time(&self) -> f64 {
        self.timelines
            .iter()
            .map(|t| t.max_time())
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))
            .unwrap_or(0.0)
    }
}

impl From<VisualTimeline> for Vec<VItem> {
    fn from(value: VisualTimeline) -> Self {
        let timeline_height = value.height / value.timelines.len() as f64;
        let padded_timeline_height = timeline_height * 0.9;
        let x_per_t = if value.max_time() == 0.0 {
            0.0
        } else {
            value.width / value.max_time()
        };
        let timeline_rects = value
            .timelines
            .iter()
            .enumerate()
            .flat_map(|(i, t)| {
                let timeline_y = value.top_left.y + i as f64 * timeline_height;
                t.show_times.iter().map(move |show_time| {
                    let start_x = value.top_left.x + show_time.start * x_per_t;
                    let end_x = value.top_left.x + show_time.end * x_per_t;
                    Rectangle::new(end_x - start_x, padded_timeline_height).with(|x| {
                        x.put_anchor_on(
                            Anchor::Edge(ivec3(-1, -1, 0)),
                            dvec3(start_x, timeline_y, 0.0),
                        );
                        x.fill_rgba = manim::BLUE_C.with_alpha(0.5).into();
                        x.stroke_rgba = manim::BLUE_C.with_alpha(0.8).into();
                    })
                })
            })
            .map(VItem::from);
        // let mut res = timeline_rects.collect::<Group<_>>();
        // if res.is_empty() {
        //     res.push(VItem::empty());
        // }
        // res
        timeline_rects.collect()
    }
}

impl Extract for VisualTimeline {
    type Target = CoreItem;
    fn extract_into(&self, buf: &mut Vec<Self::Target>) {
        Vec::<VItem>::from(self.clone()).extract_into(buf);
    }
}
