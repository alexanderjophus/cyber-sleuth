use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use pavex::blueprint::{router::GET, Blueprint};
use pavex::f;
use pavex::http::StatusCode;
use pavex::request::{path::PathParams, query::QueryParams};
use pavex::response::body::Json;
use pavex::response::Response;
use std::env;

pub fn register(bp: &mut Blueprint) {
    bp.nest_at("/api", blueprint());
}

pub fn blueprint() -> Blueprint {
    let mut bp = Blueprint::new();
    PathParams::register(&mut bp);
    QueryParams::register(&mut bp);
    bp.request_scoped(f!(self::Digimon::new));
    bp.route(GET, "/digimon", f!(self::Digimon::list))
        .error_handler(f!(self::ListError::into_response));
    bp.route(GET, "/digimon/:name", f!(self::Digimon::get))
        .error_handler(f!(self::GetError::into_response));
    bp.route(GET, "/digimon_search", f!(self::Digimon::search))
        .error_handler(f!(self::SearchError::into_response));
    bp
}

#[PathParams]
pub struct GetDigimonParams {
    pub name: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct SearchDigimonParams {
    pub stage: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub attribute: Option<String>,
    pub memory: Option<i64>,
    pub equip_slots: Option<i64>,
}

pub struct Digimon {
    // database connection
    conn: MysqlConnection,
}

impl Default for Digimon {
    fn default() -> Self {
        Self::new()
    }
}

impl Digimon {
    pub fn new() -> Self {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        Self {
            conn: MysqlConnection::establish(&database_url)
                .unwrap_or_else(|_| panic!("Error connecting to {}", database_url)),
        }
    }

    pub fn list(&mut self) -> Result<Response, ListError> {
        use super::schema::digimon::dsl::*;

        let res = digimon
            .load::<super::models::Digimon>(&mut self.conn)
            .unwrap();

        let res = Json::new(res)
            .map_err(Into::into)
            .map_err(ListError::UnexpectedError)?;

        Ok(Response::new(StatusCode::OK).set_typed_body(res))
    }

    pub fn get(&mut self, params: &PathParams<GetDigimonParams>) -> Result<Response, GetError> {
        use super::schema::digimon::dsl::*;

        let res = digimon
            .filter(Name.eq(params.0.name.as_str()))
            .first::<super::models::Digimon>(&mut self.conn)
            .optional()
            .unwrap();

        let res = match res {
            Some(res) => Json::new(res)
                .map_err(Into::into)
                .map_err(GetError::UnexpectedError)?,
            None => return Err(GetError::NotFound),
        };

        Ok(Response::new(StatusCode::OK).set_typed_body(res))
    }

    pub fn search(
        &mut self,
        params: &QueryParams<SearchDigimonParams>,
    ) -> Result<Response, SearchError> {
        use super::schema::digimon::dsl::*;

        let mut query = digimon.into_boxed();
        if let Some(stage) = &params.0.stage {
            query = query.filter(Stage.eq(stage));
        }
        if let Some(type_) = &params.0.type_ {
            query = query.filter(Type.eq(type_));
        }
        if let Some(attribute) = &params.0.attribute {
            query = query.filter(Attribute.eq(attribute));
        }
        if let Some(memory) = &params.0.memory {
            query = query.filter(Memory.eq(memory));
        }
        if let Some(equip_slots) = &params.0.equip_slots {
            query = query.filter(Equip_Slots.eq(equip_slots));
        }

        let res = query
            .load::<super::models::Digimon>(&mut self.conn)
            .unwrap();

        let res = Json::new(res)
            .map_err(Into::into)
            .map_err(SearchError::UnexpectedError)?;

        Ok(Response::new(StatusCode::OK).set_typed_body(res))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ListError {
    #[error("Something went wrong. Please retry later.")]
    UnexpectedError(#[source] anyhow::Error),
}

impl ListError {
    pub fn into_response(&self) -> Response {
        match self {
            ListError::UnexpectedError(_) => Response::internal_server_error(),
        }
        .set_typed_body(format!("{self}"))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetError {
    #[error("Digimon not found.")]
    NotFound,
    #[error("Something went wrong. Please retry later.")]
    UnexpectedError(#[source] anyhow::Error),
}

impl GetError {
    pub fn into_response(&self) -> Response {
        match self {
            GetError::NotFound => Response::not_found(),
            GetError::UnexpectedError(_) => Response::internal_server_error(),
        }
        .set_typed_body(format!("{self}"))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum SearchError {
    #[error("Something went wrong. Please retry later.")]
    UnexpectedError(#[source] anyhow::Error),
}

impl SearchError {
    pub fn into_response(&self) -> Response {
        match self {
            SearchError::UnexpectedError(_) => Response::internal_server_error(),
        }
        .set_typed_body(format!("{self}"))
    }
}
