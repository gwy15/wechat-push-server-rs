use chrono::Local;
use env_logger::Env;
use log::info;
use std::io::Write;

pub fn init_logger() {
    let env = Env::default().default_filter_or("info");
    env_logger::Builder::from_env(env)
        .format(|buf, record| {
            writeln!(
                buf,
                "({t}) {level} [{module}:{line}] {args}",
                t = Local::now().format("%Y-%m-%d %H:%M:%S"),
                level = buf.default_styled_level(record.level()),
                module = record.module_path().unwrap_or("<unknown>"),
                line = record.line().unwrap_or(0),
                args = &record.args()
            )
        })
        .init();
    info!("env logger inited");
}
