use std::sync::LazyLock;

use include_lines::include_lines;

pub mod p0_hello_ranim_outer;
pub mod p1_hello_ranim_outer_output_1;
pub mod p2_hello_ranim_outer_output_2;
pub mod p3_hello_ranim_square_insert;
pub mod p4_hello_ranim_square_anim;
pub mod p5_hello_ranim_square_time_mark;
pub mod p6_hello_ranim;
pub mod p7_hello_ranim_chained;
pub mod p8_hello_ranim_with_0;
pub mod p9_hello_ranim_with_1;

pub static CODE_SLIDES: LazyLock<[(String, usize); 10]> = LazyLock::new(|| {
    let strip_lines = 13;
    let build_code_slide = |lines: &[&str]| (lines.join("\n"), lines.len());

    [
        build_code_slide(
            &include_lines!("projects/2026-ranim-020/src/code_structure/p0_hello_ranim_outer.rs")
                [strip_lines..],
        ),
        build_code_slide(
            &include_lines!(
                "projects/2026-ranim-020/src/code_structure/p1_hello_ranim_outer_output_1.rs"
            )[strip_lines..],
        ),
        build_code_slide(
            &include_lines!(
                "projects/2026-ranim-020/src/code_structure/p2_hello_ranim_outer_output_2.rs"
            )[strip_lines..],
        ),
        build_code_slide(
            &include_lines!(
                "projects/2026-ranim-020/src/code_structure/p3_hello_ranim_square_insert.rs"
            )[strip_lines..],
        ),
        build_code_slide(
            &include_lines!(
                "projects/2026-ranim-020/src/code_structure/p4_hello_ranim_square_anim.rs"
            )[strip_lines..],
        ),
        build_code_slide(
            &include_lines!(
                "projects/2026-ranim-020/src/code_structure/p5_hello_ranim_square_time_mark.rs"
            )[strip_lines..],
        ),
        build_code_slide(
            &include_lines!("projects/2026-ranim-020/src/code_structure/p6_hello_ranim.rs")
                [strip_lines..],
        ),
        build_code_slide(
            &include_lines!("projects/2026-ranim-020/src/code_structure/p7_hello_ranim_chained.rs")
                [strip_lines..],
        ),
        build_code_slide(
            &include_lines!("projects/2026-ranim-020/src/code_structure/p8_hello_ranim_with_0.rs")
                [strip_lines..],
        ),
        build_code_slide(
            &include_lines!("projects/2026-ranim-020/src/code_structure/p9_hello_ranim_with_1.rs")
                [strip_lines..],
        ),
    ]
});
