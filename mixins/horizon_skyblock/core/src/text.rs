// Minecraft formatting, numeric and duration parsing. Input is always bounded.
pub fn clean(input: &str) -> String {
    let mut out = String::new();
    let mut chars = input.chars().take(8192);
    while let Some(c) = chars.next() {
        if c == '§' {
            chars.next();
            continue;
        }
        if c == '\n' || c == '\t' || !c.is_control() {
            out.push(c);
        }
    }
    out
}
pub fn short(s: &str, n: usize) -> String {
    clean(s).chars().take(n).collect()
}
pub fn number(input: &str) -> Option<f64> {
    let s = input.trim();
    if s.is_empty() || s.len() > 64 {
        return None;
    }
    let (s, mul) = match s.as_bytes().last()?.to_ascii_lowercase() {
        b'k' => (&s[..s.len() - 1], 1e3),
        b'm' => (&s[..s.len() - 1], 1e6),
        b'b' => (&s[..s.len() - 1], 1e9),
        b't' => (&s[..s.len() - 1], 1e12),
        _ => (s, 1.0),
    };
    if !s
        .bytes()
        .all(|c| c.is_ascii_digit() || c == b',' || c == b'.')
    {
        return None;
    }
    let mut decimal = s.split('.');
    let whole = decimal.next()?;
    if let Some(frac) = decimal.next()
        && (frac.is_empty() || !frac.bytes().all(|c| c.is_ascii_digit()))
    {
        return None;
    }
    if decimal.next().is_some() {
        return None;
    }
    if whole.contains(',') {
        let groups: Vec<_> = whole.split(',').collect();
        if groups[0].is_empty()
            || groups[0].len() > 3
            || groups.iter().skip(1).any(|g| g.len() != 3)
        {
            return None;
        }
    }
    let v = s.replace(',', "").parse::<f64>().ok()? * mul;
    (v.is_finite() && (0.0..=1e15).contains(&v)).then_some(v)
}
pub fn first_number(s: &str) -> Option<f64> {
    s.split(|c: char| {
        !c.is_ascii_digit()
            && !matches!(c, ',' | '.' | 'k' | 'K' | 'm' | 'M' | 'b' | 'B' | 't' | 'T')
    })
    .filter(|s| s.starts_with(|c: char| c.is_ascii_digit()))
    .find_map(number)
}
pub fn duration(s: &str) -> Option<u64> {
    let s = s.trim();
    if s.is_empty() || s.len() > 64 {
        return None;
    }
    let seconds = if s.contains(':') {
        let parts: Vec<_> = s.split(':').collect();
        if !(2..=3).contains(&parts.len()) {
            return None;
        }
        let mut sum = 0u64;
        for (i, p) in parts.iter().enumerate() {
            let v = p.parse::<u64>().ok()?;
            if i > 0 && v >= 60 {
                return None;
            }
            sum = sum.checked_mul(60)?.checked_add(v)?;
        }
        sum
    } else if s.bytes().all(|b| b.is_ascii_digit()) {
        s.parse().ok()?
    } else {
        let mut sum = 0u64;
        let mut digits = String::new();
        let mut last = 0;
        for c in s.chars().filter(|c| !c.is_whitespace()) {
            if c.is_ascii_digit() {
                digits.push(c);
                continue;
            }
            let scale = match c {
                'd' => 86400,
                'h' => 3600,
                'm' => 60,
                's' => 1,
                _ => return None,
            };
            if last != 0 && scale >= last {
                return None;
            }
            last = scale;
            let v = digits.parse::<u64>().ok()?;
            digits.clear();
            sum = sum.checked_add(v.checked_mul(scale)?)?;
        }
        if !digits.is_empty() {
            return None;
        }
        sum
    };
    (seconds > 0 && seconds <= 604800).then_some(seconds)
}
pub fn clock(s: u64) -> String {
    if s >= 3600 {
        format!("{}:{:02}:{:02}", s / 3600, s / 60 % 60, s % 60)
    } else {
        format!("{}:{:02}", s / 60, s % 60)
    }
}
pub fn compact(n: f64) -> String {
    for (m, s) in [(1e12, "T"), (1e9, "B"), (1e6, "M"), (1e3, "k")] {
        if n >= m {
            return format!("{:.2}{s}", n / m);
        }
    }
    format!("{n:.0}")
}
pub fn ratio_before(s: &str, marker: char) -> Option<(f64, f64)> {
    let end = s.find(marker)?;
    let s = &s[..end];
    let token = s.split_whitespace().next_back()?;
    let (a, b) = token.split_once('/')?;
    let (a, b) = (number(a)?, number(b)?);
    (b > 0.0).then_some((a, b))
}
