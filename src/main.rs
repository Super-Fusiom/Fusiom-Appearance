use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};


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
    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Fusiom Appearance")
        .build();
    // Present window
    window.present();
}
