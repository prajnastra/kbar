use std::io::{self, Write};

use terminal_size::{terminal_size, Width};

pub struct Bar {
    job_title: String,
    progress_percentage: i32,
    left_cap: String,
    right_cap: String,
    filled_symbol: String,
    empty_symbol: String,
}

impl Bar {
    /// Create a new progress bar.
    pub fn new() -> Bar {
        Bar {
            job_title: String::new(),
            progress_percentage: 0,
            left_cap: String::from("["),
            right_cap: String::from("]"),
            filled_symbol: String::from("█"),
            empty_symbol: String::from(" "),
        }
    }

    /// Reset progress percentage to zero and job title to empty string. Also
    /// prints "\n".
    pub fn jobs_done(&mut self) {
        self.job_title.clear();
        self.progress_percentage = 0;

        print!("\n");
    }

    /// Set text shown in progress bar.
    pub fn set_job_label(&mut self, new_title: &str) {
        self.job_title.clear();
        self.job_title.push_str(new_title);
        self._show_progress();
    }

    /// Put progress to given percentage.
    pub fn reach_percent(&mut self, percent: i32) {
        self.progress_percentage = percent;
        self._show_progress();
    }

    /// Increase progress with given percentage.
    pub fn add_percent(&mut self, progress: i32) {
        self.progress_percentage += progress;
        self._show_progress();
    }
}

impl Bar {
    fn _show_progress(&self) {
        let width = if let Some((Width(w), _)) = terminal_size() {
            w as i32
        } else {
            81 as i32
        };
        let overhead = self.progress_percentage / 100;
        let left_percentage = self.progress_percentage - overhead * 100;
        let bar_len = width - (50 + 5) - 2;
        let bar_finished_len = ((bar_len as f32) * (left_percentage as f32 / 100.0)) as i32;
        let filled_symbol = if overhead & 0b1 == 0 {
            &self.filled_symbol
        } else {
            &self.empty_symbol
        };
        let empty_symbol = if overhead & 0b1 == 0 {
            &self.empty_symbol
        } else {
            &self.filled_symbol
        };

        io::stdout().flush().unwrap();
        print!("\r");

        print!("{:<50}", self.job_title);
        print!("{}", self.left_cap);
        for _ in 0..bar_finished_len {
            print!("{}", filled_symbol);
        }
        for _ in bar_finished_len..bar_len {
            print!("{}", empty_symbol);
        }
        print!("{}", self.right_cap);
        print!("{:>4}%", self.progress_percentage);
    }
}
