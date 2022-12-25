use gtk::*;
use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};

const LOCAL: &str = "test_local/";
const REMOTE: &str = "test_remote/";

static DIRECTION: AtomicBool = AtomicBool::new(false);

fn clear_list_box(list_box: &ListBox) {
    while let Some(row) = list_box.row_at_index(0) {
        list_box.remove(&row);
    }
}

fn call_rsync(local: &str, remote: &str, list_box: &ListBox) {
    let rsync_output = Command::new("rsync")
        .arg("-nvr")
        .arg("--ignore-existing")
        .arg(local)
        .arg(remote)
        .output();
    match rsync_output {
        Err(_) => {
            let label = Label::new(Some("Error running rsync"));
            list_box.append(&label);
        }
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            for line in stdout.lines() {
                if line.starts_with("sending") {
                    continue;
                }
                if line.is_empty() {
                    break;
                }

                let label = Label::new(Some(line));
                list_box.append(&label);
            }
        }
    }
}

pub fn forward_action(list_box: &ListBox) {
    DIRECTION.store(false, Ordering::Relaxed);
    clear_list_box(list_box);
    call_rsync(LOCAL, REMOTE, list_box)
}

pub fn backward_action(list_box: &ListBox) {
    DIRECTION.store(true, Ordering::Relaxed);
    clear_list_box(list_box);
    call_rsync(REMOTE, LOCAL, list_box)
}

pub fn confirm_action(list_box: &ListBox) {
    let local: &str;
    let remote: &str;

    if !DIRECTION.load(Ordering::Relaxed) {
        local = LOCAL;
        remote = REMOTE;
    } else {
        local = REMOTE;
        remote = LOCAL;
    }

    let rsync_result = Command::new("rsync")
        .arg("-r")
        .arg(local)
        .arg(remote)
        .output();
    if let Err(_) = rsync_result {
        eprintln!("Unsuccessful call to rsync");
    } else {
        let label = Label::new(Some("Success!"));
        list_box.append(&label);
    }
}
