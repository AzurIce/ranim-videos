//! Note: total 6s
//! optimize font search: 9min -> 26s
//! cache unchanged cell: 26s -> 25s
use std::sync::{Arc, Mutex, OnceLock};

use itertools::Itertools;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use ranim::{
    color::palettes::manim,
    components::ScaleHint,
    glam::{dvec3, DVec3},
    items::{
        vitem::{geometry::Square, svg::SvgItem, typst::typst_svg, VItem}, Group
    },
    prelude::*,
    render::primitives::{vitem::VItemPrimitive, Extract}, timeline::TimelinesFunc,
};
use rayon::prelude::*;

fn rng() -> Arc<Mutex<ChaCha8Rng>> {
    static RNG: OnceLock<Arc<Mutex<ChaCha8Rng>>> = OnceLock::new();
    RNG.get_or_init(|| Arc::new(Mutex::new(ChaCha8Rng::seed_from_u64(0))))
        .clone()
}

#[scene]
struct DenoiseScene;

impl TimelineConstructor for DenoiseScene {
    fn construct(self, r: &mut RanimScene, _r_cam: TimelineId<CameraFrame>) {
        let (width, height) = (10, 10);

        let time_surface = TimeSurface::new(width, height);
        let r_time_surface = r.init_timeline(time_surface);

        let events = (0..640)
            .map(|_| {
                let rng = rng();
                let mut rng = rng.lock().unwrap();
                (
                    rng.random::<u32>() % 500,
                    rng.random::<u32>() % height as u32,
                    rng.random::<u32>() % width as u32,
                )
            })
            .sorted()
            .collect::<Vec<_>>();

        let total_secs = 6.0;
        let step = 0.2_f64.min(total_secs / events.len() as f64);
        for event in events {
            r.timeline_mut(&r_time_surface).update_with(|time_surface| {
                time_surface.accept(event.0 as usize, event.2 as usize, event.1 as usize);
            });
            r.timelines_mut().forward(step);
        }
    }
}

struct TimeSurfaceCell {
    start: DVec3,
    cell_size: f64,
    y: usize,
    x: usize,
    t: usize,
    real_t: usize,
    _need_update: Mutex<bool>,
    _cache: Mutex<Option<Vec<VItemPrimitive>>>,
}

impl Clone for TimeSurfaceCell {
    fn clone(&self) -> Self {
        Self {
            start: self.start.clone(),
            cell_size: self.cell_size.clone(),
            y: self.y.clone(),
            x: self.x.clone(),
            t: self.t.clone(),
            real_t: self.real_t.clone(),
            _need_update: Mutex::new(self._need_update.lock().unwrap().clone()),
            _cache: Mutex::new(self._cache.lock().unwrap().clone()),
        }
    }
}

impl TimeSurfaceCell {
    pub fn new(start: DVec3, cell_size: f64, y: usize, x: usize) -> Self {
        Self {
            start,
            cell_size,
            y,
            x,
            t: 0,
            real_t: 0,
            _need_update: Mutex::new(true),
            _cache: Mutex::new(None),
        }
    }
    pub fn set_t(&mut self, t: usize) {
        self.t = t;
        *self._need_update.lock().unwrap() = true;
    }
    pub fn accept(&mut self, real_t: usize) {
        self.real_t = real_t;
        *self._need_update.lock().unwrap() = true;
    }
}

impl Extract for TimeSurfaceCell {
    type Target = Vec<VItemPrimitive>;
    fn extract(&self) -> Self::Target {
        let mut _cache = self._cache.lock().unwrap();
        let mut _need_update = self._need_update.lock().unwrap();
        // if let Some(cache) = _cache.as_ref() {
        //     if !*_need_update {
        //         return cache.clone();
        //     }
        // }
        let padding_ratio = 0.1;

        let square_size = self.cell_size * (1.0 - padding_ratio);
        let pos = self.start
            + self.y as f64 * DVec3::NEG_Y * self.cell_size
            + self.x as f64 * DVec3::X * self.cell_size;
        let text =
            SvgItem::new(typst_svg(format!("{}", self.real_t).as_str())).with(|text| {
                text.scale_to(ScaleHint::PorportionalY(square_size * 0.2))
                    .set_fill_color(manim::WHITE)
                    .put_center_on(pos);
            });
        let square = Square::new(square_size).with(|square| {
            square.put_center_on(pos);
        });
        let res = [VItem::from(square)]
            .into_iter()
            .chain(Group::<VItem>::from(text))
            .map(|item| item.extract())
            .collect::<Vec<_>>();
        *_cache = Some(res.clone());
        *_need_update = false;
        res
    }
}

#[allow(unused)]
#[derive(Clone)]
struct TimeSurface {
    width: usize,
    height: usize,
    cells: Vec<TimeSurfaceCell>,
}

impl TimeSurface {
    pub fn new(width: usize, height: usize) -> Self {
        let cell_size = 8.0 / height as f64;
        let surface_width = cell_size * width as f64;
        let start = dvec3(
            -surface_width / 2.0 + cell_size / 2.0,
            4.0 - cell_size / 2.0,
            0.0,
        );
        Self {
            width,
            height,
            cells: (0..height)
                .cartesian_product(0..width)
                .map(|(y, x)| TimeSurfaceCell::new(start, cell_size, y, x))
                .collect(),
        }
    }
    pub fn accept(&mut self, t: usize, x: usize, y: usize) {
        // let mut ts = Vec::with_capacity(8);
        // if y > 0 {
        //     ts.push(self.cells[(y - 1) * self.width + x].t);
        //     if x > 0 {
        //         ts.push(self.cells[(y - 1) * self.width + x - 1].t);
        //     }
        //     if x < self.width - 1 {
        //         ts.push(self.cells[(y - 1) * self.width + x + 1].t);
        //     }
        // }
        // if x > 0 {
        //     ts.push(self.cells[y * self.width + x - 1].t);
        // }
        // if x < self.width - 1 {
        //     ts.push(self.cells[y * self.width + x + 1].t);
        // }
        // if y < self.height - 1 {
        //     ts.push(self.cells[(y + 1) * self.width + x].t);
        //     if x > 0 {
        //         ts.push(self.cells[(y + 1) * self.width + x - 1].t);
        //     }
        //     if x < self.width - 1 {
        //         ts.push(self.cells[(y + 1) * self.width + x + 1].t);
        //     }
        // }
        // self.cells[y * self.width + x].set_t(t);
        // if ts.iter().any(|_t| _t.abs_diff(t) < 10) {
        self.cells[y * self.width + x].accept(t);
        // }
    }
}

impl Extract for TimeSurface {
    type Target = Vec<VItemPrimitive>;
    fn extract(&self) -> Self::Target {
        let t = self.cells[0].t;
        let (min_t, max_t) = self
            .cells
            .iter()
            .map(|cell| cell.real_t)
            .fold((t, t), |acc, x| (acc.0.min(x), acc.1.max(x)));
        self.cells
            .par_iter()
            .flat_map(|cell| {
                let alpha = (cell.real_t - min_t) as f32 / (max_t - min_t) as f32;
                cell.extract().with(|primitive| {
                    primitive[0].set_fill_color(manim::BLUE_C.with_alpha(alpha * 0.7));
                })
            })
            .collect()
    }
}

fn main() {
    render_scene(DenoiseScene, &AppOptions::default());
}
