use log::LevelFilter;
use log4rs::append::console::{ConsoleAppender, Target};
use log4rs::config::{Appender, Root};
use log4rs::Config;
use std::sync::Once;
static LOG: Once = Once::new();

pub(crate) fn init_logging() {
    LOG.call_once(|| {
        let stderr = ConsoleAppender::builder()
            .target(Target::Stderr)
            .encoder(Box::new(log4rs::encode::pattern::PatternEncoder::new(
                "{d(%Y-%m-%d %H:%M:%S%.6f)} {T:>15.15} {h({l:>5.5})} {t}:{L} - {m}{n}",
            )))
            .build();

        let config = Config::builder()
            .appender(Appender::builder().build("stderr", Box::new(stderr)))
            .build(Root::builder().appender("stderr").build(LevelFilter::Info))
            .unwrap();

        log4rs::init_config(config).unwrap();
    })
}
