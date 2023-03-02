use anyhow::Result;

pub mod email_notifier;
pub mod wechat_notifier;

pub trait Notify {
    fn notify(&self, title: &str, content: &str) -> Result<()>;
}
