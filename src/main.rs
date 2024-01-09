use proc_macro_test::{donothing, Template};

// TODO: "Hide" this from the main program
pub trait TemplateFile {
    const PATH: &'static str;
    const CONTENT: &'static str;
}

#[derive(Template, Debug)]
#[template(path = "index.html")]
struct MyData {
    name: &'static str,
}

#[donothing()]
fn main() {
    let data = MyData { name: "hello" };
    println!("Data: {:?}", data);
    println!("File Path: {}", <MyData as TemplateFile>::PATH);
    println!("File Content: \n{}", <MyData as TemplateFile>::CONTENT);
}
