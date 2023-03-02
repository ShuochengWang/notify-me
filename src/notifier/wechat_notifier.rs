use super::*;

/// Notifier for WeChat.
/// One WechatNotifier can only notify one corresponding WeChat account.
///
/// You can send message to your WeChat via this notifier.
/// It implemented by [xtuis](https://xtuis.cn) which is a WeChat-notify-system.
/// To use this notifier, you have to first get a xtuis token according to
/// [xtuis Chinese websites](https://xtuis.cn)
pub struct WechatNotifier {
    token: String,
}

impl WechatNotifier {
    /// # Arguments
    ///
    /// * `token` - A xtuis token corresponding to your WeChat account.
    /// You can get the token according to the document in
    /// [xtuis Chinese websites](https://xtuis.cn)
    pub fn new(token: &str) -> Result<Self> {
        let token = token.to_string();
        Ok(Self { token })
    }
}

impl Notify for WechatNotifier {
    fn notify(&self, title: &str, content: &str) -> Result<()> {
        let url = format!("https://wx.xtuis.cn/{}.send?", self.token);
        let data = [("text", title), ("desp", content)];
        let _resp = ureq::post(&url).send_form(&data)?;
        Ok(())
    }
}
