use super::*;

/// Notifier for Wechat.
///
/// You can send message to your Wechat via this notifier.
/// It implemented by xtuis which is a wechat-notify-system.
/// The key of the notifier is the token, which you can get
/// according to the document in
/// [xtuis Chinese websites](https://xtuis.cn/xtuisindex.html)
pub struct WechatNotifier {
    token: String,
}

impl WechatNotifier {
    pub fn new(token: &str) -> Result<Self> {
        let token = token.to_string();
        Ok(Self { token })
    }
}

impl Notify for WechatNotifier {
    fn notify(&self, title: &str, content: &str) -> Result<()> {
        let url = format!("https://wx.xtuis.cn/{}.send?", self.token);
        let data = [("text", title), ("desp", content)];
        let resp = ureq::post(&url).send_form(&data)?;
        log::debug!("notify response: {:?}", resp);
        Ok(())
    }
}
