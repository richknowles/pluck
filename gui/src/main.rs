use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};

fn main() {
    let app = Application::new(
        Some("com.richknowles.pluck"),
        Default::default(),
    );

    app.connect_activate(|app| {
        let win = ApplicationWindow::builder()
            .application(app)
            .title("Pluck")
            .default_width(400)
            .default_height(300)
            .build();

        win.show();
    });

    app.run();
}
