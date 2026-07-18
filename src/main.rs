mod commands;

use gtk4::prelude::*;
use gtk4::{
    Application, ApplicationWindow, Box, Button, ColorChooserDialog, Orientation,
};

fn main() {
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
        .default_width(350)
        .default_height(120)
        .build();

    let vbox = Box::new(Orientation::Vertical, 12);

    let pick = Button::with_label("Pick Color");

    let parent = window.clone();

    pick.connect_clicked(move |_| {
        let dialog = ColorChooserDialog::new(Some("Choose Color"), Some(&parent));

        dialog.connect_response(|dialog, response| {
            if response == gtk4::ResponseType::Ok {
                let color = dialog.rgba();

                let r = (color.red() * 255.0) as u8;
                let g = (color.green() * 255.0) as u8;
                let b = (color.blue() * 255.0) as u8;

                println!("#{:02X}{:02X}{:02X}", r, g, b);

                commands::set_color(r, g, b);
            }

            dialog.close();
        });

        dialog.present();
    });

    vbox.append(&pick);

    window.set_child(Some(&vbox));
    window.present();
}

