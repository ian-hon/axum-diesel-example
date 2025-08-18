#import "@preview/catppuccin:1.0.0": catppuccin, flavors
#import "@preview/cetz:0.4.1"
#import "@preview/metropolis-polylux:0.1.0" as metropolis
#import "@preview/polylux:0.4.0": *
#import metropolis: new-section, focus

#show: metropolis.setup

#let flavor = flavors.mocha
#show: catppuccin.with(flavor)
#let palette = flavor.colors

#set page(footer: none)

#show heading.where(level: 2): set align(top)

#slide[
  #set page(header: none, footer: none, margin: 3em)

  Rust as a High-level Programming Language:

  #text(size: 1.3em)[
    *Backend Webdev with axum and Diesel*
  ]

  #metropolis.divider

  #set text(size: .8em, weight: "light")

  Ian & Han

  Aug 21, 2025

  Rust Malaysia \@ Shortcut Asia
]

#slide[
  = Agenda

  #metropolis.outline
]

#new-section[axum web framework]

#slide[
  = Hello, World!

  ```rust
  use axum::Router;
  use axum::routing::get;
  use tokio::net::TcpListener;

  #[tokio::main]
  async fn main() {
      // build our application with a single route
      let app = Router::new()
          .route("/", get(async || "Hello, World!"));

      // run our app with hyper, listening globally on port 3000
      let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
      axum::serve(listener, app).await.unwrap();
  }
  ```
]

#slide[
  = Hello, World!

  ```http
  GET / HTTP/1.1
  Host: localhost:3000
  ```

  #box(inset: (top: 1em), stroke: (top: palette.overlay0.rgb))[
    ```http
    HTTP/1.1 200 OK
    Content-Type: text/plain; charset=utf-8

    Hello, World!
    ```
  ]
]

#slide[
  #set align(center)

  #stack(
    spacing: 0.5em,
    block(
      width: 10em,
      fill: palette.sky.rgb,
      inset: 1em,
      text(palette.crust.rgb)[axum]
    ),
    block(
      width: 10em,
      fill: palette.green.rgb,
      inset: 1em,
      text(palette.crust.rgb)[
        tower

        #text(palette.surface1.rgb, style: "italic")[
          (middleware)
        ]
      ]
    ),
    block(
      width: 10em,
      fill: palette.crust.rgb,
      inset: 1em,
      text(palette.text.rgb)[
        hyper

        #text(palette.subtext0.rgb, style: "italic")[
          (HTTP)
        ]
      ]
    ),
  )
]

#slide[
  = Routing

  ```rust
  use axum::Router;
  use axum::routing::{get, post};

  let app = Router::new()
      .route("/", get(get_root))
      .route("/cats", get(get_cats))
      .route("/cappuccino", post(post_cappuccino));

  async fn get_root() {}
  async fn get_cats() {}
  async fn post_cappuccino() {}
  ```
]

#slide[
  = Routing

  #grid(columns: (auto, 1fr), align: (left, center), [
    ```rust
    use axum::Router;
    use axum::routing::{get, post};

    let cat_routes = Router::new()
        .route("/{cat_id}", get(get_cat));

    let app = Router::new()
        .route("/", get(get_root))
        .nest("/cats", cat_routes)
        .route("/cappuccino", post(post_cappuccino));

    async fn get_root() {}
    async fn get_cat() {}
    async fn post_cappuccino() {}
    ```
  ], block[
    #set align(left)

    Nesting routes
  ])
]

#slide[
  = Handlers

  == Matching path parameters

  ```http
  GET /cats/5e HTTP/1.1
  Host: localhost:3000
  ```

  #box(inset: (top: 1em), stroke: (top: palette.overlay0.rgb))[
    ```http
    HTTP/1.1 200 OK
    Content-Type: text/plain; charset=utf-8

    Cat 5e
    ```
  ]
]

#slide[
  = Handlers

  #grid(columns: (auto, 1fr), align: (left, center), [
    ```rust
    use axum::Router;
    use axum::extract::Path;
    use axum::routing::get;

    let app = Router::new()
        .route("/cats/{cat_id}", get(get_cat));

    async fn get_cat(
        Path(cat_id): Path<String>,
    ) -> String {
        format!("Cat {cat_id}")
    }
    ```
  ], block[
    #set align(left)

    Matching path parameters
  ])
]

#slide[
  = Handlers

  == Extracting query parameters

  ```http
  GET /greeting?name=Frieren HTTP/1.1
  Host: localhost:3000
  ```

  #box(inset: (top: 1em), stroke: (top: palette.overlay0.rgb))[
    ```http
    HTTP/1.1 200 OK
    Content-Type: text/plain; charset=utf-8

    Hello, Frieren!
    ```
  ]
]

#slide[
  = Handlers

  #grid(columns: (auto, 1fr), align: (left, center), [
    ```rust
    use axum::extract::Query;
    use serde::Deserialize;

    #[derive(Deserialize)]
    struct GetGreetingParams {
        name: String,
    }

    async fn get_greeting(
        Query(GetGreetingParams {
            name,
        }): Query<GetGreetingParams>,
    ) -> String {
        format!("Hello, {name}!")
    }
    ```
  ], block[
    #set align(left)

    Extracting query parameters
  ])
]

#slide[
  = Handlers

  == JSON request

  ```http
  POST /greeting HTTP/1.1
  Host: localhost:3000
  Content-Type: application/json

  {
    "name": "Frieren"
  }
  ```

  #box(inset: (top: 1em), stroke: (top: palette.overlay0.rgb))[
    ```http
    HTTP/1.1 200 OK
    Content-Type: text/plain; charset=utf-8

    Hello, Frieren!
    ```
  ]
]

#slide[
  = Handlers

  #grid(columns: (auto, 1fr), align: (left, center), [
    ```rust
    use axum::extract::Json;
    use serde::Deserialize;

    #[derive(Deserialize)]
    struct PostGreetingPayload {
        name: String,
    }

    async fn post_greeting(
        Json(PostGreetingPayload {
            name,
        }): Json<PostGreetingPayload>,
    ) -> String {
        format!("Hello, {name}!")
    }
    ```
  ], block[
    #set align(left)

    JSON request
  ])
]

#slide[
  = Handlers

  == JSON response

  ```http
  POST /greeting HTTP/1.1
  Host: localhost:3000
  Content-Type: application/json

  { "name": "Frieren" }
  ```

  #box(inset: (top: 1em), stroke: (top: palette.overlay0.rgb))[
    ```http
    HTTP/1.1 200 OK
    Content-Type: application/json

    { "message": "Hello, Frieren!" }
    ```
  ]
]

#slide[
  = Handlers

  #grid(columns: (auto, 1fr), align: (left, center), [
    #set text(size: .9em)

    ```rust
    use axum::extract::Json;
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize)]
    struct PostGreetingPayload { name: String }

    #[derive(Serialize)]
    struct PostGreetingResponse { message: String }

    async fn post_greeting(
        Json(PostGreetingPayload { name }): Json<PostGreetingPayload>,
    ) -> Json<PostGreetingResponse> {
        Json(PostGreetingResponse {
            message: format!("Hello, {name}!"),
        })
    }
    ```
  ], block[
    #set align(left)

    JSON response
  ])
]

#slide[
  = Handlers

  == Optional query parameter with a fallback value

  #grid(columns: (auto, 1fr), gutter: 1em, [
    #box(inset: (bottom: 1em), stroke: (bottom: palette.overlay0.rgb))[
      ```http
      GET /greeting?name=Frieren HTTP/1.1
      Host: localhost:3000
      ```
    ]

    ```http
    HTTP/1.1 200 OK
    Content-Type: text/plain

    Hello, Frieren!
    ```
  ], [
    ```http
    GET /greeting HTTP/1.1
    Host: localhost:3000
    ```

    #box(inset: (top: 1em), stroke: (top: palette.overlay0.rgb))[
      ```http
      HTTP/1.1 200 OK
      Content-Type: text/plain

      Hello, stranger!
      ```
    ]
  ])
]

#slide[
  = Handlers

  #grid(columns: (auto, 1fr), align: (left, center), [
    #set text(size: .9em)

    ```rust
    use axum::extract::Query;
    use serde::Deserialize;

    #[derive(Deserialize)]
    struct GetGreetingParams {
      name: Option<String>,
    }

    async fn get_greeting(
        Query(GetGreetingParams { name }): Query<GetGreetingParams>,
    ) -> String {
        let name = name.unwrap_or_else(|| "stranger".to_owned());

        format!("Hello, {name}!")
    }
    ```
  ], block[
    #set align(left)

    Optional query parameter with a fallback value
  ])
]

#slide[
  = Handlers

  == Error handling

  #grid(columns: (1fr, auto), gutter: 1em, [
    #set text(size: .9em)

    #box(inset: (bottom: 1em), stroke: (bottom: palette.overlay0.rgb))[
      ```http
      GET /greeting?name=Frieren HTTP/1.1
      Host: localhost:3000
      ```
    ]

    ```http
    HTTP/1.1 400 Bad Request
    Content-Type: text/plain

    nyan please
    ```
  ], [
    #box(inset: (bottom: 1em), stroke: (bottom: palette.overlay0.rgb))[
      ```http
      GET /greeting?name=Frienyan HTTP/1.1
      Host: localhost:3000
      ```
    ]

    ```http
    HTTP/1.1 200 OK
    Content-Type: text/plain

    Hello, Frienyan!
    ```
  ])
]

#slide[
  = Handlers

  #grid(columns: (auto, 1fr), align: (left, center), [
    #set text(size: .9em)

    ```rust
    use axum::extract::Query;
    use axum::http::StatusCode;
    use axum::response::Result;
    use serde::Deserialize;

    #[derive(Deserialize)]
    struct GetGreetingParams { name: String }

    async fn get_greeting(
        Query(GetGreetingParams { name }): Query<GetGreetingParams>,
    ) -> Result<String> {
        if !name.ends_with("nyan") {
          return Err((StatusCode::BAD_REQUEST, "nyan please"))?;
        }

        Ok(format!("Hello, {name}!"))
    }
    ```
  ], block[
    #set align(left)

    Error handling
  ])
]

#slide[
  = Middleware

  #grid(columns: (auto, 1fr), align: (left, center), [
    ```rust
    use axum::Router;
    use axum::routing::get;

    async fn handler() {}

    let app = Router::new()
        .route("/", get(handler))
        .layer(layer_one)
        .layer(layer_two)
        .layer(layer_three);
    ```
  ], [
    #set text(size: .9em)

    #cetz.canvas({
      import cetz.draw: *

      content((0, 0), [handler], name: "handler")
      group(name: "layer_one", {
        rect-around("handler", padding: 1em, stroke: (paint: palette.red.rgb, dash: "dashed"), name: "layer_one rect")
        content((name: "layer_one rect", anchor: 90deg), box(fill: palette.base.rgb, text(palette.red.rgb)[layer_one]))
        content((name: "layer_one rect", anchor: -90deg), box(fill: palette.base.rgb, text(palette.red.rgb)[layer_one]))
      })
      group(name: "layer_two", {
        rect-around("layer_one", padding: 1em, stroke: (paint: palette.green.rgb, dash: "dashed"), name: "layer_two rect")
        content((name: "layer_two rect", anchor: 90deg), box(fill: palette.base.rgb, text(palette.green.rgb)[layer_two]))
        content((name: "layer_two rect", anchor: -90deg), box(fill: palette.base.rgb, text(palette.green.rgb)[layer_two]))
      })
      group(name: "layer_three", {
        rect-around("layer_two", padding: 1em, stroke: (paint: palette.blue.rgb, dash: "dashed"), name: "layer_three rect")
        content((name: "layer_three rect", anchor: 90deg), box(fill: palette.base.rgb, text(palette.blue.rgb)[layer_three]))
        content((name: "layer_three rect", anchor: -90deg), box(fill: palette.base.rgb, text(palette.blue.rgb)[layer_three]))
      })
      line(
        (rel: (0, 2em), to: (name: "layer_three", anchor: 90deg)),
        (rel: (0, 0.5em), to: (name: "layer_three", anchor: 90deg)),
        mark: (end: ">"),
        stroke: palette.overlay2.rgb,
        name: "requests line"
      )
      content(
        (rel: (0, 2.5em), to: (name: "layer_three", anchor: 90deg)),
        box(fill: palette.base.rgb, [requests])
      )
      line(
        (rel: (0, -0.5em), to: (name: "layer_three", anchor: -90deg)),
        (rel: (0, -2em), to: (name: "layer_three", anchor: -90deg)),
        mark: (end: ">"),
        stroke: palette.overlay2.rgb,
        name: "responses line"
      )
      content(
        (rel: (0, -2.5em), to: (name: "layer_three", anchor: -90deg)),
        box(fill: palette.base.rgb, [responses])
      )
    })
  ])
]

#slide[
  = Middleware

  #grid(columns: (auto, 1fr), align: (left, center), [
    ```rust
    use axum::Router;
    use axum::routing::get;
    use tower::ServiceBuilder;

    async fn handler() {}

    let app = Router::new()
        .route("/", get(handler))
        .layer(
            ServiceBuilder::new()
                .layer(layer_one)
                .layer(layer_two)
                .layer(layer_three),
        );
    ```
  ], [
    #set text(size: .9em)

    #cetz.canvas({
      import cetz.draw: *

      content((0, 0), [handler], name: "handler")
      group(name: "layer_three", {
        rect-around("handler", padding: 1em, stroke: (paint: palette.red.rgb, dash: "dashed"), name: "layer_three rect")
        content((name: "layer_three rect", anchor: 90deg), box(fill: palette.base.rgb, text(palette.red.rgb)[layer_three]))
        content((name: "layer_three rect", anchor: -90deg), box(fill: palette.base.rgb, text(palette.red.rgb)[layer_three]))
      })
      group(name: "layer_two", {
        rect-around("layer_three", padding: 1em, stroke: (paint: palette.green.rgb, dash: "dashed"), name: "layer_two rect")
        content((name: "layer_two rect", anchor: 90deg), box(fill: palette.base.rgb, text(palette.green.rgb)[layer_two]))
        content((name: "layer_two rect", anchor: -90deg), box(fill: palette.base.rgb, text(palette.green.rgb)[layer_two]))
      })
      group(name: "layer_one", {
        rect-around("layer_two", padding: 1em, stroke: (paint: palette.blue.rgb, dash: "dashed"), name: "layer_one rect")
        content((name: "layer_one rect", anchor: 90deg), box(fill: palette.base.rgb, text(palette.blue.rgb)[layer_one]))
        content((name: "layer_one rect", anchor: -90deg), box(fill: palette.base.rgb, text(palette.blue.rgb)[layer_one]))
      })
      line(
        (rel: (0, 2em), to: (name: "layer_one", anchor: 90deg)),
        (rel: (0, 0.5em), to: (name: "layer_one", anchor: 90deg)),
        mark: (end: ">"),
        stroke: palette.overlay2.rgb,
        name: "requests line"
      )
      content(
        (rel: (0, 2.5em), to: (name: "layer_one", anchor: 90deg)),
        box(fill: palette.base.rgb, [requests])
      )
      line(
        (rel: (0, -0.5em), to: (name: "layer_one", anchor: -90deg)),
        (rel: (0, -2em), to: (name: "layer_one", anchor: -90deg)),
        mark: (end: ">"),
        stroke: palette.overlay2.rgb,
        name: "responses line"
      )
      content(
        (rel: (0, -2.5em), to: (name: "layer_one", anchor: -90deg)),
        box(fill: palette.base.rgb, [responses])
      )
    })
  ])
]

#slide[
  = State

  #set text(size: .9em)

  ```rust
  use axum::Router;
  use axum::extract::State;
  use axum::routing::get;

  #[derive(Clone)]
  struct AppState { cat: String }

  let state = AppState { flavor: "Mocha".to_owned() };

  let app = Router::new()
      .route("/cat", get(get_cat))
      .with_state(state);

  async fn get_cat(State(flavor): State<String>) -> String {
      format!("Catppuccin {flavor}")
  }
  ```
]

#slide[
  = State

  ```http
  GET /cat HTTP/1.1
  Host: localhost:3000
  ```

  #box(inset: (top: 1em), stroke: (top: palette.overlay0.rgb))[
    ```http
    HTTP/1.1 200 OK
    Content-Type: text/plain; charset=utf-8

    Catppuccin Mocha
    ```
  ]
]

#new-section[Diesel ORM]

#slide[
  = Schema

  ```env
  DATABASE_URL=postgres://username:password@pg.example.com:5432/db_name
  ```

  #v(1em)

  ```shell
  diesel print-schema > src/schema.rs
  ```
]

#slide[
  = Schema

  #grid(columns: (auto, 1fr), align: (left, center), [
    ```rust
    diesel::table! {
        cats (id) {
            id -> Uuid,
            name -> Nullable<Varchar>,
            legs -> Int4,
            purrs -> Bool,
            human_id -> Nullable<Uuid>,
        }
    }

    diesel::table! {
        humans (id) {
            id -> Uuid,
        }
    }
    ```
  ], block[
    #set align(left)

    Auto-generated from database schema
  ])
]

#slide[
  = Schema

  #grid(columns: (auto, 1fr), align: (left, center), [
    ```rust
    diesel::joinable!(cats -> humans (human_id));

    diesel::allow_tables_to_appear_in_same_query!(
        cats,
        humans,
    );
    ```
  ], block[
    #set align(left)

    Auto-detected relations from foreign key constraints
  ])
]

#slide[
  = Models

  ```rust
  use diesel::prelude::*;
  use uuid::Uuid;

  use crate::schema::cats;

  #[derive(Debug, Queryable, Selectable)]
  #[diesel(table_name = cats)]
  #[diesel(check_for_backend(diesel::pg::Pg))]
  pub struct Cat {
      pub id: Uuid,
      pub name: Option<String>,
      pub legs: i32,
      pub purrs: bool,
      pub human_id: Option<Uuid>,
  }
  ```
]

#slide[
  = Models

  ```rust
  use diesel::prelude::*;
  use uuid::Uuid;

  use crate::schema::humans;

  #[derive(Debug, Queryable, Selectable)]
  #[diesel(table_name = humans)]
  #[diesel(check_for_backend(diesel::pg::Pg))]
  pub struct Human {
      pub id: Uuid,
  }
  ```
]

#slide[
  = Connection

  ```rust
  use std::env;

  use diesel_async::pooled_connection::deadpool::Pool;
  use diesel_async::pooled_connection::AsyncDieselConnectionManager;
  use diesel_async::AsyncPgConnection;

  dotenvy::dotenv()?;
  let db_url = env::var("DATABASE_URL")?;

  let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(
      db_url.as_str(),
  );
  let pool = Pool::builder(manager).build()?;
  let mut conn = pool.get().await?;
  ```
]

#slide[
  = CRUD

  #set text(size: .9em)

  ```rust
  use diesel::prelude::*;
  use diesel_async::{AsyncPgConnection, RunQueryDsl};

  use crate::models::Cat;

  pub async fn get_cats(
      conn: &mut AsyncPgConnection,
  ) -> QueryResult<Vec<Cat>> {
      use crate::schema::cats::dsl::*;

      cats
          .filter(purrs.eq(true))
          .limit(5)
          .select(Cat::as_select())
          .load(conn)
          .await
  }
  ```
]

#slide[
  = Models

  ```rust
  use diesel::prelude::*;
  use uuid::Uuid;

  use crate::schema::cats;

  #[derive(Debug, Default, Insertable)]
  #[diesel(table_name = cats)]
  pub struct NewCat {
      pub name: Option<String>,
      pub legs: i32,
      pub purrs: bool,
      pub human_id: Option<Uuid>,
  }
  ```
]

#slide[
  = CRUD

  #set text(size: .9em)

  ```rust
  use diesel::prelude::*;
  use diesel_async::{AsyncPgConnection, RunQueryDsl};

  use crate::models::{Cat, NewCat};

  pub async fn create_cat(conn: &mut AsyncPgConnection) -> QueryResult<Cat> {
      use crate::schema::cats;

      let new_cat = NewCat { legs: 3, ..Default::default() };

      diesel::insert_into(cats::table)
          .values(&new_cat)
          .returning(Cat::as_returning())
          .get_result(conn)
          .await
  }
  ```
]

#slide[
  = Models

  ```rust
  use diesel::prelude::*;
  use uuid::Uuid;

  use crate::schema::cats;

  #[derive(Debug, AsChangeset, Identifiable, Queryable, Selectable)]
  #[diesel(table_name = cats)]
  #[diesel(check_for_backend(diesel::pg::Pg))]
  pub struct Cat {
      pub id: Uuid,
      pub name: Option<String>,
      pub legs: i32,
      pub purrs: bool,
      pub human_id: Option<Uuid>,
  }
  ```
]

#slide[
  = CRUD

  #set text(size: .9em)

  ```rust
  use diesel::prelude::*;
  use diesel_async::{AsyncPgConnection, RunQueryDsl};

  use crate::models::Cat;

  pub async fn update_cat(
      conn: &mut AsyncPgConnection,
      cat: &Cat,
  ) -> QueryResult<Cat> {
      use crate::schema::cats::dsl::*;

      diesel::update(cat)
          .set(purrs.eq(true))
          .returning(Cat::as_returning())
          .get_result(conn)
          .await
  }
  ```
]

#slide[
  = CRUD

  #set text(size: .9em)

  ```rust
  use diesel::prelude::*;
  use diesel_async::{AsyncPgConnection, RunQueryDsl};

  use crate::models::Cat;

  pub async fn get_cat(
      conn: &mut AsyncPgConnection,
      cat_id: uuid::Uuid,
  ) -> QueryResult<Option<Cat>> {
      use crate::schema::cats::dsl::*;
      cats
          .find(cat_id)
          .select(Cat::as_select())
          .first(conn)
          .await
          .optional()
  }
  ```
]

#slide[
  = CRUD

  #set text(size: .9em)

  ```rust
  use diesel::prelude::*;
  use diesel_async::{AsyncPgConnection, RunQueryDsl};

  pub async fn delete_impostor_cats(
      conn: &mut AsyncPgConnection,
  ) -> QueryResult<usize> {
      use crate::schema::cats::dsl::*;

      diesel::delete(
          cats
              .filter(name.eq("Flerken").and(legs.gt(6))),
      )
          .execute(conn)
          .await
  }
  ```
]

#new-section[Example & demo - e-wallet app]

#slide[
  #set align(center)

  https://github.com/ian-hon/axum-diesel-example
]

#slide[
  #show: focus

  Demo time!
]

#new-section[Q&A]

#slide[
  #show: focus

  Questions?
]

#slide[
  #show: focus
  #set page(fill: palette.base.rgb)
  #set text(palette.text.rgb)

  Thank You!

  🙇🙇

  🙏🙏
]
