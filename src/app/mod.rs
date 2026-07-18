use std::rc::Rc;

use gtk4::prelude::*;
use gtk4::{
    Application, ApplicationWindow, Box, DropDown, Label, Orientation, ScrolledWindow, Stack,
    StringList,
};

use crate::rogauracore::client::ExecutionResult;
use crate::ui::pages;

pub fn run() {
    let app = Application::builder()
        .application_id("com.github.ayakase.rogauracoregui")
        .build();

    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("ROG Aura Core")
        .default_width(920)
        .default_height(720)
        .build();

    let root = Box::new(Orientation::Vertical, 16);
    root.set_margin_top(16);
    root.set_margin_bottom(16);
    root.set_margin_start(16);
    root.set_margin_end(16);

    let intro = Label::new(Some(
        "Control the installed rogauracore CLI without memorizing every command.",
    ));
    intro.set_wrap(true);
    intro.set_xalign(0.0);
    root.append(&intro);

    let status = Label::new(Some("Pick a mode, choose values, and apply a command."));
    status.set_wrap(true);
    status.set_xalign(0.0);
    status.add_css_class("dim-label");

    let update_status: Rc<dyn Fn(ExecutionResult)> = {
        let status = status.clone();
        Rc::new(move |result: ExecutionResult| {
            status.set_label(&result.summary());
        })
    };

    let pages = pages::build_pages(update_status.clone());
    let page_titles: Vec<&str> = pages.iter().map(|page| page.title.as_str()).collect();

    let command_picker = DropDown::new(
        Some(StringList::new(&page_titles)),
        None::<gtk4::Expression>,
    );
    command_picker.set_selected(0);
    root.append(&labeled_header("Command", &command_picker));

    let stack = Stack::new();
    stack.set_hexpand(true);
    stack.set_vexpand(true);
    stack.set_transition_type(gtk4::StackTransitionType::Crossfade);

    for page in pages {
        stack.add_titled(&page.content, Some(&page.id), &page.title);
    }

    let stack_for_picker = stack.clone();
    command_picker.connect_selected_notify(move |picker| {
        if let Some(item) = picker.selected_item()
            && let Ok(string_object) = item.downcast::<gtk4::StringObject>()
        {
            stack_for_picker.set_visible_child_name(string_object.string().as_str());
        }
    });

    stack.set_visible_child_name("Single Static");

    let scroll = ScrolledWindow::builder()
        .hscrollbar_policy(gtk4::PolicyType::Never)
        .min_content_height(500)
        .child(&stack)
        .build();

    root.append(&scroll);
    root.append(&status);

    window.set_child(Some(&root));
    window.present();
}

fn labeled_header(label: &str, widget: &impl IsA<gtk4::Widget>) -> Box {
    let row = Box::new(Orientation::Horizontal, 12);

    let text = Label::new(Some(label));
    text.set_xalign(0.0);
    text.set_width_chars(10);
    text.add_css_class("heading");

    row.append(&text);
    row.append(widget);
    row
}
