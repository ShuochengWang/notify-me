use ctor::ctor;
use notify_me::{Notify, WechatNotifier};

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
    notifier.notify("testing title", "testing message").unwrap();
}
