use actix_web::middleware::Logger;
use env_logger::Env;
use log::LevelFilter;

pub fn init_logger() {
  env_logger::Builder::from_env(Env::default().default_filter_or("info"))
    .format(|buf, record| {
      use chrono::Local;
      use std::io::Write;
      writeln!(
        buf,
        "{} [{}] - {}",
        Local::now().format("%Y-%m-%d %H:%M:%S"),
        record.level(),
        record.args()
      )
    })
    .filter(None, LevelFilter::Info)
    .init();
}

pub fn get_logger_middleware() -> Logger {
  Logger::new("%a '%r' %s %b '%{Referer}i' '%{User-Agent}i' %T")
}
