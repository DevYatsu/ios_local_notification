import Foundation
import SwiftRs
import UserNotifications

// sorry for the shitty code I am no swift developer...

// MARK: - Permissions

// Request default permission (all options)
@_cdecl("request_notification_permission_default")
public func request_notification_permission_default() {
    UNUserNotificationCenter.current().requestAuthorization(
        options: [.alert, .sound, .badge, .carPlay, .criticalAlert, .provisional]
    ) { _, error in
        if let error = error {
            print("❌ Permission error: \(error.localizedDescription)")
        }
    }
}

// Request custom permission (pass bitmask from Rust)
@_cdecl("request_notification_permission")
public func request_notification_permission(_ options: Int32) {
    var opts: UNAuthorizationOptions = []
    if options & 1 != 0 { opts.insert(.alert) }
    if options & 2 != 0 { opts.insert(.sound) }
    if options & 4 != 0 { opts.insert(.badge) }
    if options & 8 != 0 { opts.insert(.carPlay) }
    if options & 16 != 0 { opts.insert(.criticalAlert) }
    if options & 32 != 0 { opts.insert(.provisional) }

    UNUserNotificationCenter.current().requestAuthorization(options: opts) { _, error in
        if let error = error {
            print("❌ Permission error: \(error.localizedDescription)")
        }
    }
}

// MARK: - Scheduling

@_cdecl("schedule_notification")
public func schedule_notification(
    _ id: SRString, _ title: SRString, _ body: SRString, _ seconds: Int
) {
    let content = UNMutableNotificationContent()
    content.title = title.toString()
    content.body = body.toString()
    content.sound = .default

    let trigger = UNTimeIntervalNotificationTrigger(
        timeInterval: TimeInterval(seconds), repeats: false)
    let request = UNNotificationRequest(
        identifier: id.toString(), content: content, trigger: trigger)
    UNUserNotificationCenter.current().add(request, withCompletionHandler: nil)
}

@_cdecl("schedule_repeat_notification")
public func schedule_repeat_notification(
    _ id: SRString, _ title: SRString, _ body: SRString, _ seconds: Int
) {
    let content = UNMutableNotificationContent()
    content.title = title.toString()
    content.body = body.toString()
    content.sound = .default

    let trigger = UNTimeIntervalNotificationTrigger(
        timeInterval: TimeInterval(seconds), repeats: true)
    let request = UNNotificationRequest(
        identifier: id.toString(), content: content, trigger: trigger)
    UNUserNotificationCenter.current().add(request, withCompletionHandler: nil)
}

// MARK: - Attachments

@_cdecl("schedule_image_notification")
public func schedule_image_notification(
    _ id: SRString, _ title: SRString, _ body: SRString, _ image: SRString, _ seconds: Int
) {
    let content = UNMutableNotificationContent()
    content.title = title.toString()
    content.body = body.toString()
    content.sound = .default

    if let url = Bundle.main.url(forResource: image.toString(), withExtension: "png") {
        if let attachment = try? UNNotificationAttachment(
            identifier: "image", url: url, options: nil)
        {
            content.attachments = [attachment]
        }
    }

    let trigger = UNTimeIntervalNotificationTrigger(
        timeInterval: TimeInterval(seconds), repeats: false)
    let request = UNNotificationRequest(
        identifier: id.toString(), content: content, trigger: trigger)
    UNUserNotificationCenter.current().add(request, withCompletionHandler: nil)
}

// MARK: - Management

@_cdecl("remove_all_pending_notifications")
public func remove_all_pending_notifications() {
    UNUserNotificationCenter.current().removeAllPendingNotificationRequests()
}

@_cdecl("remove_all_delivered_notifications")
public func remove_all_delivered_notifications() {
    UNUserNotificationCenter.current().removeAllDeliveredNotifications()
}

@_cdecl("remove_notification_by_id")
public func remove_notification_by_id(_ id: SRString) {
    UNUserNotificationCenter.current().removePendingNotificationRequests(withIdentifiers: [
        id.toString()
    ])
    UNUserNotificationCenter.current().removeDeliveredNotifications(withIdentifiers: [id.toString()]
    )
}

@_cdecl("get_pending_notifications")
public func get_pending_notifications() -> SRObjectArray {
    let semaphore = DispatchSemaphore(value: 0)
    var results: [NotificationRequest] = []

    UNUserNotificationCenter.current().getPendingNotificationRequests { requests in
        for r in requests {
            results.append(NotificationRequest(from: r))
        }
        semaphore.signal()
    }

    semaphore.wait()
    return SRObjectArray(results)
}

@_cdecl("get_delivered_notifications")
public func get_delivered_notifications() -> SRObjectArray {
    let semaphore = DispatchSemaphore(value: 0)
    var results: [NotificationRequest] = []

    UNUserNotificationCenter.current().getDeliveredNotifications { notifications in
        for r in notifications {
            results.append(NotificationRequest(from: r.request))
        }
        semaphore.signal()
    }

    semaphore.wait()
    return SRObjectArray(results)
}

@objc
public class NotificationRequest: NSObject {
    private let _identifier: SRString
    private let _title: SRString
    private let _subtitle: SRString
    private let _body: SRString

    // Init from UNNotificationRequest
    @objc public init(from request: UNNotificationRequest) {
        self._identifier = SRString(request.identifier)
        self._title = SRString(request.content.title)
        self._subtitle = SRString(request.content.subtitle)
        self._body = SRString(request.content.body)
    }
}
