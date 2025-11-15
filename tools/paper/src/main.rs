use axum::{Router, middleware::map_response, response::Response};

mod api;
mod error;
mod handlers;
mod response;

pub use self::error::{Error, Result};
pub use self::response::{Created, EmptyOK};

#[tokio::main]
async fn main() -> Result<()> {
    let app = Router::new()
        .nest("/trader/v1", api::router())
        .layer(map_response(main_response_mapper));

    const HOST: &str = "127.0.0.1";
    const PORT: u16 = 9000;
    let addr = format!("{HOST}:{PORT}");
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .map_err(|err| format!("Cannot start TcpListener. \nCause: {err}"))?;

    println!("->> LISTENING on {:?}\n", listener.local_addr().unwrap());

    axum::serve(listener, app)
        .await
        .map_err(|err| format!("Cannot start axum::serve. \nCause:{err}"))?;

    Ok(())
}

async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");
    println!();
    res
}
