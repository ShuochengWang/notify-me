//! A Rust library for sending notifications.
//!
//! Send notifications to email or communication software, such as WeChat.
//! It is very suitable for developers to receive notifications of
//! their software on mobile phones.
//!
//! ## Features
//! - Send notifications to your email
//! - Send notifications to your WeChat
//!
//! ## Example
//! To use this library, add the following to your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! notify-me = "0.2"
//! ```
//!
//! ### Send notifications to WeChat
//!
//! Note that, this crate use [xtuis](https://xtuis.cn/) to implement WeChat notifications.
//! Hence you have to first follow the WeChat official account of [xtuis](https://xtuis.cn/) and get the `token`.
//!
//! ```no_run
//! use notify_me::{Notify, WechatNotifier};
//!
//! let notifier = WechatNotifier::new("your xtuis token").unwrap();
//! notifier.notify("notification title", "notification content").unwrap();
//! ```
//!
//! ### Send notifications to email
//!
//! ```no_run
//! use notify_me::{Notify, EmailNotifier};
//!
//! let notifier = EmailNotifier::new("smtp_host", "smtp_username", "smtp_password", "recipient").unwrap();
//! notifier.notify("notification title", "notification content").unwrap();
//! ```
mod notifier;

pub use notifier::email_notifier::EmailNotifier;
pub use notifier::wechat_notifier::WechatNotifier;
pub use notifier::Notify;
