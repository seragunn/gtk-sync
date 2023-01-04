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

fn rsync_dryrun(local: &str, remote: &str, list_box: &ListBox, delete: bool) {
    let mut args = vec!["-nvr", "--ignore-existing", local, remote];
    if delete {
        args.push("--delete");
    }

    let rsync_output = Command::new("rsync").args(args).output();
    // handle possible error
    if rsync_output.is_err() {
        let label = Label::new(Some("Error running rsync"));
        list_box.append(&label);
        return;
    }
    // rsync_output is Ok
    let output = unsafe { rsync_output.unwrap_unchecked() };

    // check for error messages
    let stderr = String::from_utf8_lossy(&output.stderr);
    for line in stderr.lines() {
        let label = Label::new(Some(line));
        list_box.append(&label);
    }

    // write list of files to list_box
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

pub fn forward_action(list_box: &ListBox, delete: bool) {
    DIRECTION.store(false, Ordering::Relaxed);
    clear_list_box(list_box);
    rsync_dryrun(LOCAL, REMOTE, list_box, delete)
}

pub fn backward_action(list_box: &ListBox, delete: bool) {
    DIRECTION.store(true, Ordering::Relaxed);
    clear_list_box(list_box);
    rsync_dryrun(REMOTE, LOCAL, list_box, delete)
}

pub fn confirm_action(list_box: &ListBox, delete: bool) {
    let local: &str;
    let remote: &str;

    // recall last pressed button to decide which way to send files
    if !DIRECTION.load(Ordering::Relaxed) {
        local = LOCAL;
        remote = REMOTE;
    } else {
        local = REMOTE;
        remote = LOCAL;
    }

    let mut args = vec!["-r", local, remote];
    if delete {
        args.push("--delete");
    }

    let rsync_result = Command::new("rsync").args(args).spawn();
    if rsync_result.is_err() {
        eprintln!("Unsuccessful call to rsync");
    } else {
        let label = Label::new(Some("Success!"));
        list_box.append(&label);
    }
}
