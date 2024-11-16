mod auth;
mod qr;
mod redirect;
mod snippets;
mod users;

use std::collections::BTreeMap;
use utoipa::openapi::security::{Http, HttpAuthScheme, SecurityScheme};
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
        // Auth
        auth::login,
        // Users
        users::create_user,
        users::get_user,
        users::get_users,
        users::delete_user,
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

        let mut security_schemes = BTreeMap::new();
        security_schemes.insert(
            "bearerAuth".to_string(),
            SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)),
        );
        let mut components = doc.components.unwrap_or_default();
        components.security_schemes = security_schemes;
        doc.components = Some(components);

        doc
    }
}

pub fn build_router(state: AppState) -> axum::Router<AppState> {
    axum::Router::new()
        .merge(auth::declare_routes())
        .merge(setup_swagger_ui())
        .merge(redirect::declare_routes())
        .merge(qr::declare_routes())
        .merge(snippets::declare_routes())
        .merge(users::declare_routes(state))
}

fn setup_swagger_ui() -> SwaggerUi {
    let config = utoipa_swagger_ui::Config::new(["/swagger.json"])
        .try_it_out_enabled(true)
        .persist_authorization(true);

    SwaggerUi::new("/swagger")
        .config(config)
        .url("/swagger.json", ApiDoc::new())
}
