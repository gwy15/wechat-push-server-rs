pub fn init_logger() {
    log4rs::init_file("config/log4rs.yml", Default::default()).unwrap();
    log::info!("logging initialized");
}
