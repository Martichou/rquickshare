use std::fs::File;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::time::SystemTime;

use fern::colors::{Color, ColoredLevelConfig};
use time::OffsetDateTime;

pub fn set_up_logging() -> Result<(), anyhow::Error> {
    // Use log from ENV if defined, otherwise use Info/Trace depending on the build
    let default_level = match std::env::var("RQS_LOG") {
        Ok(r) => log::Level::from_str(&r)
            .unwrap_or(log::Level::Debug)
            .to_level_filter(),
        Err(_) => {
            if cfg!(debug_assertions) {
                log::LevelFilter::Trace
            } else {
                log::LevelFilter::Info
            }
        }
    };

    let colors = ColoredLevelConfig::new()
        .error(Color::Red)
        .warn(Color::Yellow)
        .info(Color::Green)
        .debug(Color::Blue)
        .trace(Color::Cyan);

    let dispatch = fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "\x1B[2m{date}\x1b[0m {level: >5} \x1B[2m{target}:\x1b[0m {message}",
                date = humantime::format_rfc3339_seconds(SystemTime::now()),
                target = record.target(),
                level = colors.color(record.level()),
                message = message,
            ));
        })
        .level(default_level)
        .level_for("mdns_sd", log::LevelFilter::Error)
        .level_for("polling", log::LevelFilter::Error)
        .level_for("neli", log::LevelFilter::Error)
        .level_for("bluez_async", log::LevelFilter::Error)
        .level_for("bluer", log::LevelFilter::Error)
        .level_for("async_io", log::LevelFilter::Error)
        .level_for("polling", log::LevelFilter::Error)
        .chain(std::io::stdout());

    if let Some(base_dirs) = directories::BaseDirs::new() {
        let mut path = base_dirs.config_dir().to_path_buf();
        path.push("dev.mandre.rquickshare.ng");
        path.push("logs");
        if !path.exists() {
            std::fs::create_dir_all(&path)?;
        }

        let app_name = &get_program_name();
        let file_logger = fern::log_file(get_log_file_path(&path, app_name, 40000)?)?;

        dispatch.chain(file_logger).apply()?;
    } else {
        dispatch.apply()?;
    }

    debug!("Finished setting up logging! yay!");
    Ok(())
}

fn get_log_file_path(
    dir: &impl AsRef<Path>,
    file_name: &str,
    max_file_size: u128,
) -> Result<PathBuf, anyhow::Error> {
    let path = dir.as_ref().join(format!("{file_name}.log"));

    if path.exists() {
        let log_size = File::open(&path)?.metadata()?.len() as u128;
        if log_size > max_file_size {
            let to = dir.as_ref().join(format!(
                "{}_{}.log",
                file_name,
                OffsetDateTime::now_utc()
                    .format(
                        &time::format_description::parse(
                            "[year]-[month]-[day]_[hour]-[minute]-[second]"
                        )
                        .unwrap()
                    )
                    .unwrap(),
            ));

            if to.is_file() {
                let mut to_bak = to.clone();
                to_bak.set_file_name(format!(
                    "{}.bak",
                    to_bak.file_name().unwrap().to_string_lossy()
                ));
                std::fs::rename(&to, to_bak)?;
            }

            std::fs::rename(&path, to)?;
        }
    }

    Ok(path)
}

// Currently hardcoded
fn get_program_name() -> String {
    String::from("RQuickShare")
}
