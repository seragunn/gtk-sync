mod actions;

use gtk::prelude::*;
use gtk::*;

const APP_WIDTH: i32 = 600;
const APP_HEIGHT: i32 = 400;

fn main() {
    let application = Application::new(Some("org.gtk-sync.app"), Default::default());
    application.connect_activate(build_ui);
    application.run();
}

fn build_ui(app: &Application) {
    // Window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Gtk Sync")
        .build();

    window.set_size_request(APP_WIDTH, APP_HEIGHT);

    // Box Model
    let box_model = Box::new(Orientation::Vertical, 4);
    window.set_child(Some(&box_model));

    // Grid
    let button_grid = Grid::builder()
        .margin_start(40)
        .margin_end(40)
        .margin_top(20)
        .margin_bottom(20)
        .halign(Align::Center)
        .valign(Align::Center)
        .row_spacing(6)
        .column_spacing(20)
        .build();
    box_model.append(&button_grid);

    // Buttons
    let button_forward = Button::builder().label("Forward").build();
    let button_backward = Button::builder().label("Backward").build();
    let button_confirm = Button::builder().label("Confirm").build();

    let delete_option = CheckButton::builder().label("Delete").build();

    button_grid.attach(&button_forward, 0, 0, 1, 1);
    button_grid.attach(&button_backward, 1, 0, 1, 1);
    button_grid.attach(&button_confirm, 2, 0, 1, 1);
    button_grid.attach(&delete_option, 3, 0, 1, 1);

    // ListBox for rsync output
    let list_box = ListBox::new();
    let list_scroll = ScrolledWindow::builder()
        .propagate_natural_height(true)
        .child(&list_box)
        .build();
    box_model.append(&list_scroll);

    // Actions
    button_forward.connect_clicked(glib::clone!(
        @weak list_box,
        @weak delete_option => move |_| {
            let delete = delete_option.is_active();
            actions::forward_action(&list_box, delete);
        }
    ));

    button_backward.connect_clicked(glib::clone!(
        @weak list_box,
        @weak delete_option => move |_| {
            let delete = delete_option.is_active();
            actions::backward_action(&list_box, delete);
        }
    ));

    button_confirm.connect_clicked(glib::clone!(
        @weak list_box,
        @weak delete_option => move |_| {
            let delete = delete_option.is_active();
            actions::confirm_action(&list_box, delete);
        }
    ));

    window.present();
}
