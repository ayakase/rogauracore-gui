use std::rc::Rc;

use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Box, Label, Notebook, Orientation, ScrolledWindow};

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

    let notebook = Notebook::new();
    notebook.set_hexpand(true);
    notebook.set_vexpand(true);

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

    for (title, page) in pages::build_pages(update_status) {
        let tab = Label::new(Some(&title));
        notebook.append_page(&page, Some(&tab));
    }

    let scroll = ScrolledWindow::builder()
        .hscrollbar_policy(gtk4::PolicyType::Never)
        .min_content_height(500)
        .child(&notebook)
        .build();

    root.append(&scroll);
    root.append(&status);

    window.set_child(Some(&root));
    window.present();
}
