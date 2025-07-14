use std::env;
use std::error::Error;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = env::args().skip(1).collect();

    let mut cmd = Command::new("ssh");
    cmd.args(args);

    loop {
        let mut child = cmd.spawn()?;
        if let Ok(status) = child.wait() {
            if status.success() {
                break;
            }
        }

        sleep(Duration::from_secs(1));
    }

    Ok(())
}
