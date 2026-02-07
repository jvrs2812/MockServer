use rand::Rng;
use tokio::time::{sleep, Duration};

use crate::config::DelayConfig;

pub async fn apply_delay(delay: &DelayConfig) {
    let delay_ms = match delay {
        DelayConfig::Fixed(ms) => *ms,
        DelayConfig::Range { min, max, .. } => {
            let mut rng = rand::thread_rng();
            rng.gen_range(*min..=*max)
        }
    };

    if delay_ms > 0 {
        tracing::debug!("Applying delay of {}ms", delay_ms);
        sleep(Duration::from_millis(delay_ms)).await;
    }
}
