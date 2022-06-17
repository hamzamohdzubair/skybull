fn main() {
    if let Err(e) = skybull::get_args()
        .and_then(skybull::run) {
            eprintln!("{}", e);
            std::process::exit(1);
    }
}
