# kbar

A progrss bar for cli. See [Timer](https://github.com/prajnastra/timer) which is built using this library.

### Example:

- Simple

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


- Custom 

```rust
use kbar::Bar;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut bar = Bar::custom(
        // left cap symbol
        String::from("["),
        // right cap symbol
        String::from("]"),
        // filled symbol
        String::from("#"),
        // empty symbol
        String::from("-"),
    );

    bar.set_job_label("Percentage");

    for i in 0..101 {
        sleep(Duration::from_millis(100));
        bar.reach_percent(i);
    }
}
```
