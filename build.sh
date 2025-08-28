cargo build --release
strip target/release/health_reminders
cp target/release/health_reminders ./