use proc_macro_test::{donothing, template};

// TODO: "Hide" this from the main program
pub trait TemplateFile {
    const PATH: &'static str;
    const CONTENT: &'static str;
}

#[template(path = "index.html")]
#[derive(Debug)]
struct MyData {
    name: &'static str,
}

#[donothing()]
fn main() {
    let data = MyData { name: "hello" };
    println!("Data: {:?}", data);
    println!("File Path: {}", <MyData as TemplateFile>::PATH);
    println!("File Content: {}", <MyData as TemplateFile>::CONTENT);
}
