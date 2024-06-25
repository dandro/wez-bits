use app::run_app;

mod app;

fn main() {
    match run_app() {
        Err(err) => println!("{}", &err.to_string()),
        Ok(_) => (),
    }
}
