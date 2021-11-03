use std::borrow::Cow;

use rocket::State;
use rocket::tokio::sync::Mutex;
use rocket::serde::json::{Json, Value, json};
use rocket::serde::{Serialize, Deserialize};

// The type to represent the ID of a message.
type Id = usize;

// We're going to store all of the messages here. No need for a DB.
type FileList = Mutex<Vec<(String, String)>>;
type Files<'r> = &'r State<FileList>;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct File<'r> {
    id: Option<Id>,
    name: Cow<'r, str>,
    content: Cow<'r, str>,
}

#[post("/", format = "json", data = "<file>")]
async fn new(file: Json<File<'_>>, list: Files<'_>) -> Value {
    let mut list = list.lock().await;
    let id = list.len();
    let new_file = (file.name.to_string(), file.content.to_string());
    list.push(new_file);
    json!({
        "id": "/files/".to_owned() + &id.to_string(),
        "name": file.name.to_string(),
        "type": "signable",
        "contentType": "application/pdf",
        "description": null,
        "createdAt": "2018-12-01T11:36:20+01:00",
        "updatedAt": "2018-12-01T11:36:20+01:00",
        "sha256": "bb57ae2b2ca6ad0133a699350d1a6f6c8cdfde3cf872cf526585d306e4675cc2",
        "metadata": [],
        "workspace": "/workspaces/XXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX",
        "creator": null,
        "protected": false,
        "position": 0,
        "parent": null
    })
}

#[get("/<id>", format = "json")]
async fn get(id: Id, list: Files<'_>) -> Option<Json<File<'_>>> {
    let list = list.lock().await;
    let (name, content) = list.get(id)?;

    Some(Json(File {
        id: Some(id),
        name: name.to_string().into(),
        content: content.to_string().into(),
    }))
}

#[catch(404)]
fn not_found() -> Value {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("JSON", |rocket| async {
        rocket.mount("/files", routes![new, get])
            .register("/files", catchers![not_found])
            .manage(FileList::new(vec![]))
    })
}
