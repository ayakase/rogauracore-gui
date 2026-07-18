use std::rc::Rc;

use gtk4::prelude::*;
use gtk4::{Box, CheckButton, ColorButton, Label};

use crate::rogauracore::client::{ExecutionResult, run};
use crate::rogauracore::command::AuraCommand;
use crate::ui::widgets;

pub struct CommandPage {
    pub id: String,
    pub title: String,
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
        page("Brightness", brightness_page(on_result)),
    ]
}

fn page(title: &str, content: Box) -> CommandPage {
    CommandPage {
        id: title.into(),
        title: title.into(),
        content,
    }
}

fn single_static_page(on_result: Rc<dyn Fn(ExecutionResult)>) -> Box {
    let page = widgets::page_shell(
        "Single Static",
        "Set the whole keyboard to one solid color.",
    );

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
    let page = widgets::page_shell(
        "Single Breathing",
        "Fade a single color in and out, or alternate between two colors. The speed argument remains optional just like the CLI.",
    );

    let color1 = widgets::color_button(0.0, 1.0, 1.0);
    let color2 = widgets::color_button(1.0, 0.0, 1.0);
    let use_color2 = widgets::optional_check("Use second color");
    let use_speed = widgets::optional_check("Set speed");
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
    let page = widgets::page_shell("Single Pulsing", "Pulse one color at a required speed.");

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
    let page = widgets::page_shell(
        "Single Colorcycle",
        "Cycle through colors across the whole keyboard.",
    );

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
    let page = widgets::page_shell(
        "Multi Static",
        "Set four separate zones to four fixed colors.",
    );

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
    let page = widgets::page_shell(
        "Multi Breathing",
        "Fade four keyboard zones using four colors at a shared speed.",
    );

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
    let page = widgets::page_shell(
        "Rainbow",
        "Run the built-in rainbow mode, optionally overriding its speed.",
    );

    let use_speed = widgets::optional_check("Set speed");
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

fn brightness_page(on_result: Rc<dyn Fn(ExecutionResult)>) -> Box {
    let page = widgets::page_shell(
        "Brightness",
        "Change keyboard brightness without changing the current effect.",
    );

    let brightness = widgets::brightness_dropdown();
    let note = Label::new(Some("CLI values are off, low, medium, and high."));
    note.set_xalign(0.0);

    page.append(&widgets::labeled_row("Brightness", &brightness));
    page.append(&note);

    let apply = widgets::apply_button();
    apply.connect_clicked(move |_| {
        let command = AuraCommand::Brightness {
            brightness: widgets::brightness_from_dropdown(&brightness),
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
