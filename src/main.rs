fn main() {
    let mut script = creek::api::Scripts::new("lua");
    let _ = script.exec("print('Hello, Creek!');");
}