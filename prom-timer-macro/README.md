# prom-timer
Rust RAII Prometheus timer

```rust
use std::collections::HashMap;
    use std::thread::sleep;
    use std::time::Duration;
    use prometheus::{self, HistogramVec, histogram_opts};
    use prom_timer_macro::timer;
    use lazy_static::lazy_static;

    lazy_static! { static ref TIMER: HistogramVec = HistogramVec::new(
                histogram_opts!("timer", "Timer")
                    .namespace("api")
                    .const_labels(HashMap::new())
                    .buckets(
                        [
                            0.001, 0.0025, 0.005, 0.01, 0.025, 0.05, 0.1, 0.2, 0.3, 0.4, 0.5, 1.0, 2.0,
                           3.0, 4.0, 5.0, 10.0,
                        ]
                        .into(),
                    ),
                &["function"],
            )
            .unwrap();}

    #[timer(TIMER.clone(), "f")]
    fn f() {
        sleep(Duration::from_secs(1));
    }

    #[test]
    fn test() {
        f();
    }
```
