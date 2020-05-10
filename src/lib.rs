use actix_web::{dev::ServiceResponse, test};
use actix_service::Service;


/// Response body holding body as String and status
pub struct RespBody {
    pub status: actix_web::http::StatusCode,
    pub body: String,
}

/// Get method to test Actix server
/// ```rust
/// use actix_web::{dev::ServiceResponse, web, Responder, test};
/// use actix_service::Service;
///
/// async fn index(info: web::Path<(u32, String)>) -> impl Responder {
///     format!("Hello {}! id:{}", info.1, info.0)
/// }
/// 
/// #[actix_rt::test]
/// async fn test_minimal() {
///     let mut app = test::init_service(App::new().service(index)).await;
/// 
///     let resp = get(&mut app, "/32/Filip").await;
///     assert_eq!(resp.status.as_u16(), 200);
///     assert_eq!(resp.body, "Hello Filip! id:32");
/// }
/// ```
pub async fn get<S, B, E>(mut app: &mut S, url: &str) -> RespBody
where
    B: actix_http::body::MessageBody,
    S: Service<Request = actix_http::Request, Response = ServiceResponse<B>, Error = E>,
    E: std::fmt::Debug,
{
    let req = test::TestRequest::get().uri(url).to_request();
    let resp = test::call_service(&mut app, req).await;
    let status = resp.status().clone();
    let body = test::read_body(resp).await;
    let body = String::from_utf8_lossy(&body).to_string();

    RespBody {
        status,
        body,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, get, web, App, Responder};

    #[get("/{id}/{name}")]
    async fn index(info: web::Path<(u32, String)>) -> impl Responder {
        format!("Hello {}! id:{}", info.1, info.0)
    }

    #[actix_rt::test]
    async fn test_minimal() {
        let mut app = test::init_service(App::new().service(index)).await;

        let resp = get(&mut app, "/32/Filip").await;
        assert_eq!(resp.status.as_u16(), 200);
        assert_eq!(resp.body, "Hello Filip! id:32");
    }

    #[actix_rt::test]
    async fn test_fail() {
        let mut app = test::init_service(App::new().service(index)).await;

        let resp = get(&mut app, "/32/Filip/Not").await;
        assert_eq!(resp.status.as_u16(), 404);
        assert_eq!(resp.body, "");
    }
}
