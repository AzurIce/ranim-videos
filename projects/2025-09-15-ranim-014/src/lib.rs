#![feature(iter_intersperse)]
#![allow(unused)]
mod code_structure;

use ranim::{
    anims::{
        creation::WritingAnim, fading::FadingAnim, lagged::LaggedAnim, transform::TransformAnim,
    },
    color::palettes::manim,
    glam::DVec3,
    items::vitem::{
        Group, VItem,
        geometry::Circle,
        svg::SvgItem,
        typst::{TypstText, typst_svg},
    },
    prelude::*,
    utils::rate_functions::smooth,
};

#[scene(clear_color = "#ffffff")]
pub fn test_alpha(r: &mut RanimScene) {
    let _r_cam = r.insert_and_show(CameraFrame::default());

    let circles = (0..5)
        .map(|i: i32| {
            Circle::new(1.0).with(|x| {
                x.set_color(manim::RED_C.with_alpha(0.5))
                    .shift(2.0 * i as f64 / 5.0 * DVec3::X);
            })
        })
        .collect::<Group<_>>()
        .with(|x| {
            x.put_center_on(DVec3::ZERO);
        });
    let r_circles = r.insert_and_show(circles);
}

#[scene(clear_color = "#ffffff")]
pub fn test(r: &mut RanimScene) {
    let _r_cam = r.insert_and_show(CameraFrame::default());
    let a = Group::<VItem>::from(SvgItem::new(typst_svg("`fn main()`"))).with(|x| {
        x.scale_to(ScaleHint::PorportionalY(2.0)).shift(DVec3::Y);
    });
    let b = Group::<VItem>::from(SvgItem::new(typst_svg("`fn foo()`"))).with(|x| {
        x.scale_to(ScaleHint::PorportionalY(2.0)).shift(DVec3::Y);
    });
    let r_a = r.insert_and(a, |t| {
        t.play_with(|x| x.transform_to(b));
    });

    let a = TypstText::new_inline_code("fn main()").with(|x| {
        x.scale_to(ScaleHint::PorportionalY(2.0))
            .shift(DVec3::NEG_Y);
    });
    let b = TypstText::new_inline_code("fn foo()").with(|x| {
        x.scale_to(ScaleHint::PorportionalY(2.0))
            .shift(DVec3::NEG_Y);
    });
    let r_a = r.insert_and(a, |t| {
        t.play_with(|x| x.transform_to(b));
    });
}

const CODE_TRAIT_WITH: &'static str = r#"pub trait With {
    /// Mutating a value inplace
    fn with(mut self, f: impl Fn(&mut Self)) -> Self
    where
        Self: Sized,
    {
        f(&mut self);
        self
    }
}

impl<T> With for T {}
"#;

#[scene(clear_color = "#ffffffff")]
pub fn code_structure(r: &mut RanimScene) {
    let padded_height = 8.0 * 0.9;
    let padded_width = 16.0 / 9.0 * padded_height;

    let initial_lines = code_structure::CODE_SLIDES[0].1;
    let codes = code_structure::CODE_SLIDES
        .iter()
        .map(|(code, lines)| {
            TypstText::new_multiline_code(code, Some("rust")).with(|x| {
                x.scale_to_min(&[
                    ScaleHint::PorportionalY(padded_height),
                    ScaleHint::PorportionalX(padded_width),
                ])
                .set_stroke_width(0.002);
            })
        })
        .collect::<Vec<_>>();

    let _r_cam = r.insert_and_show(CameraFrame::default());

    let r_code = r.insert(Group::<VItem>::from(codes[0].clone()));
    // Chained API
    /* 这，是一个 ranim 动画场景的基础代码结构。很简单，就是一个函数。*/
    r.timeline_mut(&r_code)
        .play_with(|x| x.lagged(0.01, |x| x.write()).with_duration(4.0))
        .forward(5.0);
    let r_code = r.map(r_code, |_| codes[0].clone());

    /* 如果我想要为它添加视频输出，只需要添加一个 `#[output]` 宏。 */
    r.timeline_mut(&r_code)
        .play_with(|x| x.transform_to(codes[1].clone()).with_rate_func(smooth))
        .forward(3.0) // 当然，可以添加任意数量的输出，并带有不同的参数
        .play_with(|x| x.transform_to(codes[2].clone()).with_rate_func(smooth))
        .forward(2.0);

    r.timeline_mut(&r_code)
        .play_with(|x| x.transform_to(codes[1].clone()).with_rate_func(smooth));
    /* Ranim 中用于编码动画的主要 API 都在 `RanimScene` 中。 */
    r.timeline_mut(&r_code)
        .forward(3.0) // 你可以用它来创建一条物件时间线
        .play_with(|x| x.transform_to(codes[3].clone()).with_rate_func(smooth))
        .forward(2.0) // 用它来在时间线上编码动画
        .play_with(|x| x.transform_to(codes[4].clone()).with_rate_func(smooth))
        .forward(5.0) // 此外，如果你想让 Ranim 帮你截取一张图片，只需要向其中
        .play_with(|x| x.transform_to(codes[5].clone()).with_rate_func(smooth)) // 插入一个 `TimeMark`
        .forward(5.0) // 值得一提的是，Ranim 中的绝大部分 API 都是支持链式调用的，
        .play_with(|x| x.transform_to(codes[6].clone()).with_rate_func(smooth)) // 所以这些代码
        .forward(2.0) // 你可以这样写
        .play_with(|x| x.transform_to(codes[7].clone()).with_rate_func(smooth))
        .forward(0.5)
        .play_with(|x| x.transform_to(codes[8].clone()).with_rate_func(smooth))
        .forward(4.0) // 除此之外，为了支持将这种声明、修改、使用的代码写成一条语句
        .play_with(|x| {
            x.transform_to(
                TypstText::new_multiline_code(
                    r#"let mut square = Square::new(2.0);
square.set_color(manim::BLUE_C).set_fill_opacity(0.8);

let r_square = r.insert(square);"#,
                    Some("rust"),
                )
                .with(|x| {
                    x.scale_to_min(&[
                        ScaleHint::PorportionalY(padded_height),
                        ScaleHint::PorportionalX(padded_width),
                    ])
                    .set_stroke_width(0.002);
                }),
            )
            .with_rate_func(smooth)
        })
        .forward(2.0); // Ranim 提供了一个 `With` Trait

    let time = r.timelines().max_total_secs();
    let last_time = 3.0;
    let _r_code_with_trait = r.insert_and(
        TypstText::new_multiline_code(CODE_TRAIT_WITH, Some("rust")).with(|x| {
            x.scale_to_min(&[
                ScaleHint::PorportionalY(padded_height * 0.8),
                ScaleHint::PorportionalX(padded_width * 0.8),
            ]);
        }),
        |t| {
            t.forward_to(time)
                .play_with(|x| x.fade_in())
                .forward(last_time)
                .play_with(|x| x.fade_out());
        },
    );
    r.timeline_mut(&r_code).with_snapshot(|t, s| {
        t.play_with(|x| x.fade_out())
            .forward(last_time)
            .play(s.fade_in());
    });

    // With Trait
    r.timeline_mut(&r_code).forward(1.0);
    r.timeline_mut(&r_code) // 所以你也可以这样写代码
        .play_with(|x| x.transform_to(codes[8].clone()).with_rate_func(smooth))
        .forward(0.5)
        .play_with(|x| x.transform_to(codes[9].clone()).with_rate_func(smooth))
        .forward(0.5)
        .play_with(|x| x.transform_to(codes[10].clone()).with_rate_func(smooth))
        .forward(2.0);
}
