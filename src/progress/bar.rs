use better_term::{flush_styles, Color};
#[cfg(feature = "crossterm")]
use crossterm::{cursor, execute, terminal};
use std::error::Error;
#[cfg(feature = "crossterm")]
use std::io::stdout;

/// The Type of bar to be used
#[derive(Debug, Clone)]
pub enum BarType {
    /// creates a bar that looks like "████████████████████"
    Bar, // [██████████]
    /// creates a bar that looks like "████████████████████"
    RawBar, // ██████
    /// Cycles through ., .., and ...
    Dots, // ...
    /// Cycles through |, /, -, and \
    Line, // |
}

#[derive(Debug, Clone)]
pub struct KBar {
    // crossterm position handling
    #[cfg(feature = "crossterm")]
    x: u16,
    #[cfg(feature = "crossterm")]
    y: u16,

    // settings
    color: bool,
    bar_type: BarType,
    show_percent: bool,
    bar_length: u16,

    // for special types
    repeat_at: u8,

    // value the bar is at
    percent: u8,
}

impl KBar {
    /// create a new progress bar when not using crossterm
    /// # parameters
    /// bar_type: the type of bar to display
    /// color: if it should output in color
    /// show_percent: if it should show the percentage next to the bar
    /// bar_length: How long the bar should be
    #[cfg(not(feature = "crossterm"))]
    pub fn new(bar_type: BarType, color: bool, show_percent: bool, bar_length: u16) -> Self {
        Self {
            percent: 0,
            color,
            bar_type,
            bar_length,
            show_percent,
            repeat_at: 0,
        }
    }

    #[cfg(not(feature = "crossterm"))]
    fn set_pos(&mut self) {
        print!("\r");
    }

    /// create a new progress bar when using crossterm
    /// # parameters
    /// bar_type: the type of bar to display
    /// color: if it should output in color
    /// show_percent: if it should show the percentage next to the bar
    /// bar_length: How long the bar should be
    #[cfg(feature = "crossterm")]
    pub fn new(bar_type: BarType, color: bool, show_percent: bool, bar_length: u16) -> Self {
        Self::new_at(0, 0, bar_type, color, show_percent, bar_length)
    }

    /// Create a new bar using crossterm at a location
    /// # parameters
    /// x: the x location to put the bar (0 is the left)
    /// y: the y location to put the bar (0 is the top)
    /// bar_type: the type of bar to display
    /// color: if it should output in color
    /// show_percent: if it should show the percentage next to the bar
    /// bar_length: How long the bar should be
    #[cfg(feature = "crossterm")]
    pub fn new_at(
        x: u16,
        y: u16,
        bar_type: BarType,
        color: bool,
        show_percent: bool,
        bar_length: u16,
    ) -> Self {
        Self {
            x,
            y,
            percent: 0,
            color,
            bar_type,
            bar_length,
            show_percent,
            repeat_at: 0,
        }
    }

    /// Creates a new bar using crossterm at the cursor's current position
    /// # parameters
    /// bar_type: the type of bar to display
    /// color: if it should output in color
    /// show_percent: if it should show the percentage next to the bar
    /// bar_length: How long the bar should be
    #[cfg(feature = "crossterm")]
    pub fn new_at_cursor(
        bar_type: BarType,
        color: bool,
        show_percent: bool,
        bar_length: u16,
    ) -> Result<Self, String> {
        let pos = crossterm::cursor::position();
        if pos.is_err() {
            return Err(pos.unwrap_err().to_string());
        }

        let (x, y) = pos.unwrap();

        Ok(Self::new_at(
            x,
            y,
            bar_type,
            color,
            show_percent,
            bar_length,
        ))
    }

    #[cfg(feature = "crossterm")]
    fn set_pos(&mut self) -> Result<(), String> {
        let r = execute!(stdout(), cursor::MoveTo(self.x, self.y));
        if r.is_err() {
            return Err(r.unwrap_err().to_string());
        }
        Ok(())
    }

    /// Update the bar's progress
    /// # Parameters
    /// percent: the new percentage (0 to 100)
    pub fn update(&mut self, mut percent: u8) {
        if percent > 100 {
            percent = 100;
        }
        self.percent = percent;
    }

    fn draw_dots_and_line(&mut self, output: String, color: Color) -> String {
        format!(
            "{}{}",
            output,
            if self.show_percent {
                if self.color {
                    format!(" {}{}%", color, self.percent)
                } else {
                    format!(" {}%", self.percent)
                }
            } else {
                format!("")
            }
        )
    }

    pub fn clear_term(&mut self) -> Result<(), Box<dyn Error>> {
        execute!(stdout(), terminal::Clear(terminal::ClearType::All))?;
        Ok(())
    }

    /// Draw the bar at its set location
    pub fn draw(&mut self) {
        #[cfg(feature = "crossterm")]
        {
            let r = self.set_pos();
            if r.is_err() {
                panic!("Failed to set cursor position!");
            }
        }

        #[cfg(not(feature = "crossterm"))]
        self.set_pos();

        // get the current completion color of the bar
        // only used if self.color is true
        let red = 255 - ((self.percent as f32 / 100.0) * 200.0) as u8;
        let green = (self.percent as f32 / 100.0 * 200.0) as u8;
        let color = Color::RGB(red, green, 25);

        let output = match self.bar_type {
            BarType::RawBar => {
                let chunk_weight = 100 / self.bar_length;
                // set how complete the bar should be
                let bar_completion = self.percent as usize / chunk_weight as usize;

                if self.color {
                    // create the colored bar
                    let bar = format!("{}{}", color, "█".repeat(bar_completion));

                    format!(
                        "{}{}",
                        bar,
                        if self.show_percent {
                            format!(" {}{}{}%", color, self.percent, Color::White)
                        } else {
                            format!("")
                        }
                    )
                } else {
                    // create the bar
                    let bar = format!("{}", "█".repeat(bar_completion));

                    format!(
                        "{}{}",
                        bar,
                        if self.show_percent {
                            format!(" {}%", self.percent)
                        } else {
                            format!("")
                        }
                    )
                }
            }
            BarType::Bar => {
                let chunk_weight = 100 / self.bar_length;
                // set how complete the bar should be
                let bar_completion = self.percent as usize / chunk_weight as usize;
                let mut bar_uncomplete = (100 - (self.percent)) as usize / chunk_weight as usize;
                // handle if the bar needs to be resized because of rounding issues
                let add = bar_completion + bar_uncomplete;
                if add < self.bar_length as usize {
                    bar_uncomplete += 1;
                }
                if add > self.bar_length as usize {
                    bar_uncomplete -= 1;
                }

                if self.color {
                    // create the main bar
                    let completed_bar = format!("{}{}", color, "█".repeat(bar_completion));
                    let uncompleted_bar =
                        format!("{}{}", Color::BrightBlack, "█".repeat(bar_uncomplete));

                    // format the bar and return it
                    format!(
                        "{dc}**[{}{}{dc}]{}",
                        completed_bar,
                        uncompleted_bar,
                        if self.show_percent {
                            format!(" {}{}{}%", color, self.percent, Color::White)
                        } else {
                            format!("")
                        },
                        dc = Color::White
                    )
                } else {
                    // create the main bar
                    let completed_bar = format!("{}", "█".repeat(bar_completion));
                    let uncompleted_bar = format!("{}", "=".repeat(bar_uncomplete));

                    // format the bar and return it
                    format!(
                        "[{}{}]{}",
                        completed_bar,
                        uncompleted_bar,
                        if self.show_percent {
                            format!(" {}%", self.percent)
                        } else {
                            format!("")
                        }
                    )
                }
            }
            BarType::Dots => {
                if self.percent % 2 == 0 {
                    self.repeat_at += 1;
                    if self.repeat_at == 3 {
                        self.repeat_at = 0;
                    }
                }

                let output = match self.repeat_at {
                    0 => {
                        format!(".  ")
                    }
                    1 => {
                        format!(".. ")
                    }
                    _ => {
                        format!("...")
                    }
                };

                self.draw_dots_and_line(output, color)
            }
            BarType::Line => {
                if self.percent % 2 == 0 {
                    self.repeat_at += 1;
                    if self.repeat_at == 4 {
                        self.repeat_at = 0;
                    }
                }

                let output = match self.repeat_at {
                    0 => {
                        format!("|")
                    }
                    1 => {
                        format!("/")
                    }
                    2 => {
                        format!("-")
                    }
                    _ => {
                        format!("\\")
                    }
                };

                self.draw_dots_and_line(output, color)
            }
        };

        #[cfg(feature = "crossterm")]
        print!("{}", output);
        #[cfg(not(feature = "crossterm"))]
        print!("\r{}", output);

        if self.color {
            flush_styles()
        }
    }

    pub fn is_complete(&self) -> bool {
        self.percent == 100
    }
}
