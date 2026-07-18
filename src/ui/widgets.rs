use gtk4::gdk::RGBA;
use gtk4::prelude::*;
use gtk4::{
    Align, Box, Button, CheckButton, ColorButton, DropDown, Label, Orientation, StringList,
};

use crate::rogauracore::command::{Brightness, Speed};

pub fn page_shell(title: &str, description: &str) -> Box {
    let page = Box::new(Orientation::Vertical, 12);
    page.set_margin_top(16);
    page.set_margin_bottom(16);
    page.set_margin_start(16);
    page.set_margin_end(16);

    let heading = Label::new(Some(title));
    heading.set_xalign(0.0);
    heading.add_css_class("title-3");

    let summary = Label::new(Some(description));
    summary.set_xalign(0.0);
    summary.set_wrap(true);

    page.append(&heading);
    page.append(&summary);
    page
}

pub fn labeled_row(label: &str, widget: &impl IsA<gtk4::Widget>) -> Box {
    let row = Box::new(Orientation::Horizontal, 12);
    row.set_halign(Align::Fill);

    let text = Label::new(Some(label));
    text.set_xalign(0.0);
    text.set_width_chars(16);

    row.append(&text);
    row.append(widget);
    row
}

pub fn apply_button() -> Button {
    let button = Button::with_label("Apply");
    button.add_css_class("suggested-action");
    button
}

pub fn color_button(red: f32, green: f32, blue: f32) -> ColorButton {
    let button = ColorButton::new();
    button.set_rgba(&RGBA::new(red, green, blue, 1.0));
    button
}

pub fn rgba_to_hex(button: &ColorButton) -> String {
    let color = button.rgba();
    let r = (color.red() * 255.0).round() as u8;
    let g = (color.green() * 255.0).round() as u8;
    let b = (color.blue() * 255.0).round() as u8;
    format!("{:02x}{:02x}{:02x}", r, g, b)
}

pub fn speed_dropdown() -> DropDown {
    let options: Vec<&str> = Speed::ALL.iter().map(|speed| speed.label()).collect();
    dropdown(&options)
}

pub fn brightness_dropdown() -> DropDown {
    let options: Vec<&str> = Brightness::ALL
        .iter()
        .map(|brightness| brightness.label())
        .collect();
    dropdown(&options)
}

pub fn speed_from_dropdown(dropdown: &DropDown) -> Speed {
    Speed::ALL[dropdown.selected() as usize]
}

pub fn brightness_from_dropdown(dropdown: &DropDown) -> Brightness {
    Brightness::ALL[dropdown.selected() as usize]
}

pub fn optional_check(label: &str) -> CheckButton {
    CheckButton::with_label(label)
}

fn dropdown(options: &[&str]) -> DropDown {
    let list = StringList::new(options);
    let dropdown = DropDown::new(Some(list), None::<gtk4::Expression>);
    dropdown.set_selected(0);
    dropdown
}
