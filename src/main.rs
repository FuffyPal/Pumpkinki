mod check;
mod create_path;
mod url_generation;

fn main() {
    let base: String = String::from("pumpkin");
    let path: String = String::from("./PumpkinMC");
    if check::do_you_have_any_pumpkin(path.clone(), base) == 0 {
        println!("this exitis")
    } else if create_path::create_path(path.clone()) == 0 {
        let url = url_generation::get_download();
        println!("{}", url);
    } else {
        panic!("Dont Create PATH:  {}", path)
    }
}
