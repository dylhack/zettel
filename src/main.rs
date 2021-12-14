use chrono::Local;

fn get_now() -> (String, String) {
    // The formats are composed by following the strftime specifiers
    // https://docs.rs/chrono/latest/chrono/format/strftime/index.html#specifiers
    // regular_format = "20211214134505"
    // spaced_format = "2021-12-14 134505"
    let regular_format = "%Y%m%d%H%M%S";
    let spaced_format = "%Y-%m-%d %H%M%S";

    let now = Local::now();
    let regular = now.format(regular_format).to_string();
    let spaced = now.format(spaced_format).to_string();

    return (regular, spaced);
}

fn main() {
    let (regular, spaced) = get_now();

    println!("{} ({})", regular, spaced)
}
