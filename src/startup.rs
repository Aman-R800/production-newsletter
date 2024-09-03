
use std::net::TcpListener;
use std::time::Duration;

use actix_web::{dev::Server, web, App, HttpServer};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use secrecy::ExposeSecret;
use tracing_actix_web::TracingLogger;
use crate::configuration::{DatabaseSettings, Settings};
use crate::email_client::EmailClient;
use crate::routes::health_check::health_check;
use crate::routes::subscribe::subscribe;

// pub struct Application {
//     port: u16,
//     server: Server
// }
// 
// impl Application {
//     pub async fn build(config: Settings) -> Result<Application, std::io::Error> {
//         let connection_pool = get_connection_pool(&config.database);
//         let sender_email = config.email_client.sender().expect("Failed to get valid sender email");
//         let email_client = EmailClient::new(config.email_client.base_url, sender_email, secrecy::Secret::new(Faker.fake()), config.email_client.timeout);
// 
//         let address = format!( "{}:{}",
//             config.application.host, config.application.port
//         );
// 
//         let listener = TcpListener::bind(&address)?;
//         let port = listener.local_addr().unwrap().port();
//         let server = run(listener, connection_pool, email_client)?;
// 
//         Ok(Self{port, server})
//     }
// 
//     pub fn port(&self) -> u16 {
//         self.port
//     }
// 
//     pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
//         self.server.await
//     }
// }

pub fn get_connection_pool(config: &DatabaseSettings) -> Pool<ConnectionManager<PgConnection>> {
    let manager = ConnectionManager::<PgConnection>::new(&*config.connection_string().expose_secret());
    Pool::builder()
        .test_on_check_out(true)
        .connection_timeout(Duration::from_secs(5))
        .build_unchecked(manager)
}

pub async fn build(config: &Settings) -> Result<Server, std::io::Error> {
    let address = format!("{}:{}", config.application.host, config.application.port);
    let listener = TcpListener::bind(&address)?;

    let connection_pool = get_connection_pool(&config.database);
    let sender_email = config.email_client.sender().expect("Failed to get valid sender email");
    let email_client = EmailClient::new(config.email_client.base_url.clone(), sender_email, config.email_client.authorization_token.clone(), config.email_client.timeout);
    run(listener, connection_pool, email_client)
}

pub fn run(listener: TcpListener, connection_pool: Pool<ConnectionManager<PgConnection>>, email_client: EmailClient) -> Result<Server, std::io::Error>{

    let connection_pool = web::Data::new(connection_pool);
    let email_client = web::Data::new(email_client);

    let server = HttpServer::new(move || { 
        App::new()
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(connection_pool.clone())
            .app_data(email_client.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
