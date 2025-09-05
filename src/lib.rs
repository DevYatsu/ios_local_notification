//! # ios_local_notification
//!
//! `ios_local_notification` provides a safe Rust interface for scheduling,
//! managing, and clearing local notifications on **iOS**.
//!
//! This crate is a thin wrapper over iOS' native notification APIs
//! through [`swift-rs`](https://crates.io/crates/swift-rs).
//!
//! ⚠️ **Platform support**: Only **iOS** is supported.  
//! Android support was considered, but not implemented.
//!
//! ## Example
//! ```ignore
//! use ios_local_notification as notif;
//!
//! // Request permission with default options
//! notif::request_permission_default();
//!
//! // Schedule a notification after 5 seconds
//! notif::schedule("id1", "Hello", "World", 5);
//!
//! // Retrieve pending notifications
//! let pending = notif::pending_notifications();
//! println!("Pending: {:?}", pending);
//! ```

/// Permission bitflags for configuring notification permissions.
///
/// Combine them with bitwise OR (`|`) when requesting permissions.
/// For example:
/// ```ignore
/// use ios_local_notification::permission;
/// notif::request_permission(permission::ALERT | permission::SOUND);
/// ```
pub mod permission {
    /// Display alert banners and popups.
    pub const ALERT: i32 = 0b000001;
    /// Play sounds for notifications.
    pub const SOUND: i32 = 0b000010;
    /// Show a badge on the app icon.
    pub const BADGE: i32 = 0b000100;
    /// Display notifications while connected to CarPlay.
    pub const CARPLAY: i32 = 0b001000;
    /// Allow critical alerts (bypass Do Not Disturb).
    pub const CRITICAL_ALERT: i32 = 0b010000;
    /// Grant provisional authorization (deliver quietly).
    pub const PROVISIONAL: i32 = 0b100000;
}

/// Represents a delivered or scheduled iOS notification.
///
/// Returned by [`pending_notifications`] or [`delivered_notifications`].
#[repr(C)]
#[derive(Debug, Clone)]
pub struct NotificationRequest {
    /// A developer-defined unique identifier for the notification.
    pub identifier: String,
    /// The main title of the notification.
    pub title: String,
    /// An optional subtitle for the notification.
    pub subtitle: String,
    /// The body text shown in the notification.
    pub body: String,
}

// iOS backend implementation (private)
#[cfg(target_os = "ios")]
mod backend {
    use super::NotificationRequest;
    use swift_rs::{Int, SRObject, SRObjectArray, SRString, swift};

    #[repr(C)]
    pub struct NotificationRequestFFI {
        pub identifier: SRString,
        pub title: SRString,
        pub subtitle: SRString,
        pub body: SRString,
    }

    impl From<&SRObject<NotificationRequestFFI>> for NotificationRequest {
        fn from(raw: &SRObject<NotificationRequestFFI>) -> Self {
            Self {
                identifier: raw.identifier.to_string(),
                title: raw.title.to_string(),
                subtitle: raw.subtitle.to_string(),
                body: raw.body.to_string(),
            }
        }
    }

    swift!(fn request_notification_permission_default());
    swift!(fn request_notification_permission(options: i32));

    swift!(fn schedule_notification(id: SRString, title: SRString, body: SRString, seconds: Int));
    swift!(fn schedule_repeat_notification(id: SRString, title: SRString, body: SRString, seconds: Int));
    swift!(fn schedule_image_notification(id: SRString, title: SRString, body: SRString, image: SRString, seconds: Int));

    swift!(fn remove_all_pending_notifications());
    swift!(fn remove_all_delivered_notifications());
    swift!(fn remove_notification_by_id(id: SRString));

    swift!(fn get_pending_notifications() -> SRObjectArray<NotificationRequestFFI>);
    swift!(fn get_delivered_notifications() -> SRObjectArray<NotificationRequestFFI>);

    pub fn request_permission_default() {
        unsafe { request_notification_permission_default() }
    }
    pub fn request_permission(options: i32) {
        unsafe { request_notification_permission(options) }
    }

    pub fn schedule(id: &str, title: &str, body: &str, seconds: isize) {
        unsafe { schedule_notification(id.into(), title.into(), body.into(), seconds) }
    }
    pub fn schedule_repeat(id: &str, title: &str, body: &str, seconds: isize) {
        unsafe { schedule_repeat_notification(id.into(), title.into(), body.into(), seconds) }
    }
    pub fn schedule_image(id: &str, title: &str, body: &str, image: &str, seconds: isize) {
        unsafe {
            schedule_image_notification(id.into(), title.into(), body.into(), image.into(), seconds)
        }
    }

    pub fn clear_all_pending() {
        unsafe { remove_all_pending_notifications() }
    }
    pub fn clear_all_delivered() {
        unsafe { remove_all_delivered_notifications() }
    }
    pub fn remove_by_id(id: &str) {
        unsafe { remove_notification_by_id(id.into()) }
    }

    pub fn pending_notifications() -> Vec<NotificationRequest> {
        unsafe {
            get_pending_notifications()
                .iter()
                .map(|req| req.into())
                .collect()
        }
    }
    pub fn delivered_notifications() -> Vec<NotificationRequest> {
        unsafe {
            get_delivered_notifications()
                .iter()
                .map(|req| req.into())
                .collect()
        }
    }
}

// ====== Public iOS API ======

/// Request default notification permissions (alert, sound, badge).
#[cfg(target_os = "ios")]
pub fn request_permission_default() {
    backend::request_permission_default()
}

/// Request notification permissions with custom options.
/// See [`permission`] constants for available flags.
#[cfg(target_os = "ios")]
pub fn request_permission(options: i32) {
    backend::request_permission(options)
}

/// Schedule a one-time notification.
///
/// - `id`: Unique identifier for the notification.
/// - `title`: Notification title.
/// - `body`: Notification body text.
/// - `seconds`: Delay before showing the notification.
#[cfg(target_os = "ios")]
pub fn schedule(id: &str, title: &str, body: &str, seconds: isize) {
    backend::schedule(id, title, body, seconds)
}

/// Schedule a repeating notification at fixed intervals.
#[cfg(target_os = "ios")]
pub fn schedule_repeat(id: &str, title: &str, body: &str, seconds: isize) {
    backend::schedule_repeat(id, title, body, seconds)
}

/// Schedule a notification that displays an image attachment.
#[cfg(target_os = "ios")]
pub fn schedule_image(id: &str, title: &str, body: &str, image: &str, seconds: isize) {
    backend::schedule_image(id, title, body, image, seconds)
}

/// Clear all **pending** notifications (scheduled but not yet delivered).
#[cfg(target_os = "ios")]
pub fn clear_all_pending() {
    backend::clear_all_pending()
}

/// Clear all **delivered** notifications (already shown).
#[cfg(target_os = "ios")]
pub fn clear_all_delivered() {
    backend::clear_all_delivered()
}

/// Remove a notification by its identifier.
#[cfg(target_os = "ios")]
pub fn remove_by_id(id: &str) {
    backend::remove_by_id(id)
}

/// Get a list of all pending notifications.
#[cfg(target_os = "ios")]
pub fn pending_notifications() -> Vec<NotificationRequest> {
    backend::pending_notifications()
}

/// Get a list of all delivered notifications.
#[cfg(target_os = "ios")]
pub fn delivered_notifications() -> Vec<NotificationRequest> {
    backend::delivered_notifications()
}
