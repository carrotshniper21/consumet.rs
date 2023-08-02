use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::config::Appender;
use log4rs::config::Config;
use log4rs::config::Logger;
use log4rs::config::Root;
use log4rs::encode::pattern::PatternEncoder;

/// Initialize logging factilities.
pub fn initialize() {
    let conf = Config::builder()
        .appender(
            Appender::builder().build(
                "stdout",
                Box::new(
                    ConsoleAppender::builder()
                        .encoder(Box::new(PatternEncoder::new(
                            "{h([{d(%Y-%m-%d %H:%M:%S %Z)(local):>30}] [{l:>6}] [{M}]  {m})}{n}",
                        )))
                        .build(),
                ),
            ),
        )
        .logger(Logger::builder().build("h2", LevelFilter::Info))
        .logger(Logger::builder().build("hyper", LevelFilter::Info))
        .logger(Logger::builder().build("mio", LevelFilter::Info))
        .logger(Logger::builder().build("rdkafka", LevelFilter::Info))
        .logger(Logger::builder().build("rustls", LevelFilter::Info))
        .logger(Logger::builder().build("scylla", LevelFilter::Info))
        .logger(Logger::builder().build("tokio_tungstenite", LevelFilter::Info))
        .logger(Logger::builder().build("trust_dns_proto", LevelFilter::Info))
        .logger(Logger::builder().build("trust_dns_resolver", LevelFilter::Info))
        .logger(Logger::builder().build("tungstenite", LevelFilter::Info))
        .logger(Logger::builder().build("twilight_gateway", LevelFilter::Info))
        .logger(Logger::builder().build("twilight_http", LevelFilter::Info))
        .logger(Logger::builder().build("twilight_model", LevelFilter::Info))
        .logger(Logger::builder().build("want", LevelFilter::Info))
        .build(Root::builder().appender("stdout").build(LevelFilter::Debug))
        .expect("failed to build log4rs configuration");

    log4rs::init_config(conf).expect("failed to initialize log4rs");
}
