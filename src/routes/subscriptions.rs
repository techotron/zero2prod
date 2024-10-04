use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;
use tracing;
use tracing_futures::Instrument;

// This macro automatically implements the correct serialise/deserialise implementation for us
//  In our case, it'll parse the definition of the FormData type and generate the right implementation for it
#[derive(serde::Deserialize)]
pub struct FormData {
    pub email: String,
    pub name: String,
}

pub async fn subscribe(
    form: web::Form<FormData>,
    // Retrieving a pool from the application state!
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let request_id = Uuid::new_v4();
    // Spans, like logs, have an associated level
    // `info_span` creates a span at the info-level
    let request_span = tracing::info_span!(
        "Adding a new subscriber.",
        // This is implicitly providing a key=value pair by using the variable name as the key
        // The % is used to tell tracing to use their Display implementation for logging purposes.
        %request_id,
        subscriber_email = %form.email,
        subscriber_name = %form.name
    );
    
    // _request_span_guard is dropped at the end of `subscribe`. That's when we "exit" the span.
    let _request_span_guard = request_span.enter();
    
    // We do not call `.enter` on query_span!
    // `.instrument` takes care of it at the right moments
    // in the query future lifetime.
    let query_span = tracing::info_span!(
        "Saving new subscriber details in the database"
    );

    // `Result` has two variants: `Ok` and `Err`.
    // The first for successes, the second for failures.
    // We use a `match` statement to choose what to do based on the outcome.
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
        // We use `get_ref` to get an immutable reference to the `PgConnection` wrapped by `web::Data`.
        .execute(pool.get_ref())
        // This passes the span to the future and we'll see "query_span" info log every time the executor polls the future
        .instrument(query_span)
        .await
    {
        Ok(_) => {
            HttpResponse::Ok().finish()
        },
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
