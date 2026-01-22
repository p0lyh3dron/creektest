fn main() {
    let mut script = creek::api::Scripts::new("lua");
    let _ = script.exec("require('main'); init();");

    std::thread::spawn(move || {
        loop {
            _ = script.exec("update();");
        }
    });

    let mut io = creek::api::IO::new("vulkan");
    io.init();
}