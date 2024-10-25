use tokio::sync::{OnceCell};
use log::info;
use tokio::task::JoinHandle;

pub struct Server {
    #[allow(dead_code)]
    server_handle: JoinHandle<()>,
}

impl Server {
    pub async fn start() -> Self {
        let server_handle = tokio::spawn(async move {
            dotenv::dotenv().ok();
            env_logger::init_from_env(env_logger::Env::default()
                .filter_or("RUST_LOG", "debug"));
            let settings = vortex::Settings::new().unwrap();

            info!("Starting vortex test server at {}", &settings.http_host);
            signaling::server::launch(&settings.http_host).
                await
                .expect("Failed to launch server");
        });
        Server {
            server_handle,
        }
    }
}

pub(crate) static TEST_SERVER_ONCE: OnceCell<Server> = OnceCell::const_new();

pub(crate) async fn init() {
    TEST_SERVER_ONCE.get_or_init(Server::start).await;
}
