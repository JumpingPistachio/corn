use clap::Parser;

#[derive(Parser, Debug)]
#[command(author = "JumpingPistachio", version, about = "A simple calculator built with rust.", long_about = None)]

// Defines arguments for the command(--help, etc).
struct Args {
    /// Base Operand or 'Start' number.
    #[arg(short = 's', long = "start")]
    start_num: f64,

    /// Operators (a/Add, s/Subtact, m/Multiply, d/Divide, q/SquareRoot)
    #[arg(short = 'o', long = "operator", default_value = "add")]
    operator: String,

    /// End Operand or 'End' number.
    #[arg(short = 'e', long = "end", default_value_t = 0.0)]
    end_num: f64
}

fn main() {
    // Parses arguments inputted into variables for the program.
    let args: Args = Args::parse();
    // Makes the 'operator' string all lowercase for easier usage.
    let operator: String = args.operator.to_lowercase();

    // Using if block instead of match case due to errors.
    if operator == "a" || operator == "add" {
        println!("{}", args.start_num + args.end_num);
    }else if operator == "s" || operator == "subtract" {
        println!("{}", args.start_num - args.end_num);
    }else if operator == "d" || operator == "divide" {
        println!("{}", args.start_num / args.end_num);
    }else if operator == "m" || operator == "multiply" {
        println!("{}", args.start_num - args.end_num);
    }else if operator == "p" || operator == "percent"{
        println!("{}", args.start_num % args.end_num);
    }else if operator == "q" || operator == "squareroot"{
        println!("{}", args.start_num.sqrt());
    }else {
        println!("Not a valid argument.");
    }
}
