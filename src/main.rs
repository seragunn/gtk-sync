use gtk::prelude::*;
use gtk::*;

const APP_WIDTH: i32 = 600;
const APP_HEIGHT: i32 = 400;

fn main() {
    let application = gtk::Application::new(Some("org.gtk-sync.app"), Default::default());
    application.connect_activate(build_ui);
    application.run();
}

fn build_ui(app: &Application) {
    // Window
    let window = gtk::ApplicationWindow::builder()
        .application(app)
        .title("Gtk Sync")
        .build();

    window.set_size_request(APP_WIDTH, APP_HEIGHT);

    window.present();
}
