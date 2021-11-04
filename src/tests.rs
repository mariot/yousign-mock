use rocket::local::blocking::Client;
use rocket::http::{Status, ContentType, Accept};
use rocket::serde::json::{Value, json};
use rocket::serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct File {
    id: Option<usize>,
    name: String,
    content: String
}

impl File {
    fn new(name: impl Into<String>, content: impl Into<String>) -> Self {
        File { name: name.into(), content: content.into(), id: None }
    }
}

#[test]
fn files_bad_get() {
    let client = Client::tracked(super::rocket()).unwrap();

    // Try to get a file with an ID that doesn't exist.
    let res = client.get("/files/99").header(ContentType::JSON).dispatch();
    assert_eq!(res.status(), Status::NotFound);

    let body = res.into_string().unwrap();
    assert!(body.contains("error"));
    assert!(body.contains("Resource was not found."));

    // Try to get a file with an invalid ID.
    let res = client.get("/files/hi").header(ContentType::JSON).dispatch();
    assert_eq!(res.status(), Status::NotFound);
    assert!(res.into_string().unwrap().contains("error"));
}

#[test]
fn files_bad_download() {
    let client = Client::tracked(super::rocket()).unwrap();

    // Try to download a file with an ID that doesn't exist.
    let res = client.get("/files/99/download").header(ContentType::JSON).dispatch();
    assert_eq!(res.status(), Status::NotFound);

    let body = res.into_string().unwrap();
    assert!(body.contains("error"));
    assert!(body.contains("Resource was not found."));

    // Try to get a file with an invalid ID.
    let res = client.get("/files/hi/download").header(ContentType::JSON).dispatch();
    assert_eq!(res.status(), Status::NotFound);
    assert!(res.into_string().unwrap().contains("error"));
}

#[test]
fn files_post_get_download() {
    let client = Client::tracked(super::rocket()).unwrap();

    // Create/read/download a few files.
    for id in 0..10 {
        let uri = format!("/files/{}", id);
        let download_uri = format!("/files/{}/download", id);
        let name= format!("Hello, JSON {}!", id);
        let content = "How are you?";
        let json_response = json!({
            "id": "/files/".to_owned() + &id.to_string(),
            "name": name.to_string(),
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
        });

        // Check that a file with current id doesn't exist.
        let res = client.get(&uri).header(ContentType::JSON).dispatch();
        assert_eq!(res.status(), Status::NotFound);

        // Add a new file. This should be ID 0.
        let file = File::new(name, content);
        let res = client.post("/files").json(&file).dispatch();
        assert_eq!(res.status(), Status::Ok);

        // Check that the file exists with the correct contents.
        let res = client.get(&uri).header(Accept::JSON).dispatch();
        assert_eq!(res.status(), Status::Ok);
        assert_eq!(res.into_json::<Value>().unwrap(), json_response);

        // Check that the downloaded file contents are correct.
        let res = client.get(&download_uri).header(Accept::JSON).dispatch();
        assert_eq!(res.status(), Status::Ok);
        assert_eq!(res.into_json::<String>().unwrap(), content);
    }
}
