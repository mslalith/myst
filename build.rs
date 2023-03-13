use std::{path::Path, process::Command, time::Instant};

fn main() -> std::io::Result<()> {
    println!("Regenerating Tailwind CSS file");

    let input = Path::new("./src/input.css");
    let output = Path::new("./dist/output.css");
    let start = Instant::now();

    let tailwind = Command::new("/Users/mslalith/Keep/tailwindcss")
        .arg("-i")
        .arg(input)
        .arg("-o")
        .arg(output)
        .arg("--minify")
        .output()
        .unwrap();

    if tailwind.status.success() {
        println!(
            "Compiled Tailwind CSS styles in {:.2}s",
            Instant::now().duration_since(start).as_secs_f64()
        );
    } else {
        println!("Failed compiling Tailwind CSS");
    }

    Ok(())
}
