use axum::{middleware::Next, response::Response, http::Request};
use sysinfo::{System, SystemExt};

pub async fn memory_cleaner<B>(req: Request<B>, next: Next<B>) -> Response {
    let mut sys = System::new();
    
    let response = next.run(req).await;
    
    // Liberar memoria después de cada request
    sys.refresh_memory();
    if sys.used_memory() > 50 * 1024 * 1024 { // Si usa más de 50MB
        tokio::task::spawn_blocking(|| {
            System::new_all().refresh_all();
        });
    }
    
    response
}
