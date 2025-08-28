# ⏱️ health-reminder

A lightweight Rust CLI tool that helps you take care of your **eyes** and **hydration**.  
It reminds you to look away from the screen every 20 minutes (20-20-20 rule) and to drink water every hour.  
Runs in the terminal with optional desktop notifications (Linux/macOS).

---

## ✨ Features

- Eye reminders 👀 every *N* minutes or hours
- Water reminders 💧 every *N* minutes or hours
- Terminal messages (always)
- Native desktop notifications (Linux/macOS, optional)
- Simple command-line configuration

---

## 🚀 Usage

Run with default settings (20 minutes for eyes, 1 hour for water):

```bash
./health_reminders
````

Customize intervals:

```bash
./health_reminders --eyes 25m --water 2h
```

### Arguments

| Option    | Default | Description                                     |
|-----------|---------|-------------------------------------------------|
| `--eyes`  | `20m`   | Interval for eye reminders (minutes or hours)   |
| `--water` | `1h`    | Interval for water reminders (minutes or hours) |

👉 Only **minutes (`m`)** and **hours (`h`)** are supported.
Examples: `15m`, `45m`, `2h`.

---

## 🖥️ Notifications

* **Linux / FreeBSD** → Uses `notify-rust` (DBus)
* **macOS** → Uses `mac-notification-sys`
* **Other OS** → Terminal output only

