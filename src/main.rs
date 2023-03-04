use clap::Parser;

/// Simple program to greet a person

// These attributes apply only to the item and not the crate.
// https://doc.rust-lang.org/rust-by-example/attribute.html
// Debug makes the {:?} formatter available.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// Name of the person to greet
   #[arg(short, long)]
   name: String,

   /// Number of times to greet
   #[arg(short, long, default_value_t = 1)]
   count: u8,
}

fn main() {
   let args = Args::parse();

   for _ in 0..args.count {
       println!("Hello {}!", args.name)
   }
}
