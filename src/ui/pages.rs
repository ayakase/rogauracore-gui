use std::rc::Rc;

use gtk4::prelude::*;
use gtk4::{Box, CheckButton, ColorButton, Label, Orientation, PositionType, Scale};

use crate::rogauracore::client::{ExecutionResult, run};
use crate::rogauracore::command::{AuraCommand, Brightness};
use crate::ui::widgets;

pub struct CommandPage {
    pub id: String,
    pub title: String,
    pub nav_label: String,
    pub content: Box,
}

pub fn build_pages(on_result: Rc<dyn Fn(ExecutionResult)>) -> Vec<CommandPage> {
    vec![
        page("Single Static", single_static_page(on_result.clone())),
        page("Single Breathing", single_breathing_page(on_result.clone())),
        page("Single Pulsing", single_pulsing_page(on_result.clone())),
        page(
            "Single Colorcycle",
            single_colorcycle_page(on_result.clone()),
        ),
        page("Multi Static", multi_static_page(on_result.clone())),
        page("Multi Breathing", multi_breathing_page(on_result.clone())),
        page("Rainbow", rainbow_page(on_result.clone())),
    ]
}

pub fn build_brightness_section(on_result: Rc<dyn Fn(ExecutionResult)>) -> Box {
    let section = widgets::page_shell("Brightness", "Common setting.");

    let slider_row = Box::new(Orientation::Horizontal, 12);
    slider_row.add_css_class("form-row");

    let label = Label::new(Some("Level"));
    label.set_xalign(0.0);
    label.set_width_chars(14);

    let brightness = Scale::with_range(Orientation::Horizontal, 0.0, 3.0, 1.0);
    brightness.set_draw_value(false);
    brightness.set_hexpand(true);
    brightness.set_value(2.0);
    brightness.set_round_digits(0);
    brightness.set_restrict_to_fill_level(false);
    brightness.set_increments(1.0, 1.0);
    brightness.set_digits(0);
    brightness.add_mark(0.0, PositionType::Bottom, Some("Off"));
    brightness.add_mark(1.0, PositionType::Bottom, Some("Low"));
    brightness.add_mark(2.0, PositionType::Bottom, Some("Med"));
    brightness.add_mark(3.0, PositionType::Bottom, Some("High"));

    slider_row.append(&label);
    slider_row.append(&brightness);
    section.append(&slider_row);

    brightness.connect_change_value(move |slider, _, value| {
        let snapped = value.round().clamp(0.0, 3.0);
        if (slider.value() - snapped).abs() > f64::EPSILON {
            slider.set_value(snapped);
        }

        let command = AuraCommand::Brightness {
            brightness: brightness_from_scale(snapped),
        };
        on_result(run(command));

        glib::Propagation::Stop
    });

    section
}

fn brightness_from_scale(value: f64) -> Brightness {
    match value.round() as i32 {
        0 => Brightness::Off,
        1 => Brightness::Low,
        2 => Brightness::Medium,
        _ => Brightness::High,
    }
}

fn page(title: &str, content: Box) -> CommandPage {
    CommandPage {
        id: slug(title),
        title: title.into(),
        nav_label: nav_label(title),
        content,
    }
}

fn slug(title: &str) -> String {
    title.to_lowercase().replace(' ', "-")
}

fn nav_label(title: &str) -> String {
    match title {
        "Single Static" => "Static".into(),
        "Single Breathing" => "Breathing".into(),
        "Single Pulsing" => "Pulsing".into(),
        "Single Colorcycle" => "Cycle".into(),
        "Multi Static" => "4-Zone Static".into(),
        "Multi Breathing" => "4-Zone Breath".into(),
        "Rainbow" => "Rainbow".into(),
        "Brightness" => "Brightness".into(),
        _ => title.into(),
    }
}

fn single_static_page(on_result: Rc<dyn Fn(ExecutionResult)>) -> Box {
    let page = widgets::page_shell("Single Static", "One solid color.");

    let color = widgets::color_button(1.0, 0.0, 0.0);
    page.append(&widgets::labeled_row("Color", &color));

    let apply = widgets::apply_button();
    apply.connect_clicked(move |_| {
        let command = AuraCommand::SingleStatic {
            color: widgets::rgba_to_hex(&color),
        };
        on_result(run(command));
    });
    page.append(&apply);

    page
}

fn single_breathing_page(on_result: Rc<dyn Fn(ExecutionResult)>) -> Box {
    let page = widgets::page_shell("Single Breathing", "One or two colors.");

    let color1 = widgets::color_button(0.0, 1.0, 1.0);
    let color2 = widgets::color_button(1.0, 0.0, 1.0);
    let use_color2 = widgets::optional_check("Color 2");
    let use_speed = widgets::optional_check("Speed");
    let speed = widgets::speed_dropdown();
    speed.set_sensitive(false);
    color2.set_sensitive(false);

    toggle_sensitive(&use_color2, &color2);
    toggle_sensitive(&use_speed, &speed);

    page.append(&widgets::labeled_row("Primary color", &color1));
    page.append(&use_color2);
    page.append(&widgets::labeled_row("Secondary color", &color2));
    page.append(&use_speed);
    page.append(&widgets::labeled_row("Speed", &speed));

    let apply = widgets::apply_button();
    apply.connect_clicked(move |_| {
        let command = AuraCommand::SingleBreathing {
            color1: widgets::rgba_to_hex(&color1),
            color2: use_color2
                .is_active()
                .then(|| widgets::rgba_to_hex(&color2)),
            speed: use_speed
                .is_active()
                .then(|| widgets::speed_from_dropdown(&speed)),
        };
        on_result(run(command));
    });
    page.append(&apply);

    page
}

fn single_pulsing_page(on_result: Rc<dyn Fn(ExecutionResult)>) -> Box {
    let page = widgets::page_shell("Single Pulsing", "Pulse one color.");

    let color = widgets::color_button(0.0, 0.75, 0.0);
    let speed = widgets::speed_dropdown();

    page.append(&widgets::labeled_row("Color", &color));
    page.append(&widgets::labeled_row("Speed", &speed));

    let apply = widgets::apply_button();
    apply.connect_clicked(move |_| {
        let command = AuraCommand::SinglePulsing {
            color: widgets::rgba_to_hex(&color),
            speed: widgets::speed_from_dropdown(&speed),
        };
        on_result(run(command));
    });
    page.append(&apply);

    page
}

fn single_colorcycle_page(on_result: Rc<dyn Fn(ExecutionResult)>) -> Box {
    let page = widgets::page_shell("Single Colorcycle", "Cycle all colors.");

    let speed = widgets::speed_dropdown();
    page.append(&widgets::labeled_row("Speed", &speed));

    let apply = widgets::apply_button();
    apply.connect_clicked(move |_| {
        let command = AuraCommand::SingleColorcycle {
            speed: widgets::speed_from_dropdown(&speed),
        };
        on_result(run(command));
    });
    page.append(&apply);

    page
}

fn multi_static_page(on_result: Rc<dyn Fn(ExecutionResult)>) -> Box {
    let page = widgets::page_shell("Multi Static", "Four fixed zones.");

    let colors = zone_colors();
    append_zone_rows(&page, &colors);

    let apply = widgets::apply_button();
    apply.connect_clicked(move |_| {
        let command = AuraCommand::MultiStatic {
            colors: gather_four_colors(&colors),
        };
        on_result(run(command));
    });
    page.append(&apply);

    page
}

fn multi_breathing_page(on_result: Rc<dyn Fn(ExecutionResult)>) -> Box {
    let page = widgets::page_shell("Multi Breathing", "Four zones with breathing.");

    let colors = zone_colors();
    let speed = widgets::speed_dropdown();

    append_zone_rows(&page, &colors);
    page.append(&widgets::labeled_row("Speed", &speed));

    let apply = widgets::apply_button();
    apply.connect_clicked(move |_| {
        let command = AuraCommand::MultiBreathing {
            colors: gather_four_colors(&colors),
            speed: widgets::speed_from_dropdown(&speed),
        };
        on_result(run(command));
    });
    page.append(&apply);

    page
}

fn rainbow_page(on_result: Rc<dyn Fn(ExecutionResult)>) -> Box {
    let page = widgets::page_shell("Rainbow", "Built-in rainbow mode.");

    let use_speed = widgets::optional_check("Speed");
    let speed = widgets::speed_dropdown();
    speed.set_sensitive(false);
    toggle_sensitive(&use_speed, &speed);

    page.append(&use_speed);
    page.append(&widgets::labeled_row("Speed", &speed));

    let apply = widgets::apply_button();
    apply.connect_clicked(move |_| {
        let command = AuraCommand::Rainbow {
            speed: use_speed
                .is_active()
                .then(|| widgets::speed_from_dropdown(&speed)),
        };
        on_result(run(command));
    });
    page.append(&apply);

    page
}

fn zone_colors() -> [ColorButton; 4] {
    [
        widgets::color_button(1.0, 0.0, 0.0),
        widgets::color_button(1.0, 0.5, 0.0),
        widgets::color_button(0.0, 0.5, 1.0),
        widgets::color_button(0.5, 0.0, 1.0),
    ]
}

fn gather_four_colors(colors: &[ColorButton; 4]) -> [String; 4] {
    [
        widgets::rgba_to_hex(&colors[0]),
        widgets::rgba_to_hex(&colors[1]),
        widgets::rgba_to_hex(&colors[2]),
        widgets::rgba_to_hex(&colors[3]),
    ]
}

fn append_zone_rows(page: &Box, colors: &[ColorButton; 4]) {
    for (index, color) in colors.iter().enumerate() {
        let label = format!("Zone {}", index + 1);
        page.append(&widgets::labeled_row(&label, color));
    }
}

fn toggle_sensitive(toggle: &CheckButton, widget: &impl IsA<gtk4::Widget>) {
    let widget = widget.clone().upcast::<gtk4::Widget>();
    toggle.connect_toggled(move |button| {
        widget.set_sensitive(button.is_active());
    });
}
