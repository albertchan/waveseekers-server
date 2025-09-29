use actix_web::web::Data;
use actix_web::{App, HttpServer, dev::Server, web};
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

use crate::datafeed::HistoricalData;
use crate::routes::{get_datafeed_config, get_historical_data, symbols_handler};
use crate::storage::Database;
use crate::{configuration::Settings, routes::heartbeat};

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(config: &Settings) -> Result<Self, anyhow::Error> {
        let address = format!("{}:{}", config.application.host, config.application.port);
        let database = Database::new(&config.database.url);
        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr().unwrap().port();
        let server = run(listener, database).await?;

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

pub async fn run(listener: TcpListener, database: Database) -> Result<Server, anyhow::Error> {
    let historical_data = Data::new(HistoricalData::new(database.clone()));
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/", web::get().to(heartbeat))
            .service(
                web::scope("/api/v1")
                    .route("/", web::get().to(heartbeat))
                    .route("/health", web::get().to(heartbeat))
                    .route("/data/config", web::get().to(get_datafeed_config))
                    .route("/data/historical", web::get().to(get_historical_data))
                    .route("/symbols", web::get().to(symbols_handler))
            )
            .app_data(historical_data.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
