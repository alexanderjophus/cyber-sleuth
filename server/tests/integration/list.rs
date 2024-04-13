use crate::helpers::TestApi;
use pavex::http::StatusCode;

#[tokio::test]
async fn list_works() {
    let api = TestApi::spawn().await;

    let response = api.list_digimon().await;

    assert_eq!(response.status(), StatusCode::OK);
}
