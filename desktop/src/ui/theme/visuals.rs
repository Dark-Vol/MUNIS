use egui::{
    Color32,
    CornerRadius,
    Stroke,
    Style,
    Visuals,
};

use super::colors::*;

pub fn create_theme() -> Visuals {
    let mut visuals = Visuals::dark();

    visuals.panel_fill = BACKGROUND;
    visuals.window_fill = SURFACE;

    visuals.widgets.noninteractive.bg_fill = SURFACE;
    visuals.widgets.noninteractive.fg_stroke.color =
        TEXT_PRIMARY;

    visuals.widgets.inactive.bg_fill = SURFACE;
    visuals.widgets.inactive.fg_stroke.color =
        TEXT_PRIMARY;

    visuals.widgets.hovered.bg_fill = SURFACE_HOVER;
    visuals.widgets.hovered.fg_stroke.color =
        TEXT_PRIMARY;

    visuals.widgets.active.bg_fill = ACCENT;
    visuals.widgets.active.fg_stroke.color =
        Color32::WHITE;

    visuals.widgets.inactive.corner_radius =
        CornerRadius::same(6);

    visuals.widgets.hovered.corner_radius =
        CornerRadius::same(6);

    visuals.widgets.active.corner_radius =
        CornerRadius::same(6);

    visuals.widgets.noninteractive.bg_stroke =
        Stroke::new(1.0, BORDER);

    visuals
}