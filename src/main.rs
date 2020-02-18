fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1).collect::<Vec<_>>();
    let bin = args.remove(0);
    std::process::Command::new(bin)
        .args(args)
        .spawn()?
    ;
    Ok(())
}
