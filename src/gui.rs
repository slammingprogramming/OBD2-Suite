use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button};
use crate::obd_protocol::OBDInterface;

pub fn launch_gui(obd_interface: &OBDInterface) {
    let app = Application::new(Some("com.example.obd2"), Default::default());

    app.connect_activate(move |app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Universal OBD-II Debugging Suite")
            .default_width(350)
            .default_height(70)
            .build();

        let button = Button::with_label("Connect to Vehicle");
        button.connect_clicked(move |_| {
            println!("Attempting to connect...");
            obd_interface.connect().expect("Failed to connect");
        });

        window.add(&button);
        window.show_all();
    });

    app.run();
}