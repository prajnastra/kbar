use better_term::{flush_styles, Color};
#[cfg(feature = "crossterm")]
use crossterm::{cursor, execute};
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

/// The struct for handling progress bars
///
/// # Example:
/// ```rust
/// use kbar::{KBar, BarType};
/// use std::thread::sleep;
/// use std::time::Duration;
///
/// fn main() {
///     // using crossterm, this will create a kbar at 0,0
///     // without crossterm, this is the only way to create a bar
///     let mut kbar = KBar::new(BarType::Bar, true, true, 20);
///
///     for x in 0..1000 {
///         // get the percentage complete as a decimal
///         let percentage_decimal = x as f32 / 1000.0;
///         // scale the percentage from 0..1 to 0..100 and convert to a u8
///         let percent = (percentage_decimal * 100.0) as u8;
///         // update the kbar
///         kbar.update(percent);
///         // draw the kbar
///         kbar.draw();
///         // delay for 10ms, making this run in 10 seconds
///         sleep(Duration::from_millis(10));
///     }
/// }
/// ```
///
/// # crossterm specific creation
/// Creating at a location
/// ```rust
/// use kbar::{KBar, BarType};
///
/// KBar::new_at(0, 0, BarType::Bar, true, true, 20);
/// ```
///
/// Creating at the cursor's current location
/// ```rust
/// use kbar::{KBar, BarType};
///
/// KBar::new_at_cursor(BarType::Bar, true, true, 20);
/// ```
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
                        "{dc}[{}{}{dc}]{}",
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

/// Hides the cursor using crossterm
#[cfg(feature = "crossterm")]
pub fn hide_cursor() {
    execute!(stdout(), crossterm::cursor::Hide).expect("Failed to hide cursor!");
}

/// Shows the cursor using crossterm
#[cfg(feature = "crossterm")]
pub fn show_cursor() {
    execute!(stdout(), crossterm::cursor::Show).expect("Failed to show cursor!");
}

impl Default for KBar {
    /// Creates a bar using default values
    #[cfg(feature = "crossterm")]
    fn default() -> Self {
        Self::new_at_cursor(BarType::Bar, true, true, 20).expect("Failed to make default KBar")
    }

    /// Creates a bar using default values
    #[cfg(not(feature = "crossterm"))]
    fn default() -> Self {
        Self::new(BarType::Bar, true, true, 20)
    }
}

#[cfg(test)]
mod tests {
    use crate::{hide_cursor, show_cursor};
    use crate::{BarType, KBar};
    use crossterm::execute;
    use crossterm::terminal::ClearType;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn t1() {
        use std::io::stdout;
        execute!(stdout(), crossterm::terminal::Clear(ClearType::All))
            .expect("Failed to clear screen!");
        let mut kbar = KBar::new_at(0, 1, BarType::RawBar, true, true, 20);
        let mut pbar2 = KBar::new_at(0, 3, BarType::Bar, true, true, 20);
        let mut pbar3 = KBar::new_at(7, 5, BarType::Dots, true, true, 20);
        let mut pbar4 = KBar::new_at(8, 7, BarType::Line, true, true, 20);

        hide_cursor();

        execute!(stdout(), crossterm::cursor::MoveTo(0, 5)).expect("Failed to move!");
        print!("Loading");

        execute!(stdout(), crossterm::cursor::MoveTo(0, 7)).expect("Failed to move!");
        print!("Loading");

        let max = 1000;
        for x in 0..max {
            let percent = ((x as f32 / (max - 1) as f32) * 100.0) as u8;
            kbar.update(percent);
            kbar.draw();

            pbar2.update(percent);
            pbar2.draw();

            pbar3.update(percent);
            pbar3.draw();

            pbar4.update(percent);
            pbar4.draw();
            thread::sleep(Duration::from_millis(10));
        }
        println!();
        show_cursor();
    }
}
