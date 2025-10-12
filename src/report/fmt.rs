use colored::{ColoredString, Colorize};

pub(crate) fn format_index<T: Into<String>>(index: T) -> ColoredString {
    index.into().truecolor(0x6b, 0xa5, 0xf8).bold()
}

pub(crate) fn format_number(min: u64, max: u64, val: u64) -> ColoredString {
    if min == max {
        return format!("{val}").into();
    }

    let mut ratio = ((val - min) as f64) / ((max - min) as f64);
    ratio = ratio.clamp(0.0, 1.0);

    let red = (ratio * 255.0) as u8;
    let green = ((1.0 - ratio) * 255.0) as u8;

    format!("{val}").truecolor(red, green, 170).bold()
}

pub(crate) fn format_pct(min: f64, max: f64, val: f64) -> ColoredString {
    if min == max {
        return format!("{val:05.2}").into();
    }

    let mut ratio = (val - min) / (max - min);
    ratio = ratio.clamp(0.0, 1.0);

    let red = (ratio * 255.0) as u8;
    let green = ((1.0 - ratio) * 255.0) as u8;

    format!("{val:05.2}").truecolor(red, green, 170).bold()
}
