use std::fmt;

use cfg_if::cfg_if;
use chrono::Local;
use nu_ansi_term::Color::{Blue, DarkGray, Green, Purple, Red, White, Yellow};
use tracing::{Level, Metadata};
use tracing_core::{Event, Subscriber};
use tracing_subscriber::registry::{Extensions, LookupSpan};
use tracing_subscriber::{
    fmt::{
        format::{FormatEvent, FormatFields, Writer},
        FmtContext, FormattedFields,
    },
    EnvFilter,
};

pub fn init() {
    cfg_if! {
        if #[cfg(feature = "bench")] {
            let filter = EnvFilter::builder().with_default_directive(Level::INFO.into());
        } else {
            let filter = EnvFilter::builder().with_default_directive(Level::TRACE.into());
        }
    }

    let filter: EnvFilter = filter
        .from_env()
        .expect("Failed to parse env trace filter")
        .add_directive("bevy=off".parse().unwrap());

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .event_format(UssrFormatter)
        .init();
}

macro_rules! write_colored {
    ($writer:expr, $color:expr, $($arg:tt)*) => {
        write!($writer, "{}", $color.paint(format!($($arg)*)))
    }
}

pub struct UssrFormatter;
impl<S, N> FormatEvent<S, N> for UssrFormatter
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        ctx: &FmtContext<'_, S, N>,
        mut writer: Writer<'_>,
        event: &Event<'_>,
    ) -> fmt::Result {
        let meta: &Metadata<'_> = event.metadata();

        write_colored!(writer, DarkGray, "[{}] [", Local::now().format("%H:%M:%S"))?;
        write_colored!(writer, DarkGray, "{}", meta.target())?;

        // Write spans
        if let Some(scope) = ctx.event_scope() {
            for span in scope.from_root() {
                // Write the span name
                write_colored!(writer, DarkGray, "::")?;
                write_colored!(writer, White.bold(), "{}", span.name())?;

                // Write the span fields
                let ext: Extensions = span.extensions();
                let fields: &&FormattedFields<N> = &ext.get::<FormattedFields<N>>().unwrap();
                if !fields.is_empty() {
                    write_colored!(writer, DarkGray, "{{{}}}", fields)?;
                }
            }
        }

        write_colored!(writer, DarkGray, "/")?;
        write!(
            writer,
            "{}",
            match *meta.level() {
                Level::TRACE => Purple.paint("TRACE"),
                Level::DEBUG => Blue.paint("DEBUG"),
                Level::INFO => Green.paint("INFO"),
                Level::WARN => Yellow.paint("WARN"),
                Level::ERROR => Red.paint("ERROR"),
            }
        )?;
        write!(writer, "{}", DarkGray.paint("]: "))?;

        // Write the event message
        ctx.field_format().format_fields(writer.by_ref(), event)?;

        writeln!(writer)
    }
}
