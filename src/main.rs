use fast_qr::{
    convert::{image::ImageBuilder, svg::SvgBuilder},
    QRBuilder,
};
use serde::Deserialize;
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use axum::{
    extract::Query,
    http::{header, StatusCode},
    response::IntoResponse,
    routing::get,
    Router,
};
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    addr: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_env("QRCODE_HTTP_LOG").unwrap_or_else(|_| {
                "qrcode_http=debug,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let args = Args::parse();

    let addr: SocketAddr = match args.addr.parse() {
        Ok(addr) => addr,
        Err(err) => {
            eprintln!("failed to parse address; err={:?}", err);
            std::process::exit(-1);
        }
    };

    let app = Router::new()
        .route("/", get(generate_qrcode))
        .layer(TraceLayer::new_for_http());

    tracing::info!("Listening on {addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Deserialize)]
struct GenerateQrCodeQueryArgs {
    code: String,
    format: Option<String>,
    width: Option<u32>,
}

enum Format {
    Png,
    Svg,
    Txt,
}

impl TryFrom<String> for Format {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "png" => Ok(Format::Png),
            "svg" => Ok(Format::Svg),
            "txt" => Ok(Format::Txt),
            _ => Err("Invalid format."),
        }
    }
}

async fn generate_qrcode(args: Query<GenerateQrCodeQueryArgs>) -> impl IntoResponse {
    let default_format = String::from("png");

    let format = Format::try_from(args.format.clone().unwrap_or(default_format));
    if let Err(err) = format {
        return (
            StatusCode::BAD_REQUEST,
            [(header::CONTENT_TYPE, "text/plain")],
            err.as_bytes().to_vec(),
        );
    }

    let qrcode = QRBuilder::new(args.code.clone()).build().unwrap();

    let (content_type, data) = match format.unwrap() {
        Format::Png => {
            let default_width: u32 = 600;
            let encoded = ImageBuilder::default()
                .fit_width(args.width.unwrap_or(default_width))
                .to_pixmap(&qrcode)
                .encode_png()
                .unwrap();
            ("image/png", encoded)
        }
        Format::Svg => {
            let encoded = SvgBuilder::default().to_str(&qrcode);
            ("image/svg+xml", encoded.as_bytes().to_vec())
        }
        Format::Txt => ("text/plain", qrcode.to_str().as_bytes().to_vec()),
    };

    (StatusCode::OK, [(header::CONTENT_TYPE, content_type)], data)
}
