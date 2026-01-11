fn main() {
    let mut script = creek::api::Scripts::new("lua");
    let _ = script.exec("print('Hello, Creek!');");

    let mut graphics = creek::api::Graphics::new("dummy");
}