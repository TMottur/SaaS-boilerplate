use axum::{extract::FromRequestParts};
use http::request::Parts;
use serde::{Deserialize, Serialize};
use tower_sessions::Session;
use chrono::{DateTime, Utc, Duration};

const COUNTER_KEY: &str = "counter";
const WINDOW_START_KEY: &str = "window_start";
const WINDOW_SECONDS: i64 = 30;
const MAX_REQUESTS: usize = 5;

#[derive(Default, Deserialize, Serialize)]
pub struct Counter(pub usize);

pub struct RateLimit;

impl<S> FromRequestParts<S> for RateLimit
where
    S: Send + Sync,
{
    type Rejection = (http::StatusCode, &'static str);

    async fn from_request_parts(req: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let session = Session::from_request_parts(req, state).await
            .map_err(|_| (http::StatusCode::INTERNAL_SERVER_ERROR, "Failed to extract session"))?;

        let now = Utc::now();

        let counter: usize = session.get(COUNTER_KEY).await.unwrap_or(Some(0)).unwrap_or(0);
        let window_start: DateTime<Utc> = session.get(WINDOW_START_KEY).await.unwrap_or(None).unwrap_or(now);

        if now - window_start > Duration::seconds(WINDOW_SECONDS) {
            // Reset window
            session.insert(COUNTER_KEY, 1).await.map_err(|_| (http::StatusCode::INTERNAL_SERVER_ERROR, "Session update failed"))?;
            session.insert(WINDOW_START_KEY, now).await.map_err(|_| (http::StatusCode::INTERNAL_SERVER_ERROR, "Session update failed"))?;
            return Ok(RateLimit);
        }

        if counter >= MAX_REQUESTS {
            return Err((http::StatusCode::TOO_MANY_REQUESTS, "Rate limit exceeded"));
        }

        session.insert(COUNTER_KEY, counter + 1).await
            .map_err(|_| (http::StatusCode::INTERNAL_SERVER_ERROR, "Session update failed"))?;

        Ok(RateLimit)
    }
}

