use actix_web::{App, HttpServer, web::Data};
use sqlx::PgPool;

mod appserve;
use appserve::fonction::add;
mod component;

#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    // Définir directement l'URL de la base de données
    let database_url = "postgres://fossouo:fowama2006@localhost:5432/fullstack";

    // Initialiser le pool de connexion
    let pool = PgPool::connect(database_url)
        .await
        .expect("Failed to connect to the database");

    // Appliquer les migrations
   // sqlx::migrate!("./migrations")
       // .run(&pool)
       // .await
     //   .expect("Failed to run migrations");

    // Démarrer le serveur HTTP
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .service(add) // Ajouter le gestionnaire de route
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

#[cfg(not(any(feature = "ssr", feature = "csr")))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
    // see optional feature `csr` instead
}

[cfg(all(not(feature = "ssr"), feature = "csr"))]
pub fn main() {
    // a client-side main function is required for using `trunk serve`
    // prefer using `cargo leptos serve` instead
    // to run: `trunk serve --open --features csr`
    use desesperer::component;

    console_error_panic_hook::set_once();

    leptos::mount_to_body(App);
}