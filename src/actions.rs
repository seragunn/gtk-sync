use gtk::*;

fn clear_list_box(list_box: &ListBox) {
    while let Some(row) = list_box.row_at_index(0) {
        list_box.remove(&row);
    }
}

pub fn forward_action(list_box: &ListBox) {
    for number in 0..=100 {
        let label = Label::new(Some(&number.to_string()));
        list_box.append(&label);
    }
}

pub fn backward_action(list_box: &ListBox){
    clear_list_box(list_box);
}
