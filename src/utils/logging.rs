use tracing_subscriber::{EnvFilter, fmt};

pub fn init_tracing() {
    let subscriber = fmt::Subscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to initialize tracing subscriber")
}
