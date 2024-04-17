//! Do NOT edit this code.
//! It was automatically generated by Pavex.
//! All manual edits will be lost next time the code is generated.
extern crate alloc;
struct ServerState {
    router: pavex_matchit::Router<u32>,
    #[allow(dead_code)]
    application_state: ApplicationState,
}
pub struct ApplicationState {}
pub async fn build_application_state() -> crate::ApplicationState {
    crate::ApplicationState {}
}
pub fn run(
    server_builder: pavex::server::Server,
    application_state: ApplicationState,
) -> pavex::server::ServerHandle {
    let server_state = std::sync::Arc::new(ServerState {
        router: build_router(),
        application_state,
    });
    server_builder.serve(route_request, server_state)
}
fn build_router() -> pavex_matchit::Router<u32> {
    let mut router = pavex_matchit::Router::new();
    router.insert("/api/digimon", 0u32).unwrap();
    router.insert("/api/digimon/:name", 1u32).unwrap();
    router
}
async fn route_request(
    request: http::Request<hyper::body::Incoming>,
    _connection_info: Option<pavex::connection::ConnectionInfo>,
    server_state: std::sync::Arc<ServerState>,
) -> pavex::response::Response {
    let (request_head, request_body) = request.into_parts();
    #[allow(unused)]
    let request_body = pavex::request::body::RawIncomingBody::from(request_body);
    let request_head: pavex::request::RequestHead = request_head.into();
    let matched_route = match server_state.router.at(&request_head.target.path()) {
        Ok(m) => m,
        Err(_) => {
            let allowed_methods: pavex::router::AllowedMethods = pavex::router::MethodAllowList::from_iter(
                    vec![],
                )
                .into();
            let matched_route_template = pavex::request::path::MatchedPathPattern::new(
                "*",
            );
            return route_0::entrypoint(
                    &request_head,
                    matched_route_template,
                    &allowed_methods,
                )
                .await;
        }
    };
    let route_id = matched_route.value;
    #[allow(unused)]
    let url_params: pavex::request::path::RawPathParams<'_, '_> = matched_route
        .params
        .into();
    match route_id {
        0u32 => {
            let matched_route_template = pavex::request::path::MatchedPathPattern::new(
                "/api/digimon",
            );
            match &request_head.method {
                &pavex::http::Method::GET => {
                    route_1::entrypoint(matched_route_template, &request_head).await
                }
                _ => {
                    let allowed_methods: pavex::router::AllowedMethods = pavex::router::MethodAllowList::from_iter([
                            pavex::http::Method::GET,
                        ])
                        .into();
                    route_0::entrypoint(
                            &request_head,
                            matched_route_template,
                            &allowed_methods,
                        )
                        .await
                }
            }
        }
        1u32 => {
            let matched_route_template = pavex::request::path::MatchedPathPattern::new(
                "/api/digimon/:name",
            );
            match &request_head.method {
                &pavex::http::Method::GET => {
                    route_2::entrypoint(
                            url_params,
                            matched_route_template,
                            &request_head,
                        )
                        .await
                }
                _ => {
                    let allowed_methods: pavex::router::AllowedMethods = pavex::router::MethodAllowList::from_iter([
                            pavex::http::Method::GET,
                        ])
                        .into();
                    route_0::entrypoint(
                            &request_head,
                            matched_route_template,
                            &allowed_methods,
                        )
                        .await
                }
            }
        }
        i => unreachable!("Unknown route id: {}", i),
    }
}
pub mod route_0 {
    pub async fn entrypoint<'a, 'b>(
        s_0: &'a pavex::request::RequestHead,
        s_1: pavex::request::path::MatchedPathPattern,
        s_2: &'b pavex::router::AllowedMethods,
    ) -> pavex::response::Response {
        let response = wrapping_0(s_0, s_1, s_2).await;
        response
    }
    async fn stage_1<'a, 'b>(
        s_0: &'a pavex::router::AllowedMethods,
        s_1: pavex::request::path::MatchedPathPattern,
        s_2: &'b pavex::request::RequestHead,
    ) -> pavex::response::Response {
        let response = wrapping_1(s_2, s_1, s_0).await;
        response
    }
    async fn stage_2<'a, 'b>(
        s_0: &'a pavex::router::AllowedMethods,
        s_1: &'b pavex_tracing::RootSpan,
    ) -> pavex::response::Response {
        let response = handler(s_0).await;
        let response = post_processing_0(response, s_1).await;
        response
    }
    async fn wrapping_0(
        v0: &pavex::request::RequestHead,
        v1: pavex::request::path::MatchedPathPattern,
        v2: &pavex::router::AllowedMethods,
    ) -> pavex::response::Response {
        let v3 = crate::route_0::Next0 {
            s_0: v2,
            s_1: v1,
            s_2: v0,
            next: stage_1,
        };
        let v4 = pavex::middleware::Next::new(v3);
        let v5 = pavex::middleware::wrap_noop(v4).await;
        <pavex::response::Response as pavex::response::IntoResponse>::into_response(v5)
    }
    async fn wrapping_1(
        v0: &pavex::request::RequestHead,
        v1: pavex::request::path::MatchedPathPattern,
        v2: &pavex::router::AllowedMethods,
    ) -> pavex::response::Response {
        let v3 = pavex::telemetry::ServerRequestId::generate();
        let v4 = app::telemetry::root_span(v0, v1, v3);
        let v5 = crate::route_0::Next1 {
            s_0: v2,
            s_1: &v4,
            next: stage_2,
        };
        let v6 = pavex::middleware::Next::new(v5);
        let v7 = <pavex_tracing::RootSpan as core::clone::Clone>::clone(&v4);
        let v8 = pavex_tracing::logger(v7, v6).await;
        <pavex::response::Response as pavex::response::IntoResponse>::into_response(v8)
    }
    async fn handler(v0: &pavex::router::AllowedMethods) -> pavex::response::Response {
        let v1 = pavex::router::default_fallback(v0).await;
        <pavex::response::Response as pavex::response::IntoResponse>::into_response(v1)
    }
    async fn post_processing_0(
        v0: pavex::response::Response,
        v1: &pavex_tracing::RootSpan,
    ) -> pavex::response::Response {
        let v2 = app::telemetry::response_logger(v0, v1).await;
        <pavex::response::Response as pavex::response::IntoResponse>::into_response(v2)
    }
    struct Next0<'a, 'b, T>
    where
        T: std::future::Future<Output = pavex::response::Response>,
    {
        s_0: &'a pavex::router::AllowedMethods,
        s_1: pavex::request::path::MatchedPathPattern,
        s_2: &'b pavex::request::RequestHead,
        next: fn(
            &'a pavex::router::AllowedMethods,
            pavex::request::path::MatchedPathPattern,
            &'b pavex::request::RequestHead,
        ) -> T,
    }
    impl<'a, 'b, T> std::future::IntoFuture for Next0<'a, 'b, T>
    where
        T: std::future::Future<Output = pavex::response::Response>,
    {
        type Output = pavex::response::Response;
        type IntoFuture = T;
        fn into_future(self) -> Self::IntoFuture {
            (self.next)(self.s_0, self.s_1, self.s_2)
        }
    }
    struct Next1<'a, 'b, T>
    where
        T: std::future::Future<Output = pavex::response::Response>,
    {
        s_0: &'a pavex::router::AllowedMethods,
        s_1: &'b pavex_tracing::RootSpan,
        next: fn(&'a pavex::router::AllowedMethods, &'b pavex_tracing::RootSpan) -> T,
    }
    impl<'a, 'b, T> std::future::IntoFuture for Next1<'a, 'b, T>
    where
        T: std::future::Future<Output = pavex::response::Response>,
    {
        type Output = pavex::response::Response;
        type IntoFuture = T;
        fn into_future(self) -> Self::IntoFuture {
            (self.next)(self.s_0, self.s_1)
        }
    }
}
pub mod route_1 {
    pub async fn entrypoint<'a>(
        s_0: pavex::request::path::MatchedPathPattern,
        s_1: &'a pavex::request::RequestHead,
    ) -> pavex::response::Response {
        let response = wrapping_0(s_0, s_1).await;
        response
    }
    async fn stage_1<'a>(
        s_0: &'a pavex::request::RequestHead,
        s_1: pavex::request::path::MatchedPathPattern,
    ) -> pavex::response::Response {
        let response = wrapping_1(s_1, s_0).await;
        response
    }
    async fn stage_2<'a, 'b>(
        s_0: &'a pavex_tracing::RootSpan,
        s_1: &'b pavex::request::RequestHead,
    ) -> pavex::response::Response {
        let response = handler(s_0, s_1).await;
        let response = post_processing_0(response, s_0).await;
        response
    }
    async fn wrapping_0(
        v0: pavex::request::path::MatchedPathPattern,
        v1: &pavex::request::RequestHead,
    ) -> pavex::response::Response {
        let v2 = crate::route_1::Next0 {
            s_0: v1,
            s_1: v0,
            next: stage_1,
        };
        let v3 = pavex::middleware::Next::new(v2);
        let v4 = pavex::middleware::wrap_noop(v3).await;
        <pavex::response::Response as pavex::response::IntoResponse>::into_response(v4)
    }
    async fn wrapping_1(
        v0: pavex::request::path::MatchedPathPattern,
        v1: &pavex::request::RequestHead,
    ) -> pavex::response::Response {
        let v2 = pavex::telemetry::ServerRequestId::generate();
        let v3 = app::telemetry::root_span(v1, v0, v2);
        let v4 = crate::route_1::Next1 {
            s_0: &v3,
            s_1: v1,
            next: stage_2,
        };
        let v5 = pavex::middleware::Next::new(v4);
        let v6 = <pavex_tracing::RootSpan as core::clone::Clone>::clone(&v3);
        let v7 = pavex_tracing::logger(v6, v5).await;
        <pavex::response::Response as pavex::response::IntoResponse>::into_response(v7)
    }
    async fn handler(
        v0: &pavex_tracing::RootSpan,
        v1: &pavex::request::RequestHead,
    ) -> pavex::response::Response {
        let v2 = pavex::request::query::QueryParams::extract(v1);
        let v3 = match v2 {
            Ok(ok) => ok,
            Err(v3) => {
                return {
                    let v4 = pavex::request::query::errors::ExtractQueryParamsError::into_response(
                        &v3,
                    );
                    let v5 = pavex::Error::new(v3);
                    app::telemetry::error_logger(&v5, v0).await;
                    <pavex::response::Response as pavex::response::IntoResponse>::into_response(
                        v4,
                    )
                };
            }
        };
        let mut v4 = app::routes::Digimon::new();
        let v5 = app::routes::Digimon::search(&mut v4, &v3);
        let v6 = match v5 {
            Ok(ok) => ok,
            Err(v6) => {
                return {
                    let v7 = app::routes::SearchError::into_response(&v6);
                    let v8 = pavex::Error::new(v6);
                    app::telemetry::error_logger(&v8, v0).await;
                    <pavex::response::Response as pavex::response::IntoResponse>::into_response(
                        v7,
                    )
                };
            }
        };
        <pavex::response::Response as pavex::response::IntoResponse>::into_response(v6)
    }
    async fn post_processing_0(
        v0: pavex::response::Response,
        v1: &pavex_tracing::RootSpan,
    ) -> pavex::response::Response {
        let v2 = app::telemetry::response_logger(v0, v1).await;
        <pavex::response::Response as pavex::response::IntoResponse>::into_response(v2)
    }
    struct Next0<'a, T>
    where
        T: std::future::Future<Output = pavex::response::Response>,
    {
        s_0: &'a pavex::request::RequestHead,
        s_1: pavex::request::path::MatchedPathPattern,
        next: fn(
            &'a pavex::request::RequestHead,
            pavex::request::path::MatchedPathPattern,
        ) -> T,
    }
    impl<'a, T> std::future::IntoFuture for Next0<'a, T>
    where
        T: std::future::Future<Output = pavex::response::Response>,
    {
        type Output = pavex::response::Response;
        type IntoFuture = T;
        fn into_future(self) -> Self::IntoFuture {
            (self.next)(self.s_0, self.s_1)
        }
    }
    struct Next1<'a, 'b, T>
    where
        T: std::future::Future<Output = pavex::response::Response>,
    {
        s_0: &'a pavex_tracing::RootSpan,
        s_1: &'b pavex::request::RequestHead,
        next: fn(&'a pavex_tracing::RootSpan, &'b pavex::request::RequestHead) -> T,
    }
    impl<'a, 'b, T> std::future::IntoFuture for Next1<'a, 'b, T>
    where
        T: std::future::Future<Output = pavex::response::Response>,
    {
        type Output = pavex::response::Response;
        type IntoFuture = T;
        fn into_future(self) -> Self::IntoFuture {
            (self.next)(self.s_0, self.s_1)
        }
    }
}
pub mod route_2 {
    pub async fn entrypoint<'a, 'b, 'c>(
        s_0: pavex::request::path::RawPathParams<'a, 'b>,
        s_1: pavex::request::path::MatchedPathPattern,
        s_2: &'c pavex::request::RequestHead,
    ) -> pavex::response::Response {
        let response = wrapping_0(s_0, s_1, s_2).await;
        response
    }
    async fn stage_1<'a, 'b, 'c>(
        s_0: pavex::request::path::RawPathParams<'a, 'b>,
        s_1: pavex::request::path::MatchedPathPattern,
        s_2: &'c pavex::request::RequestHead,
    ) -> pavex::response::Response {
        let response = wrapping_1(s_0, s_1, s_2).await;
        response
    }
    async fn stage_2<'a, 'b, 'c>(
        s_0: pavex::request::path::RawPathParams<'a, 'b>,
        s_1: &'c pavex_tracing::RootSpan,
    ) -> pavex::response::Response {
        let response = handler(s_0, s_1).await;
        let response = post_processing_0(response, s_1).await;
        response
    }
    async fn wrapping_0(
        v0: pavex::request::path::RawPathParams<'_, '_>,
        v1: pavex::request::path::MatchedPathPattern,
        v2: &pavex::request::RequestHead,
    ) -> pavex::response::Response {
        let v3 = crate::route_2::Next0 {
            s_0: v0,
            s_1: v1,
            s_2: v2,
            next: stage_1,
        };
        let v4 = pavex::middleware::Next::new(v3);
        let v5 = pavex::middleware::wrap_noop(v4).await;
        <pavex::response::Response as pavex::response::IntoResponse>::into_response(v5)
    }
    async fn wrapping_1(
        v0: pavex::request::path::RawPathParams<'_, '_>,
        v1: pavex::request::path::MatchedPathPattern,
        v2: &pavex::request::RequestHead,
    ) -> pavex::response::Response {
        let v3 = pavex::telemetry::ServerRequestId::generate();
        let v4 = app::telemetry::root_span(v2, v1, v3);
        let v5 = crate::route_2::Next1 {
            s_0: v0,
            s_1: &v4,
            next: stage_2,
        };
        let v6 = pavex::middleware::Next::new(v5);
        let v7 = <pavex_tracing::RootSpan as core::clone::Clone>::clone(&v4);
        let v8 = pavex_tracing::logger(v7, v6).await;
        <pavex::response::Response as pavex::response::IntoResponse>::into_response(v8)
    }
    async fn handler(
        v0: pavex::request::path::RawPathParams<'_, '_>,
        v1: &pavex_tracing::RootSpan,
    ) -> pavex::response::Response {
        let v2 = pavex::request::path::PathParams::extract(v0);
        let v3 = match v2 {
            Ok(ok) => ok,
            Err(v3) => {
                return {
                    let v4 = pavex::request::path::errors::ExtractPathParamsError::into_response(
                        &v3,
                    );
                    let v5 = pavex::Error::new(v3);
                    app::telemetry::error_logger(&v5, v1).await;
                    <pavex::response::Response as pavex::response::IntoResponse>::into_response(
                        v4,
                    )
                };
            }
        };
        let mut v4 = app::routes::Digimon::new();
        let v5 = app::routes::Digimon::get(&mut v4, &v3);
        let v6 = match v5 {
            Ok(ok) => ok,
            Err(v6) => {
                return {
                    let v7 = app::routes::GetError::into_response(&v6);
                    let v8 = pavex::Error::new(v6);
                    app::telemetry::error_logger(&v8, v1).await;
                    <pavex::response::Response as pavex::response::IntoResponse>::into_response(
                        v7,
                    )
                };
            }
        };
        <pavex::response::Response as pavex::response::IntoResponse>::into_response(v6)
    }
    async fn post_processing_0(
        v0: pavex::response::Response,
        v1: &pavex_tracing::RootSpan,
    ) -> pavex::response::Response {
        let v2 = app::telemetry::response_logger(v0, v1).await;
        <pavex::response::Response as pavex::response::IntoResponse>::into_response(v2)
    }
    struct Next0<'a, 'b, 'c, T>
    where
        T: std::future::Future<Output = pavex::response::Response>,
    {
        s_0: pavex::request::path::RawPathParams<'a, 'b>,
        s_1: pavex::request::path::MatchedPathPattern,
        s_2: &'c pavex::request::RequestHead,
        next: fn(
            pavex::request::path::RawPathParams<'a, 'b>,
            pavex::request::path::MatchedPathPattern,
            &'c pavex::request::RequestHead,
        ) -> T,
    }
    impl<'a, 'b, 'c, T> std::future::IntoFuture for Next0<'a, 'b, 'c, T>
    where
        T: std::future::Future<Output = pavex::response::Response>,
    {
        type Output = pavex::response::Response;
        type IntoFuture = T;
        fn into_future(self) -> Self::IntoFuture {
            (self.next)(self.s_0, self.s_1, self.s_2)
        }
    }
    struct Next1<'a, 'b, 'c, T>
    where
        T: std::future::Future<Output = pavex::response::Response>,
    {
        s_0: pavex::request::path::RawPathParams<'a, 'b>,
        s_1: &'c pavex_tracing::RootSpan,
        next: fn(
            pavex::request::path::RawPathParams<'a, 'b>,
            &'c pavex_tracing::RootSpan,
        ) -> T,
    }
    impl<'a, 'b, 'c, T> std::future::IntoFuture for Next1<'a, 'b, 'c, T>
    where
        T: std::future::Future<Output = pavex::response::Response>,
    {
        type Output = pavex::response::Response;
        type IntoFuture = T;
        fn into_future(self) -> Self::IntoFuture {
            (self.next)(self.s_0, self.s_1)
        }
    }
}
