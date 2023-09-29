use std::{
    fs::File,
    io::{self, Write},
    time::Instant,
};

fn main() {
    print!("enter a number: ");
    io::stdout().flush().unwrap();
    let mut string = String::new();
    io::stdin().read_line(&mut string).unwrap();
    let number = string.trim().parse().unwrap();

    let now = Instant::now();
    let mut file = File::create("elifulator.py").unwrap();
    let setup = format!("num1 = int(input(\"Enter first number: \"))\nnum2 = int(input(\"Enter second number: \"))\n\nif (num1 == 1 and num2 == 1):\n    print(\"2\")\n");
    file.write(setup.as_bytes()).unwrap();

    for i in 1..=number {
        for n in 2..=number {
            let math = format!(
                "elif (num1 == {i} and num2 == {n}):\n    print(\"{}\")\n",
                i + n
            );
            file.write(math.as_bytes()).unwrap();
        }
    }

    println!(
        "file elifulator.py in {}s (blazingly fast)",
        now.elapsed().as_secs_f64()
    )
}
