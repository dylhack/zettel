use chrono::prelude::{
    DateTime,
    Local,
};

fn get_now() -> (String, String) {
    let regular_format = "%Y%m%d%H%M";
    let spaced_format = "%Y-%m-%d %H%M";

    let now: DateTime<Local> = Local::now();
    let regular = now.format(regular_format);
    let spaced = now.format(spaced_format);
    let regular_str = regular.to_string();
    let spaced_str = spaced.to_string();

    return (regular_str, spaced_str);
}

fn main() {
    let (regular, spaced) = get_now();

    println!("{} ({})", regular, spaced)
}

