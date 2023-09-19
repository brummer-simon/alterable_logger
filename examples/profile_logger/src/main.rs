// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Author: Simon Brummer (simon.brummer@posteo.de)

use std::time::Instant;

struct DummyLogger;

impl log::Log for DummyLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, _record: &log::Record) {
    }

    fn flush(&self) {
    }
}

fn main() {
    let cycles = 1000000;

    log::set_max_level(log::LevelFilter::Info);
    log::set_boxed_logger(Box::new(DummyLogger)).unwrap();

    println!("Log profiling test program");
    println!("--------------------------");
    println!("Log info messages {} times", cycles);

    // Measure Time
    let now = Instant::now();

    for _ in 0..cycles {
        log::info!("Info Message");
    }

    let duration = now.elapsed();
    println!("Total operation time: {}ns", duration.as_nanos());
    println!("Operation time per cycle: {}ns", duration.as_nanos() / cycles);
}
