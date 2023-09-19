// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Author: Simon Brummer (simon.brummer@posteo.de)

use log::LevelFilter;
use simplelog::{SimpleLogger, Config};

fn main() {
    log::error!("You should not see this!");

    // Register a basic logger with maximum filter level warn
    let logger = SimpleLogger::new(LevelFilter::Warn, Config::default());
    alterable_logger::configure(LevelFilter::Error, logger);

    log::error!("You should see this!");
    log::warn!("But this is invisible!");

    // Change the log level to warning
    alterable_logger::set_max_level(LevelFilter::Warn);
    log::warn!("You should see this!");

    // Change the log level to info (this will be not visible as the logger does not support it)
    alterable_logger::set_max_level(LevelFilter::Info);
    log::info!("But this is invisible!");

    // Replace current logger with a logger that supports a more verbose logging
    let logger = SimpleLogger::new(LevelFilter::Info, Config::default());
    alterable_logger::set_boxed_logger(logger);
    log::info!("You should see this!");
}
