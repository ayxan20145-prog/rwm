mod config;
mod rwm;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    rwm::run()
}
