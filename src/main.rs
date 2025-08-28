use clap::Parser;
use std::time::Duration;
use tokio::{signal, time};

#[derive(Parser, Debug)]
#[command(
    name = "health-reminder",
    version,
    about = "Eye & water break reminders"
)]
struct Args {
    /// Interval for resting eyes (e.g. 20m, 30m, 2h)
    #[arg(long, default_value = "20m")]
    eyes: String,

    /// Interval for drinking water (e.g. 1h, 45m)
    #[arg(long, default_value = "1h")]
    water: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let eyes_every = parse(&args.eyes, "eyes")?;
    let water_every = parse(&args.water, "water")?;

    println!(
        "⏱️  Reminders started: eyes every {:?}, water every {:?}.",
        eyes_every, water_every
    );
    println!("Stop with Ctrl+C");

    // Eyes task
    let mut eyes_interval = time::interval(eyes_every);
    eyes_interval.tick().await;
    let eyes_task = tokio::spawn(async move {
        loop {
            eyes_interval.tick().await;
            remind("Eye break 👀", "Look away for ~20s. 20-20-20 rule!").await;
        }
    });

    // Water task
    let mut water_interval = time::interval(water_every);
    water_interval.tick().await;
    let water_task = tokio::spawn(async move {
        loop {
            water_interval.tick().await;
            remind("Hydration 💧", "Drink a few sips of water.").await;
        }
    });

    signal::ctrl_c().await?;
    println!("\n👋 Exit requested. Take care!");
    eyes_task.abort();
    water_task.abort();
    Ok(())
}

/// Parse durations with only minutes (m) or hours (h)
fn parse(s: &str, name: &str) -> anyhow::Result<Duration> {
    if let Some(stripped) = s.strip_suffix("m") {
        let minutes: u64 = stripped
            .parse()
            .map_err(|_| anyhow::anyhow!("Could not parse --{}='{}': invalid number", name, s))?;
        Ok(Duration::from_secs(minutes * 60))
    } else if let Some(stripped) = s.strip_suffix("h") {
        let hours: u64 = stripped
            .parse()
            .map_err(|_| anyhow::anyhow!("Could not parse --{}='{}': invalid number", name, s))?;
        Ok(Duration::from_secs(hours * 3600))
    } else {
        Err(anyhow::anyhow!(
            "Invalid value for --{}='{}': only 'm' (minutes) or 'h' (hours) are allowed",
            name,
            s
        ))
    }
}

async fn remind(title: &str, body: &str) {
    // Always print to terminal
    let now = chrono::Local::now().format("%H:%M:%S");
    println!("[{}] {} — {}", now, title, body);

    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    {
        if let Err(e) = notify_rust::Notification::new()
            .summary(title)
            .body(body)
            .show()
        {
            eprintln!("(DBus notification failed: {e})");
        }
    }

    #[cfg(target_os = "macos")]
    {
        use mac_notification_sys::*;
        if let Err(e) = send_notification(title, &Notification::default(), body, None) {
            eprintln!("(macOS notification failed: {e})");
        }
    }

    #[cfg(not(any(target_os = "linux", target_os = "freebsd", target_os = "macos")))]
    {
        // Other OS: no native notifications -> terminal only
    }
}

mod chrono {
    pub use ::chrono::*;
}
