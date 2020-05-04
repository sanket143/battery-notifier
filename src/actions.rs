use battery::units;
use notify_rust::Notification;

pub fn show_notification(ratio: &units::Ratio) {

    let mut _body = String::from("Current Battery: ");
    println!("{:?}", ratio);

    Notification::new()
        .summary("Lisa")
        .body("hello")
        .show()
        .unwrap();
}
