use std::net::TcpListener;
use std::time::Duration;

use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use newsletter::startup::run;
use newsletter::configuration::get_configuration;
use newsletter::telemetry::{get_subscriber, init_subscriber};
use secrecy::ExposeSecret;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("Newsletter".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let config = get_configuration().expect("Failed to get configuration");
    let address = format!("{}:{}", config.application.host, config.application.port);
    let listener = TcpListener::bind(&address)?;
    println!("Host: {}", config.application.host);

    let manager = ConnectionManager::<PgConnection>::new(&*config.database.connection_string().expose_secret());
    let connection_pool = Pool::builder()
                            .test_on_check_out(true)
                            .connection_timeout(Duration::from_secs(5))
                            .build_unchecked(manager);

    run(listener, connection_pool)?.await
}
