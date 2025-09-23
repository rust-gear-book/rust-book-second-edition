#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use crate::models::_entities::todos;
use crate::models::_entities::todos::Model as TodoModel;
use crate::models::todos::Todos;
use axum::debug_handler;
use axum::extract::Form;
use axum::response::Redirect;
use loco_rs::prelude::*;
use sea_orm::Set;
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;

#[derive(Serialize)]
struct TodoEntry {
    id: u32,
    text: String,
}

impl From<TodoModel> for TodoEntry {
    fn from(todo: TodoModel) -> Self {
        TodoEntry {
            id: todo.id as u32,
            text: todo.text.unwrap_or_default(),
        }
    }
}

#[derive(Deserialize)]
pub struct AddParams {
    pub text: String,
}

#[derive(Deserialize)]
pub struct DeleteParams {
    pub id: u32,
}

#[debug_handler]
pub async fn index(
    ViewEngine(v): ViewEngine<TeraView>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let db = &ctx.db;
    let todos: Vec<TodoModel> = Todos::find().all(db).await?;
    let entries: Vec<TodoEntry> = todos.into_iter().map(TodoEntry::from).collect();
    let data = json!({
        "entries": entries,
    });
    format::render().view(&v, "todo/index.html", data)
}

#[debug_handler]
pub async fn add_todo(
    State(ctx): State<AppContext>,
    Form(params): Form<AddParams>,
) -> Result<Redirect> {
    let db = &ctx.db;
    let new_todo = todos::ActiveModel {
        text: Set(Some(params.text)),
        ..Default::default()
    };
    new_todo.insert(db).await?;
    Ok(Redirect::to("/"))
}

#[debug_handler]
pub async fn delete_todo(
    State(ctx): State<AppContext>,
    Form(params): Form<DeleteParams>,
) -> Result<Redirect> {
    let db = &ctx.db;
    // idで該当レコードを削除
    todos::Entity::delete_by_id(params.id as i32)
        .exec(db)
        .await?;
    Ok(Redirect::to("/"))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("todos/")
        .add("index", get(index))
        .add("add", post(add_todo))
        .add("delete", post(delete_todo))
}
