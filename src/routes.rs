//! Place all Actix routes here, multiple route configs can be used and
//! combined.

use crate::handlers::{
    auth::{login, logout, check_auth},
    health::get_health,
    user::{create_user, delete_user, get_user, get_users, update_user},
    cliente::{create_cliente, delete_cliente, get_cliente, get_clientes, update_cliente},
    produto::{create_produto, delete_produto, get_produto, get_produtos, update_produto},
    orcamento::{create_orcamento, delete_orcamento, get_orcamento},
};
use crate::middleware::auth::Auth as AuthMiddleware;
use actix_files::Files;
use actix_web::web;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
        // Healthcheck
        .route("/health", web::get().to(get_health))
        // /api/v1 routes
        .service(
            web::scope("/api/v1")
            .service(
                web::scope("/auth")
                    .route("/login", web::post().to(login))
                    .route("/logout", web::get().to(logout)),
               )       
                // Lock down routes with AUTH Middleware
                .wrap(AuthMiddleware)
                // AUTH routes
                .route("/check-auth", web::post().to(check_auth))

                // USER routes
                .service(
                    web::scope("/user")
                        .route("/{id}", web::get().to(get_user))
                        .route("/{id}", web::put().to(update_user))
                        .route("/{id}", web::delete().to(delete_user))
                        .route("", web::get().to(get_users))
                        .route("", web::post().to(create_user)),
                )
                // CLIENTE routes
                .service(
                    web::scope("/cliente")
                        .route("/{id}", web::get().to(get_cliente))
                        .route("/{id}", web::put().to(update_cliente))
                        .route("/{id}", web::delete().to(delete_cliente))
                        .route("", web::get().to(get_clientes))
                        .route("", web::post().to(create_cliente)),
                )
                // PRODUTO routes
                .service(
                    web::scope("/produto")
                        .route("/{id}", web::get().to(get_produto))
                        .route("/{id}", web::put().to(update_produto))
                        .route("/{id}", web::delete().to(delete_produto))
                        .route("", web::get().to(get_produtos))
                        .route("", web::post().to(create_produto)),
                )
                // ORCAMENTO routes
                .service(
                    web::scope("/orcamento")
                        .route("/{id}", web::get().to(get_orcamento))
                        .route("/{id}", web::delete().to(delete_orcamento))
                        .route("", web::post().to(create_orcamento)),
                ),

        )
        // Serve secure static files from the static-private folder
        .service(
            web::scope("/secure").wrap(AuthMiddleware).service(
                Files::new("", "./static-secure")
                    .index_file("index.html")
                    .use_last_modified(true),
            ),
        )
        // Serve public static files from the static folder
        .service(
            web::scope("").default_service(
                Files::new("", "./static")
                    .index_file("index.html")
                    .use_last_modified(true),
            ),
        );
}
