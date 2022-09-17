use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button};


const APP_ID: &str = "org.gtk_rs.fusiomappearance";

fn main() {
    // Create the application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to the activate signal of app
    app.connect_activate(build_ui);

    // Run the app
    app.run();
}

fn build_ui(app: &Application) {
    // Create a button with label and margins
    let button = Button::builder()
        .label("Super!")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // connect to the clicked signal of button
    button.connect_clicked(move |button| {
        // Set the labe to Fusiom after button has been clicked
        button.set_label("Fusiom!!!");
    });


    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Fusiom Appearance")
        .child(&button)
        .build();
    // Present window
    window.present();
}
