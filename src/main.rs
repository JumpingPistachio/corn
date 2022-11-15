use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]

// Defines arguments for the command(--help, etc).
struct Args {
    /// Base Operand or 'Start' number.
    #[arg(short = 's', long = "start")]
    start_num: f64,

    /// Operators a/Add, s/Subtact, m/Multiply, d/Divide
    #[arg(short = 'o', long = "operator")]
    operator: String,

    /// End Operand or 'End' number.
    #[arg(short = 'e', long = "end")]
    end_num: f64
}

fn main() {
    // Parses arguments inputted into variables for the program.
    let args: Args = Args::parse();
    // Makes the 'operator' string all lowercase for easier usage.
    let operator: String = args.operator.to_lowercase();

    if operator == "a" || operator == "add" {
        println!("{}", args.start_num + args.end_num);
    }else if operator == "s" || operator == "subtract" {
        println!("{}", args.start_num - args.end_num);
    }else if operator == "d" || operator == "divide" {
        println!("{}", args.start_num / args.end_num);
    }else if operator == "m" || operator == "multiply" {
        println!("{}", args.start_num - args.end_num);
    }else {
        println!("Not a valid argument.");
    }
}
