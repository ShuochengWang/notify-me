use super::*;
use lettre::{
    message::Mailbox, transport::smtp::authentication::Credentials, Message, SmtpTransport,
    Transport,
};

/// Notifier for email.
/// One EmailNotifier can only notify one corresponding mailbox.
pub struct EmailNotifier {
    smtp_host: String,
    smtp_creds: Credentials,
    sender: Mailbox,
    recipient: Mailbox,
}

impl EmailNotifier {
    /// # Arguments
    ///
    /// * `smtp_host` - SMTP server using TLS connections, e.g., `smtp.gmail.com`
    /// * `smtp_username` - Your SMTP username, e.g., `sender@gmail.com`
    /// * `smtp_password` - Your SMTP password.
    /// * `recipient` - Email recipient, e.g., `recipient@gmail.com`
    pub fn new(
        smtp_host: &str,
        smtp_username: &str,
        smtp_password: &str,
        recipient: &str,
    ) -> Result<Self> {
        let smtp_host = smtp_host.to_string();
        let smtp_creds = Credentials::new(smtp_username.to_string(), smtp_password.to_string());
        let sender = smtp_username.parse()?;
        let recipient = recipient.parse()?;

        if !SmtpTransport::relay(&smtp_host)?
            .credentials(smtp_creds.clone())
            .build()
            .test_connection()?
        {
            return Err(anyhow::anyhow!("test connection failed"));
        }

        Ok(Self {
            smtp_host,
            smtp_creds,
            sender,
            recipient,
        })
    }
}

impl Notify for EmailNotifier {
    fn notify(&self, title: &str, content: &str) -> Result<()> {
        let mailer = SmtpTransport::relay(&self.smtp_host)?
            .credentials(self.smtp_creds.clone())
            .build();
        let email = Message::builder()
            .from(self.sender.clone())
            .to(self.recipient.clone())
            .subject(title)
            .body(content.to_string())?;

        mailer.send(&email)?;

        Ok(())
    }
}
