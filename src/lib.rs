use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Bar {
    pub baz: i32,
}

#[derive(Debug, Deserialize)]
pub struct Foo {
    #[serde(flatten)]
    pub bar: Bar,
}

#[test]
fn deserialize() {
    let result: Foo = serde_json::from_str(
        r#"{
            "baz": 1
        }"#,
    )
    .expect("deserialize failed");

    dbg!(result);
}
