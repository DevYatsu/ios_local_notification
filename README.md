# ios_local_notification

[![Crates.io](https://img.shields.io/crates/v/ios_local_notification.svg)](https://crates.io/crates/ios_local_notification)
[![Docs.rs](https://docs.rs/ios_local_notification/badge.svg)](https://docs.rs/ios_local_notification)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#license)

Send **local notifications on iOS** directly from Rust, powered by [swift-rs](https://crates.io/crates/swift-rs).

⚠️ **Platform support**: This crate works **only on iOS**.  
Android or other platforms are **not supported**.

## ✨ Features

- Request notification permissions (default or with fine-grained options).
- Schedule one-time, repeating, or image-based notifications.
- Clear pending or delivered notifications.
- Remove notifications by identifier.
- Retrieve lists of pending or delivered notifications.

## 📦 Installation

`cargo add ios_local_notification`

## 🚀 Usage

```rust
use ios_local_notification as notif;

fn main() {
    // Request permission with default options (alert, sound, badge)
    notif::request_permission_default();

    // Or request custom permissions
    notif::request_permission(
        notif::permission::ALERT | notif::permission::SOUND,
    );

    // Schedule a notification after 5 seconds
    notif::schedule("welcome", "Hello!", "This is your first notification 🚀", 5);

    // Schedule a repeating notification
    notif::schedule_repeat("ping", "Reminder", "This repeats every 10s", 10);

    // Schedule with an image attachment
    notif::schedule_image("img1", "Picture!", "With an image", "example.png", 5);

    // Query notifications
    let pending = notif::pending_notifications();
    let delivered = notif::delivered_notifications();
    println!("Pending: {:?}", pending);
    println!("Delivered: {:?}", delivered);

    // Remove a specific notification
    notif::remove_by_id("welcome");

    // Clear all notifications
    notif::clear_all_pending();
    notif::clear_all_delivered();
}
```

## 🔑 Permissions

You can request granular permissions using bitflags from `permission`:
```rust
use ios_local_notification::permission;

notif::request_permission(permission::ALERT | permission::SOUND);
```

Available flags:
+ ALERT → Show banners and alerts
+ SOUND → Play notification sounds
+ BADGE → Set app badge number
+ CARPLAY → Show notifications in CarPlay
+ CRITICAL_ALERT → Bypass Do Not Disturb
+ PROVISIONAL → Deliver quietly without user prompt

## 📚 Documentation

👉 Read the full API docs on [docs.rs](https://docs.rs/ios_local_notification)
