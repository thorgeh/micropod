use micropod::Command;

fn main() {
    let pod = micropod::Pod::new("/tmp/micropod")
        .run(&Command::new("lol"))
        .unwrap();
}
