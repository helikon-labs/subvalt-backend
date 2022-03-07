use crate::{NotificationProcessor, CONFIG};
use log::{error, info};
use subvt_types::app::NotificationPeriodType;
use tokio::runtime::Builder;

impl NotificationProcessor {
    /// Checks and sends notifications that should be sent immediately.
    pub(crate) fn start_immediate_notification_processor(&'static self) -> anyhow::Result<()> {
        info!("Start immediate notification processor.");
        let tokio_rt = Builder::new_current_thread().enable_all().build()?;
        std::thread::spawn(move || loop {
            info!("Check immediate notifications.");
            if let Err(error) =
                tokio_rt.block_on(self.process_notifications(NotificationPeriodType::Immediate, 0))
            {
                error!(
                    "Error while processing immediate notifications: {:?}",
                    error
                );
            }
            std::thread::sleep(std::time::Duration::from_millis(
                CONFIG.notification_processor.sleep_millis,
            ));
        });
        Ok(())
    }
}