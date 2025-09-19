use std::sync::LazyLock;

use include_lines::include_lines;

pub mod hello_ranim_outer;
pub mod hello_ranim_outer_output_1;
pub mod hello_ranim_outer_output_2;
pub mod hello_ranim_square_insert;
pub mod hello_ranim_square_anim;
pub mod hello_ranim_square_time_mark;
pub mod hello_ranim;
pub mod hello_ranim_chained_0;
pub mod hello_ranim_chained_1;
pub mod hello_ranim_with_0;
pub mod hello_ranim_with_1;


pub static CODE_SLIDES: LazyLock<[(String, usize); 11]> = LazyLock::new(|| {
    let strip_lines = 13;
    let build_code_slide = |lines: &[&str]| (lines.join("\n"), lines.len());

    [
        build_code_slide(
            &include_lines!("projects/2025-09-15-ranim-014/src/code_structure/hello_ranim_outer.rs")
                [strip_lines..],
        ),
        build_code_slide(
            &include_lines!("projects/2025-09-15-ranim-014/src/code_structure/hello_ranim_outer_output_1.rs")
                [strip_lines..],
        ),
        build_code_slide(
            &include_lines!("projects/2025-09-15-ranim-014/src/code_structure/hello_ranim_outer_output_2.rs")
                [strip_lines..],
        ),
        build_code_slide(
            &include_lines!("projects/2025-09-15-ranim-014/src/code_structure/hello_ranim_square_insert.rs")
                [strip_lines..],
        ),
        build_code_slide(
            &include_lines!("projects/2025-09-15-ranim-014/src/code_structure/hello_ranim_square_anim.rs")
                [strip_lines..],
        ),
        build_code_slide(
            &include_lines!("projects/2025-09-15-ranim-014/src/code_structure/hello_ranim_square_time_mark.rs")
                [strip_lines..],
        ),
        build_code_slide(
            &include_lines!("projects/2025-09-15-ranim-014/src/code_structure/hello_ranim.rs")
                [strip_lines..],
        ),
        build_code_slide(
            &include_lines!(
                "projects/2025-09-15-ranim-014/src/code_structure/hello_ranim_chained_0.rs"
            )[strip_lines..],
        ),
        build_code_slide(
            &include_lines!(
                "projects/2025-09-15-ranim-014/src/code_structure/hello_ranim_chained_1.rs"
            )[strip_lines..],
        ),
        build_code_slide(
            &include_lines!(
                "projects/2025-09-15-ranim-014/src/code_structure/hello_ranim_with_0.rs"
            )[strip_lines..],
        ),
        build_code_slide(
            &include_lines!(
                "projects/2025-09-15-ranim-014/src/code_structure/hello_ranim_with_1.rs"
            )[strip_lines..],
        ),
    ]
});
