macro_rules! fastio {
    ($sc:ident) => {
        let mut _input = Vec::new();
        io::stdin().read_to_end(&mut _input).unwrap();
        let mut _idx = 0;
        let mut $sc = || -> &str {
            while _idx < _input.len() && _input[_idx].is_ascii_whitespace() {
                _idx += 1;
            }
            let start = _idx;
            while _idx < _input.len() && !_input[_idx].is_ascii_whitespace() {
                _idx += 1;
            }
            unsafe { std::str::from_utf8_unchecked(&_input[start.._idx]) }
        };
    };
}

fn next<'a, F, T>(sc: &mut F) -> T
where
    F: FnMut() -> &'a str,
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    sc().parse::<T>().unwrap()
}
