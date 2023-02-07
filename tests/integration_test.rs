#[cfg(feature = "crossterm")]
use crossterm::execute;
use kbar::{BarType, KBar};
#[cfg(feature = "crossterm")]
use std::io::stdout;

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
        let mut bar2 = KBar::new_at(0, 3, BarType::Bar, true, true, 20);
        let mut bar3 = KBar::new_at(7, 5, BarType::Dots, true, true, 20);
        let mut bar4 = KBar::new_at(8, 7, BarType::Line, true, true, 20);

        kbar.clear_term().expect("Not able to clear terminal buffer");

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

            bar2.update(percent);
            bar2.draw();

            bar3.update(percent);
            bar3.draw();

            bar4.update(percent);
            bar4.draw();
            thread::sleep(Duration::from_millis(10));
        }
        println!();
        show_cursor();
    }
}
