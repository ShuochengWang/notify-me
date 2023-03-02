use ctor::ctor;
use notify_me::{EmailNotifier, Notify, WechatNotifier};

#[ctor]
fn init() {
    let _ = env_logger::builder()
        .filter_module("notify_me", log::LevelFilter::Debug)
        .init();
}

#[test]
fn test_wechat() {
    let token = std::fs::read_to_string("tests/wechat.config").unwrap();
    let notifier = WechatNotifier::new(&token);
    notifier.notify("testing title", "testing content").unwrap();
}

#[test]
fn test_email() {
    let config = std::fs::read_to_string("tests/email.config").unwrap();
    let mut iter = config.split_ascii_whitespace();
    let smtp_host = iter.next().unwrap();
    let smtp_username = iter.next().unwrap();
    let smtp_password = iter.next().unwrap();
    let recipient = iter.next().unwrap();

    let notifier = EmailNotifier::new(smtp_host, smtp_username, smtp_password, recipient).unwrap();
    notifier.notify("testing title", "testing content").unwrap();
}
