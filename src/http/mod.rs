mod qr;
mod redirect;
mod snippets;

use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::common::state::AppState;

// Utoipa Nesting Support: https://github.com/juhaku/utoipa/pull/930
#[derive(OpenApi)]
#[openapi(
    paths(
        // Snippets
        snippets::create_snippet,
        snippets::get_snippet,
        snippets::get_snippets,
        snippets::patch_snippet,
        snippets::archive_snippet,
        // Redirect
        redirect::redirect,
        // QR
        qr::generate_qr,
    ),
)]
struct ApiDoc;

impl ApiDoc {
    pub fn new() -> utoipa::openapi::OpenApi {
        let mut doc = Self::openapi();
        doc.servers = Some(vec![utoipa::openapi::Server::new(
            crate::common::utils::get_api_base_url(),
        )]);
        doc
    }
}

pub fn build_router() -> axum::Router<AppState> {
    axum::Router::new()
        .merge(setup_swagger_ui())
        .merge(redirect::declare_routes())
        .merge(qr::declare_routes())
        .merge(snippets::declare_routes())
}

fn setup_swagger_ui() -> SwaggerUi {
    let config = utoipa_swagger_ui::Config::new(["/swagger.json"])
        .try_it_out_enabled(true)
        .persist_authorization(true);

    SwaggerUi::new("/swagger")
        .config(config)
        .url("/swagger.json", ApiDoc::new())
}
