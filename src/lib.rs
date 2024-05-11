use gargoyle::Notify;

use lettre::{
    Message, 
    transport::smtp::authentication::Credentials, 
    SmtpTransport, 
    Transport
};

pub use lettre::{
    message::Mailbox,
    Address,
};

use log::info;

/// Send an email notification.
///
/// # Example
///
/// ```
/// # use std::thread::sleep;
/// # use std::time::Duration;
/// use gargoyle::{modules::{notify, monitor}, Schedule};
/// let service_name = "nginx";
/// let service_monitor = monitor::ExactService::new(service_name);
/// let email_notifier = notify::Email {
///     from: "The Gargoyle <from@example.com>".parse().unwrap(),
///     to: "Administrator <admin@example.com>".parse().unwrap(),
///     relay: "smtp.example.com".to_string(),
///     smtp_username: "username".to_string(),
///     smtp_password: "password".to_string(),
/// };
/// let mut schedule = Schedule::default();
/// schedule.add(
///     &format!("The Gargoyle has detected that {service_name} has gone down"),
///     &format!("The Gargoyle has detected that {service_name} has recovered"),
///     Duration::from_secs(60),
///     &service_monitor,
///     &email_notifier,
/// );
///     
/// loop {
///     schedule.run();
///     sleep(Duration::from_millis(100));
/// }
/// ```
pub struct Email {
    pub from: Mailbox,
    pub to: Mailbox,
    pub relay: String,
    pub smtp_username: String,
    pub smtp_password: String,
}

/// Sends an email notification.
impl Notify for Email {
    fn send(&self, msg: &str, diagnostic: Option<String>) -> Result<(), String> {
        let email = Message::builder()
            .from(self.from.clone())
            .to(self.to.clone())
            .subject(msg)
            .body(diagnostic.unwrap_or(msg.to_string()))
            .map_err(|e| format!("Failed to build a message: {e}"))?;

        let creds = Credentials::new(self.smtp_username.clone(), self.smtp_password.clone());

        let mailer = SmtpTransport::relay(&self.relay)
            .map_err(|e| format!("Failed to create a mailer: {e}"))?
            .credentials(creds)
            .build();

        info!("Sending email notification from {} to {} via {}.", self.from, self.to, self.relay);
        match mailer.send(&email) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Failed to send email: {e}")),
        }
    }
}

