use battery_notifier::actions;

fn main() -> battery::Result<()> {
    loop {
      actions::nudge();
    }
}

