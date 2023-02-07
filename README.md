# kbar

A progrss bar for cli.

### Example:

```rust
use kbar::Bar;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut bar = Bar::new();
    bar.set_job_label("Percentage");

    for i in 0..101 {
        sleep(Duration::from_millis(100));
        bar.reach_percent(i);
    }
}
```
