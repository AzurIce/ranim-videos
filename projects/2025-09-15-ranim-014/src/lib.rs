mod hello_ranim;

use include_lines::include_lines;
use ranim::{
    components::ScaleHint,
    items::vitem::{svg::SvgItem, typst::typst_svg},
    prelude::*,
    utils::typst::compile_typst_code,
};

#[scene]
pub fn code_structure(r: &mut RanimScene) {
    let _r_cam = r.insert_and_show(CameraFrame::default());
    let code = include_lines!("projects/2025-09-15-ranim-014/src/hello_ranim.rs");
    let code = code[4..].join("\n");

    let code = format!(
        r#"```rust
    {code}
    ```"#
    );

    let svg = SvgItem::new(typst_svg(&code));

    // let svg = compile_typst_code(&code);
    // let svg = SvgItem::new(svg);
    let r_code = r.insert_and_show(svg.with(|x| {
        x.scale_to(ScaleHint::PorportionalY(8.0));
    }));
}
