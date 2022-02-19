mod utils;
mod constants;

fn main() {
    println!("And it begins...");
    if let Err(msg) = utils::validate_env_presence() {
        panic!("{}", msg);
    };
}
