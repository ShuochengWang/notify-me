use anyhow::Result;

pub mod wechat_notifier;

pub trait Notify {
    fn notify(&self, title: &str, content: &str) -> Result<()>;
}
