use actix_web::{HttpResponse, web};

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct User {
    id: u32,
    name: String,
}

#[allow(dead_code)]
pub async fn handler(
  some_data: web::Data<String>,
  req_data: web::ReqData<User>
) -> HttpResponse {
  // some code
  println!("{:?}", some_data);
  println!("{:?}", req_data);
  HttpResponse::Ok().finish()
}

#[cfg(test)]
mod tests {

    use actix_web::{test, FromRequest, HttpMessage, web::ReqData};

    use super::*;
    #[actix_rt::test]
    async fn test_fn() {
    let http_request = test::TestRequest::default().to_http_request();
    http_request.extensions_mut().insert(User{id: 1, name: "User1".to_string()});
    let _ = handler(
        web::Data::new("Dummy data".to_string()),
        ReqData::extract(&http_request).await.unwrap()
    ).await;
    }
}