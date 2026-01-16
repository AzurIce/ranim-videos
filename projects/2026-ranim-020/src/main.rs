fn main() {
    #[cfg(all(feature = "render", not(feature = "preview")))]
    ranim::cmd::render_scene(ranim_020::code_structure_scene);
    #[cfg(all(feature = "preview", not(feature = "render")))]
    ranim::cmd::preview_scene(ranim_020::code_structure_scene);
}
