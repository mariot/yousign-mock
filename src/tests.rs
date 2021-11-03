use rocket::local::blocking::Client;
use rocket::http::{Status, ContentType, Accept};
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

    fn with_id(mut self, id: usize) -> Self {
        self.id = Some(id);
        self
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
fn files_post_get() {
    let client = Client::tracked(super::rocket()).unwrap();

    // Create/read a few files.
    for id in 0..10 {
        let uri = format!("/files/{}", id);

        // Check that a file with current id doesn't exist.
        let res = client.get(&uri).header(ContentType::JSON).dispatch();
        assert_eq!(res.status(), Status::NotFound);

        // Add a new file. This should be ID 0.
        let file = File::new(format!("Hello, JSON {}!", id), "How are you?");
        let res = client.post("/files").json(&file).dispatch();
        assert_eq!(res.status(), Status::Ok);

        // Check that the file exists with the correct contents.
        let res = client.get(&uri).header(Accept::JSON).dispatch();
        assert_eq!(res.status(), Status::Ok);
        assert_eq!(res.into_json::<File>().unwrap(), file.with_id(id));
    }
}
