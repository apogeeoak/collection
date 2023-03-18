use std::cmp;
use std::fmt::Display;
use std::fmt::Write;

pub fn cell(item: impl Display, width: usize) -> String {
    let item = item.to_string();
    let count = item.chars().take(width + 1).count();

    match count {
        len if len <= width => format!("{:1$}", item, width),
        _ if width <= 3 => item.chars().take(width).collect::<String>(),
        _ => {
            let ellipses = cmp::min(3, width - 3);
            let ellipsis = ".".repeat(ellipses);

            format!("{}{}", item.chars().take(width - ellipses).collect::<String>(), ellipsis)
        }
    }
}

pub struct Row<T: Display> {
    pub output: T,
    pub width: usize,
}
pub fn row<T: Display>(items: impl IntoIterator<Item = Row<T>>) -> String {
    let mut builder = String::new();
    let mut items = items.into_iter();
    if let Some(item) = items.next() {
        builder.push_str(&cell(item.output, item.width));
    }
    for item in items {
        write!(&mut builder, " | {}", &cell(item.output, item.width)).unwrap();
    }
    builder
}

pub fn divider(widths: impl IntoIterator<Item = usize>, divider: Option<&str>) -> String {
    // Set the default divider string.
    let divider = divider.unwrap_or("-");

    let size = widths.into_iter().reduce(|acc, item| acc + item + 3).unwrap_or(0);
    divider.repeat(size)
}
