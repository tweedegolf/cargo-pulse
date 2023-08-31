mod crates_io;
mod github;

fn main() {
    let crate_name = std::env::args().nth(1).unwrap();

    println!("{:?}", crates_io::get_crate_info(&crate_name))
}
