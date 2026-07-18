use std::rc::Rc;

use gtk4::gdk::Display;
use gtk4::prelude::*;
use gtk4::{
    Align, Application, ApplicationWindow, Box, DropDown, Frame, Image, Label, Orientation,
    ScrolledWindow, Settings, Stack, StringList,
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
    install_css();
    prefer_dark_theme();

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Aura")
        .default_width(780)
        .default_height(560)
        .build();

    let root = Box::new(Orientation::Vertical, 14);
    root.set_margin_top(20);
    root.set_margin_bottom(20);
    root.set_margin_start(20);
    root.set_margin_end(20);

    root.append(&hero_block());

    let status = Label::new(Some("Ready."));
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
    let mode_names: Vec<String> = pages.iter().map(|page| page.nav_label.clone()).collect();
    let mode_names_refs: Vec<&str> = mode_names.iter().map(String::as_str).collect();
    let page_ids: Rc<Vec<String>> = Rc::new(pages.iter().map(|page| page.id.clone()).collect());

    let stack = Stack::new();
    stack.set_hexpand(true);
    stack.set_transition_type(gtk4::StackTransitionType::Crossfade);

    for page in &pages {
        stack.add_titled(&page.content, Some(&page.id), &page.title);
    }
    stack.set_visible_child_name("single-static");

    let mode_picker = DropDown::new(
        Some(StringList::new(&mode_names_refs)),
        None::<gtk4::Expression>,
    );
    mode_picker.set_selected(0);

    let stack_for_picker = stack.clone();
    let ids_for_picker = page_ids.clone();
    mode_picker.connect_selected_notify(move |picker| {
        let selected = picker.selected() as usize;
        if let Some(page_id) = ids_for_picker.get(selected) {
            stack_for_picker.set_visible_child_name(page_id);
        }
    });

    let switcher_box = Box::new(Orientation::Vertical, 10);
    switcher_box.add_css_class("surface");

    let control_row = Box::new(Orientation::Horizontal, 12);
    let mode_title = Label::new(Some("Mode"));
    mode_title.set_xalign(0.0);
    mode_title.add_css_class("section-label");
    control_row.append(&mode_title);
    control_row.append(&mode_picker);
    switcher_box.append(&control_row);

    let switcher_frame = Frame::new(None);
    switcher_frame.set_child(Some(&switcher_box));

    let settings_box = Box::new(Orientation::Vertical, 0);
    settings_box.add_css_class("surface");
    settings_box.append(&pages::build_brightness_section(update_status));

    let settings_frame = Frame::new(None);
    settings_frame.set_child(Some(&settings_box));

    let content_box = Box::new(Orientation::Vertical, 0);
    content_box.add_css_class("surface");
    content_box.set_size_request(640, -1);

    let scroll = ScrolledWindow::builder()
        .hscrollbar_policy(gtk4::PolicyType::Never)
        .min_content_height(300)
        .child(&stack)
        .build();
    content_box.append(&scroll);

    let content_frame = Frame::new(None);
    content_frame.set_child(Some(&content_box));

    root.append(&switcher_frame);
    root.append(&settings_frame);
    root.append(&content_frame);
    root.append(&status);

    window.set_child(Some(&root));
    window.present();
}
fn hero_block() -> Box {
    let hero = Box::new(Orientation::Horizontal, 8);
    hero.set_valign(Align::Center);

    let logo_wrap = Box::new(Orientation::Horizontal, 0);
    logo_wrap.set_halign(Align::Start);
    logo_wrap.set_hexpand(false);

    let logo = Image::from_file(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/assets/asus-rog-1-logo-svgrepo-com.svg"
    ));
    logo.set_pixel_size(64);
    logo.set_hexpand(false);
    logo.set_vexpand(false);
    logo.add_css_class("app-logo");
    logo_wrap.append(&logo);

    let text_block = Box::new(Orientation::Vertical, 4);

    let title = Label::new(Some("Aura"));
    title.set_xalign(0.0);
    title.add_css_class("hero-title");

    let subtitle = Label::new(Some("ROG keyboard lighting"));
    subtitle.set_xalign(0.0);
    subtitle.add_css_class("hero-subtitle");

    text_block.append(&title);
    text_block.append(&subtitle);

    hero.append(&logo_wrap);
    hero.append(&text_block);
    hero
}

fn prefer_dark_theme() {
    if let Some(settings) = Settings::default() {
        settings.set_gtk_application_prefer_dark_theme(true);
    }
}

fn install_css() {
    let provider = gtk4::CssProvider::new();
    provider.load_from_data(
        "
        .hero-title {
            font-size: 30px;
            font-weight: 700;
        }

        .hero-subtitle {
            opacity: 0.7;
        }

        .app-logo {
            opacity: 0.9;
        }

        .surface {
            padding: 16px;
        }

        .section-label {
            font-size: 12px;
            font-weight: 700;
            letter-spacing: 0.08em;
            text-transform: uppercase;
            opacity: 0.7;
        }

        .page-title {
            font-size: 20px;
            font-weight: 700;
        }

        .page-subtitle {
            opacity: 0.72;
        }
        ",
    );

    if let Some(display) = Display::default() {
        gtk4::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
}
