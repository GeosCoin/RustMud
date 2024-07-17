use core::fmt;
use std::{fmt::Debug, str::FromStr};




pub fn get_key_pair(s: &str, 
    key: &mut String, val: &mut String){
    let separator: usize = match s.find('='){
        Some(a) => a,
        None => 0
    };

	if (separator == 0) {
		*key = "".to_string();
		*val = "".to_string();
		return; // not found
	}

	*key = (&s[0..separator]).to_string();
	*val = (&s[separator+1 ..s.len()]).to_string();
	*key = key.trim().to_string();
	*val = val.trim().to_string();
}

pub fn get_section_title(s: &str) -> String {
    let bracket: usize = match s.find(']'){
        Some(a) => a,
        None => 0
    };

	if bracket == 0 {
        return "".to_string(); // not found
    }

    return (&s[1..bracket]).to_string();
}

pub fn to_int<T>(s: &str) -> T
where 
    T: TryInto<isize> + FromStr + fmt::Debug,
    <T as FromStr>::Err: Debug
 {
	s.parse::<T>().unwrap()
}