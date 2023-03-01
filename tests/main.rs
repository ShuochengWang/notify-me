use notify_me::{Notify, WechatNotifier};

#[test]
fn test_wechat() {
    let token = std::fs::read_to_string("tests/wechat.config").unwrap();
    let notifier = WechatNotifier::new(&token);
    notifier.notify("testing title", "testing message").unwrap();
}
