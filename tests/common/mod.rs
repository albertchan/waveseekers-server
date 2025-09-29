use waveseekers::application::Application;
use waveseekers::configuration::get_config;

#[allow(dead_code)]
pub struct TestApp {
    pub address: String,
    port: u16,
}

pub async fn spawn_app() -> TestApp {
    let config = {
        let mut conf = get_config().expect("Failed to read configuration.");
        // use random OS port
        conf.application.port = 0;
        conf
    };
    let application = Application::build(&config)
        .await
        .expect("Failed to build application");
    let application_port = application.port();
    let _ = tokio::spawn(application.run_until_stopped());
    let test_app = TestApp {
        address: format!("http://localhost:{}", application_port),
        port: application_port,
    };

    test_app
}
