use notify_me::*;

#[test]
fn it_works() {
    let notifier = notify_me::WechatNotifier::new("");
    notifier.notify().unwrap();
}
