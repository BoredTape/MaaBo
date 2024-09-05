use std::path::PathBuf;

use log::LevelFilter;
use log4rs::{
    append::{
        console::{ConsoleAppender, Target},
        rolling_file::policy::compound::{
            roll::fixed_window::FixedWindowRoller, trigger::size::SizeTrigger, CompoundPolicy,
        },
    },
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
    filter::threshold::ThresholdFilter,
};

const TRIGGER_FILE_SIZE: u64 = 100 * 1024 * 1024;
const LOG_FILE_COUNT: u32 = 3;

pub fn init_logger(file_path: &PathBuf, file_name: &str) {
    let archive_pattern = String::from(file_path.join(file_name).to_str().unwrap()) + ".{}.log";
    let trigger = SizeTrigger::new(TRIGGER_FILE_SIZE);
    let roller = FixedWindowRoller::builder()
        .base(0)
        .build(&archive_pattern, LOG_FILE_COUNT)
        .unwrap();
    let policy = CompoundPolicy::new(Box::new(trigger), Box::new(roller));

    let log_file = log4rs::append::rolling_file::RollingFileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "[{d(%Y-%m-%d %H:%M:%S)}][{l}][{t}] {m}{n}",
        )))
        .build(
            file_path.join(format!("{}.log", file_name)),
            Box::new(policy),
        )
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("file", Box::new(log_file)))
        .appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(log::LevelFilter::Debug)))
                .build(
                    "debug",
                    Box::new(
                        ConsoleAppender::builder()
                            .encoder(Box::new(PatternEncoder::new(
                                "[{d(%Y-%m-%d %H:%M:%S)}][{l}][{t}] {m}{n}",
                            )))
                            .target(Target::Stdout)
                            .build(),
                    ),
                ),
        )
        .build(
            Root::builder()
                .appender("file")
                .appender("debug")
                .build(LevelFilter::Debug),
        )
        .unwrap();
    let _handle = log4rs::init_config(config).unwrap();
}
