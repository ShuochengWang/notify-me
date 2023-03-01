use anyhow::Result;

pub mod wechat_notifier;

pub trait Notify {
    fn notify(&self, title: &str, message: &str) -> Result<()>;
}
