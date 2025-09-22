use micropod::{Command, Pod};

fn main() -> miette::Result<()> {
    let pod = Pod::new("/tmp/micropod").layer("busybox", "busybox");

    let cmd = Command::new("init")
        .env("HOME", "/root")
        .env("USER", "root")
        .env("PATH", "/bin:/sbin:/usr/bin:/usr/sbin");

    pod.run(&cmd)?;

    Ok(())
}
