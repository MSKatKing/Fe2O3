#![feature(random)]

mod application;
mod workloads;
mod status_tags;
mod async_task;
mod networking;
mod game;
mod settings;

use std::{fs, io};
use std::fs::OpenOptions;
use chrono::Utc;
use tracing::{Event, Level, Subscriber};
use tracing_subscriber::{fmt, Registry};
use tracing_subscriber::fmt::{FmtContext, FormatEvent, FormatFields};
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::time::{ChronoLocal, FormatTime};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::util::SubscriberInitExt;
use crate::application::Application;

fn main() {
    init_tracing().expect("Failed to initialize tracing.");

    if std::env::var("LOG_DEBUG")
        .map(|val| val == "1" || val == "true")
        .unwrap_or(false) {
        tracing::warn!("Environment variable 'LOG_DEBUG' is set to true! The logs will contain the source files and lines where the message was printed. This is meant for debug environments only, so if this isn't a debug message, it is recommended to set the environment variable to false.");
    }

    tracing::info!("Starting server...");

    let app = Application::new();

    tracing::info!("Beginning ticking...");

    app.run()
}

struct CustomFormatter;

impl<S: Subscriber + for<'a> LookupSpan<'a>, N: for<'writer> FormatFields<'writer> + 'static> FormatEvent<S, N> for CustomFormatter {
    fn format_event(&self, ctx: &FmtContext<'_, S, N>, mut writer: Writer<'_>, event: &Event<'_>) -> std::fmt::Result {
        let metadata = event.metadata();

        write!(&mut writer, "\x1b[90m")?;
        ChronoLocal::new(String::from("[%H:%M:%S]")).format_time(&mut writer)?;
        write!(&mut writer, "\x1b[0m")?;

        write!(writer, " \x1b[90m[Thread/{}]", std::thread::current().name().unwrap_or("Unknown"))?;

        write!(writer, " ")?;
        let level = *metadata.level();
        match level {
            Level::ERROR => write!(writer, "\x1b[31mERROR")?, // Red
            Level::WARN => write!(writer, "\x1b[33mWARN")?,  // Yellow
            Level::INFO => write!(writer, "\x1b[32mINFO")?,  // Green
            Level::DEBUG => write!(writer, "\x1b[34mDEBUG")?, // Blue
            Level::TRACE => write!(writer, "\x1b[90mTRACE")?, // Gray
        }

        let include_filename = std::env::var("LOG_DEBUG")
            .map(|val| val == "1" || val == "true")
            .unwrap_or(false);

        if include_filename {
            write!(writer, " \x1b[90m{}", metadata.target())?;
        }

        write!(writer, "\x1b[90m: ")?;

        match level {
            Level::ERROR => write!(writer, "\x1b[91m")?, // Red
            Level::WARN => write!(writer, "\x1b[93m")?,  // Yellow
            Level::INFO => write!(writer, "\x1b[0m")?,  // Green
            Level::DEBUG => write!(writer, "\x1b[0m")?, // Blue
            Level::TRACE => write!(writer, "\x1b[0m")?, // Gray
        }

        ctx.field_format().format_fields(writer.by_ref(), event)?;

        write!(writer, "\x1b[0m")?;
        writeln!(writer)
    }
}

fn init_tracing() -> io::Result<()> {
    let file_name = format!("logs/{}.log", Utc::now().format("%Y-%m-%d_%H-%M-%S"));
    fs::create_dir_all("logs")?;

    let log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_name)?;

    let console_layer = fmt::Layer::default()
        .event_format(CustomFormatter)
        .with_writer(io::stdout);

    let file_layer = fmt::Layer::default()
        .event_format(CustomFormatter)
        .with_ansi(false)
        .with_writer(move || log_file.try_clone()
            .expect("Failed to clone log file!"));

    let subscriber = Registry::default()
        .with(console_layer)
        .with(file_layer);

    subscriber.try_init().expect("Logger has already been set");

    Ok(())
}

#[macro_export]
macro_rules! safe_borrow_unique {
    ($storages:expr, $item:ident, $call:ident, $($entry:expr),*) => {
        {
            let a = $storages.get_unique::<&$item>().expect(format!("{:?} should exist", std::any::TypeId::of::<$item>()).as_str());
            let out = a.$call($($entry,)*);
            drop(a);
            out
        }
    };
    ($storages:expr, $item:ident, $call:ident) => {
        {
            let a = $storages.get_unique::<&$item>().expect(format!("{:?} should exist", std::any::TypeId::of::<$item>()).as_str());
            let out = a.$call();
            drop(a);
            out
        }
    };
}

#[macro_export]
macro_rules! safe_borrow_unique_mut {
    ($storages:expr, $item:ident, $call:ident, $($entry:expr),*) => {
        {
            let mut a = $storages.borrow::<shipyard::UniqueViewMut<$item>>().expect(format!("{:?} should exist", std::any::TypeId::of::<$item>()).as_str());
            let out = a.$call($($entry,)*);
            drop(a);
            out
        }
    };
    ($storages:expr, $item:ident, $call:ident) => {
        {
            let mut a = $storages.borrow::<shipyard::UniqueViewMut<$item>>().expect(format!("{:?} should exist", std::any::TypeId::of::<$item>()).as_str());
            let out = a.$call();
            drop(a);
            out
        }
    };
}