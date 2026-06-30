mod check;
mod url_generation;

fn main() {
    let url = url_generation::get_download();
    println!("{}", url);
}
