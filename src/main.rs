mod channel;
mod css;
mod decoder;
mod graph;
mod hgp;
mod matrix;
mod mode;
mod node_math;
mod random_ldpc;

mod sim_classic;
mod sim_css;
mod sim_jd;

fn main() {
    let sim = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Usage: cargo run -- [classic|jd|css]");
        std::process::exit(1);
    });

    match sim.as_str() {
        "classic" => sim_classic::run(),
        "jd" => sim_jd::run(),
        "css" => sim_css::run(),

        _ => {
            eprintln!("Unknown simulation: {}", sim);
            eprintln!("Usage: cargo run -- [classic|jd|css]");
            std::process::exit(1);
        }
    }
}
