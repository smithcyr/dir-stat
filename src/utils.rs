const DISPLAY_PREFIX: usize = 3;

fn display_bytes(size: f64, prefix: &str) -> String {
    let precision = if size == size.round() { 0 } else { DISPLAY_PREFIX };
    format!("{size:.precision$} {prefix}B", size = size, precision = precision, prefix = prefix)
}

fn to_prefix<const COUNT: usize>(value: i128, prefixes: [&str; COUNT], thresholds: [u128; COUNT]) -> String 
{
    for (power, prefix) in prefixes.iter().enumerate() {
        let rounding_upper = power + 1;
        let rounding_decimal = if power == 0 { 0 } else { power - 1 };
        if (value.abs() as u128) < thresholds[rounding_upper] {
            let truncated_prefixed_bytes = value as f64 / thresholds[rounding_decimal] as f64;
            let decimal_divisor = thresholds[power - rounding_decimal] as f64;
            return display_bytes(truncated_prefixed_bytes as f64 / decimal_divisor as f64, prefix);
        }
    }
    return display_bytes((value >> (prefixes.len() * 10)) as f64, prefixes[prefixes.len() - 1]);
}

const BINARY_PREFIXES: [&str; 7] = ["", "Ki", "Mi", "Gi", "Ti", "Pi", "Ei"];
const BINARY_THRESHOLDS: [u128; 7] = [1, 1 << 10, 1 << 20, 1 << 30, 1 << 40, 1 << 50, 1 << 60];
pub fn to_binary_prefix(size_in_bytes: i128) -> String {
    return to_prefix(size_in_bytes, BINARY_PREFIXES, BINARY_THRESHOLDS);
}

const DECIMAL_PREFIXES: [&str; 7] = ["", "K", "M", "G", "T", "P", "E"];
const DECIMAL_THRESHOLDS: [u128; 7] = [1, 1_000, 1_000_000, 1_000_000_000, 1_000_000_000_000, 1_000_000_000_000_000, 1_000_000_000_000_000_000];
pub fn to_decimal_prefix(size_in_bytes: i128) -> String {
    return to_prefix(size_in_bytes, DECIMAL_PREFIXES, DECIMAL_THRESHOLDS);
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_binary_prefix() {
        const B: i128 = 1;
        const KB: i128 = B << 10;
        const MB: i128 = KB << 10;
        const GB: i128 = MB << 10;
        assert_eq!(to_binary_prefix(1), "1 B", "B");
        assert_eq!(to_binary_prefix(KB), "1 KiB", "KB");
        assert_eq!(to_binary_prefix(KB - B), "1023 B", "KB - B");
        assert_eq!(to_binary_prefix(KB + B), "1.001 KiB", "KB + B");
        assert_eq!(to_binary_prefix(GB), "1 GiB", "GB");
        assert_eq!(to_binary_prefix(GB - 1), "1024.000 MiB", "GB - 1");
        assert_eq!(to_binary_prefix(GB - KB), "1023.999 MiB", "GB - KB");
        assert_eq!(to_binary_prefix(GB - MB + KB), "1023.001 MiB", "GB - KB");
        assert_eq!(to_binary_prefix(GB - MB), "1023 MiB", "GB - MB");
    }

    #[test]
    fn test_decimal_prefix() {
        assert_eq!(to_decimal_prefix(1), "1 B", "B");
        assert_eq!(to_decimal_prefix((DECIMAL_THRESHOLDS[1] - DECIMAL_THRESHOLDS[0]) as i128), "999 B", "KB - B");
        assert_eq!(to_decimal_prefix(-((DECIMAL_THRESHOLDS[1] - DECIMAL_THRESHOLDS[0]) as i128)), "-999 B", "KB - B");
        assert_eq!(to_decimal_prefix(DECIMAL_THRESHOLDS[1] as i128), "1 KB", "KB");
        assert_eq!(to_decimal_prefix(-(DECIMAL_THRESHOLDS[1] as i128)), "-1 KB", "KB");
        assert_eq!(to_decimal_prefix((DECIMAL_THRESHOLDS[1] + DECIMAL_THRESHOLDS[0]) as i128), "1.001 KB", "KB + B");
        assert_eq!(to_decimal_prefix((DECIMAL_THRESHOLDS[3] - 1) as i128), "1000.000 MB", "GB - 1");
        assert_eq!(to_decimal_prefix((DECIMAL_THRESHOLDS[3] - DECIMAL_THRESHOLDS[0]) as i128), "1000.000 MB", "GB - B");
        assert_eq!(to_decimal_prefix((DECIMAL_THRESHOLDS[3] - DECIMAL_THRESHOLDS[1]) as i128), "999.999 MB", "GB - KB");
        assert_eq!(to_decimal_prefix((DECIMAL_THRESHOLDS[3] - DECIMAL_THRESHOLDS[2] + DECIMAL_THRESHOLDS[1]) as i128), "999.001 MB", "GB - KB");
        assert_eq!(to_decimal_prefix((DECIMAL_THRESHOLDS[3] - DECIMAL_THRESHOLDS[2]) as i128), "999 MB", "GB - MB");
    }
}