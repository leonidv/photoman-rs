use std::sync::Mutex;

pub struct ProgressIndicator {
    message: String,
    threshold: usize,
    max_threshold: String,
    is_percent: bool,
    current_step: Mutex<usize>
}

impl ProgressIndicator {
    pub(crate) fn new(total_steps: usize, message: String) -> ProgressIndicator {
        let (threshold, max_threshold, is_percent) = match total_steps {
            0..=99 => (1, total_steps.to_string(), false),
            _ => (total_steps / 100, "100%".to_string(), true),
        };

        ProgressIndicator {
            message,
            threshold,
            max_threshold,
            is_percent,
            current_step: Mutex::new(0)
        }
    }

    fn show_progress(&self, step_number: usize) {
        if step_number % self.threshold == 0 {
            let value = if self.is_percent {
                step_number / self.threshold
            } else {
                step_number
            };

            tracing::info!("{} {}/{}", self.message, value, self.max_threshold)
        }
    }

    pub(crate) fn step(&self) {
        let mut step = self.current_step.lock().unwrap();
        *step += 1;
        self.show_progress(*step);
    }
}
