use syslog::{Facility, Formatter3164};
mod settings;

use settings::Settings;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the syslog writer
    let formatter = Formatter3164 {
        facility: Facility::LOG_USER,
        hostname: None,
        process: "myapp".into(),
        pid: 0,
    };

    let mut logger = syslog::unix(formatter)?;

    // Write log messages to Journalctl
    logger.info("This is an informational message")?;
    logger.warning("This is a warning message")?;

    let settings = Settings::new();

    // Print out our settings
    //println!("{:?}", settings);
    Ok(())
}
