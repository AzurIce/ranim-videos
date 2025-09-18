mod hello_ranim;
mod hello_ranim_chained;

use include_lines::include_lines;
use ranim::{
    anims::transform::TransformAnim,
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

    // println!("{}", a.vpoints.len());
    // println!("{}", b.vpoints.len());
    // a.align_with(&mut b);
    // println!("{}", a.vpoints.len());
    // println!("{}", b.vpoints.len());

    // let mut a = a.vpoints;
    // let mut b = b.vpoints;
    // // println!("{} {:?}", a.len(), a);
    // // println!("{} {:?}", b.len(), b);
    // a.align_with(&mut b);
    // // println!("{} {:?}", a.len(), a);
    // // println!("{} {:?}", b.len(), b);
}

#[scene(clear_color = "#ffffffff")]
pub fn code_structure(r: &mut RanimScene) {
    let padded_height = 8.0 * 0.9;

    let _r_cam = r.insert_and_show(CameraFrame::default());
    let code = include_lines!("projects/2025-09-15-ranim-014/src/hello_ranim.rs");
    let lines = code.len() - 4 + 1;
    let code = code[14..].join("\n");

    // let svg = compile_typst_code(&code);
    // let svg = SvgItem::new(svg);
    let r_code = r.insert_and_show(
        TypstText::new_multiline_code(&code, Some("rust")).with(|x| {
            x.scale_to(ScaleHint::PorportionalY(padded_height));
        }),
    );

    let code_chained = include_lines!("projects/2025-09-15-ranim-014/src/hello_ranim_chained.rs");
    let lines_chained = code_chained.len() - 4 + 1;
    let code_chained = code_chained[14..].join("\n");

    let code_chained = TypstText::new_multiline_code(&code_chained, Some("rust")).with(|x| {
        x.scale_to(ScaleHint::PorportionalY(
            padded_height * lines_chained as f64 / lines as f64,
        ));
    });
    // let r_code_chained = r.insert_and_show(code_chained.with(|x| {
    //     x.scale_to(ScaleHint::PorportionalY(8.0));
    // }));

    r.timeline_mut(&r_code)
        .play_with(|x| x.transform_to(code_chained).with_rate_func(smooth));
}
