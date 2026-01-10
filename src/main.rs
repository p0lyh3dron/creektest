fn main() {
    let mut script = creek::api::Scripts::new("dummy");
    let _ = script.exec("print('Hello, Creek!');");
}