use super::*;

pub struct WechatNotifier {
    token: String,
}

impl WechatNotifier {
    pub fn new(token: &str) -> Self {
        let token = token.to_string();
        Self { token }
    }
}

impl Notify for WechatNotifier {
    fn notify(&self) -> Result<()> {
        Ok(())
    }
}
