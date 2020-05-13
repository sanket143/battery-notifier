extern crate daemonize;

use battery_notifier::actions;
use std::fs::File;
use daemonize::Daemonize;

fn main() {
    let stdout = File::create("/tmp/battery_notifier_s.out").unwrap();
    let stderr = File::create("/tmp/battery_notifier_s.err").unwrap();

    let daemonize = Daemonize::new()
        .pid_file("/tmp/battery_notifier_s.pid") // Every method except `new` and `start`
        .chown_pid_file(true)      // is optional, see `Daemonize` documentation
        .working_directory("/tmp") // for default behaviour.
        .umask(0o777)    // Set umask, `0o027` by default.
        .stdout(stdout)  // Redirect stdout to `/tmp/daemon.out`.
        .stderr(stderr)  // Redirect stderr to `/tmp/daemon.err`.
        .exit_action(|| println!("Executed before master process exits"))
        .privileged_action(|| "Executed before drop privileges");

    match daemonize.start() {
        Ok(_) => {
          actions::ticker();
        }
        Err(e) => eprintln!("Error, {}", e),
    }

}

