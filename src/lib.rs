use actix_web::{dev::ServiceResponse, test};
use actix_service::Service;


/// Response body holding body as String and status
pub struct RespBody {
    pub status: actix_web::http::StatusCode,
    pub body: String,
}

/// Get test method for Actix server
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
pub async fn get<'a, SERVICE, BODY, E>(app: &'a mut SERVICE, url: &'a str) -> RespBody
where
    BODY: actix_http::body::MessageBody,
    E: std::fmt::Debug,
    SERVICE: Service<Request = actix_http::Request, Response = ServiceResponse<BODY>, Error = E>,
{
    let req = test::TestRequest::get().uri(url).to_request();
    let resp = test::call_service(app, req).await;
    let status = resp.status().clone();
    let body = test::read_body(resp).await;
    let body = String::from_utf8_lossy(&body).to_string();

    RespBody {
        status,
        body,
    }
}

/// Post test method for Actix server
///
/// ```rust
/// use actix_web::{dev::ServiceResponse, post, web, Responder, test};
/// use actix_service::Service;
/// use serde::{Serialize, Deserialize};
///
/// #[derive(Serialize, Deserialize)]
/// struct User {
///     name: String, 
/// }
/// 
/// #[post("/api/users")]
/// async fn post_user(
///     user: web::Json<User>,
/// ) -> impl Responder {
///     format!("Name is: {}!", user.name)
/// }
/// 
/// #[actix_rt::test]
/// async fn test_minimal() {
///     let mut app = test::init_service(App::new().service(post_users)).await;
/// 
///     let user = User { name: "Filip".to_string() };
///     let resp = post_json(&mut app, user, "/api/users").await;
///     assert_eq!(resp.status.as_u16(), 200);
///     assert_eq!(resp.body, "Name is: Filip!");
/// }
/// ```
pub async fn post_json<'a, SERVICE, BODY, SERDE, E>(app: &'a mut SERVICE, json: SERDE, url: &'a str) -> RespBody
where
    BODY: actix_http::body::MessageBody,
    SERVICE: Service<Request = actix_http::Request, Response = ServiceResponse<BODY>, Error = E>,
    SERDE: serde::ser::Serialize,
    E: std::fmt::Debug,
{
    let req = test::TestRequest::post()
        .set_json(&json)
        .uri(url).to_request();
    let resp = test::call_service(app, req).await;
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
    use actix_web::{test, get, post, web, App, Responder};
    use serde::{Serialize, Deserialize};

    #[get("/{id}/{name}")]
    async fn index(info: web::Path<(u32, String)>) -> impl Responder {
        format!("Hello {}! id:{}", info.1, info.0)
    }

    #[derive(Serialize, Deserialize)]
    struct User {
        name: String, 
    }

    #[post("/api/users")]
    async fn post_user(
        user: web::Json<User>,
    ) -> impl Responder {
        format!("Name is: {}!", user.name)
    }

    #[actix_rt::test]
    async fn test_get_minimal() {
        let mut app = test::init_service(App::new().service(index)).await;

        let resp = get(&mut app, "/32/Filip").await;
        assert_eq!(resp.status.as_u16(), 200);
        assert_eq!(resp.body, "Hello Filip! id:32");
    }

    #[actix_rt::test]
    async fn test_get_fail() {
        let mut app = test::init_service(App::new().service(index)).await;

        let resp = get(&mut app, "/32/Filip/Not").await;
        assert_eq!(resp.status.as_u16(), 404);
        assert_eq!(resp.body, "");
    }

    #[actix_rt::test]
    async fn test_post() {
        let mut app = test::init_service(App::new().service(post_user)).await;

        let user = User { name: "Filip".to_string() };
        let resp = post_json(&mut app, user, "/api/users").await;
        assert_eq!(resp.status.as_u16(), 200);
        assert_eq!(resp.body, "Name is: Filip!");
    }
}
