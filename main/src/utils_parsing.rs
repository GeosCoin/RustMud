use core::fmt;
use std::{fmt::Debug, fs::read_to_string, str::{FromStr, Lines}};


pub fn trim(s: &str, delimiters: &str) -> String {
    let separator: usize = match s.find(delimiters){
        Some(a) => a,
        None => 0
    };
    if separator == 0 {
        return s.to_string();
    }
    (&s[0..separator]).to_string()
}

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
	*val = trim(val, "#");
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
    <T as FromStr>::Err :  Debug
 {
	s.parse::<T>().unwrap()
}

pub fn to_float(s: &str) -> f32 
{
    s.parse::<f32>().unwrap()
}

pub fn strip_carriage_return(s:&str) -> String {
    s.trim_end_matches("\r")
        .trim_end_matches("\n")
        .to_string()
}

pub fn pop_first_string(s:&str) -> (String, String) {
    let mut remains = "".to_string();
    let mut outs = "".to_string();
	let mut seppos = 0;
    let s_clone = strip_carriage_return(s.trim());
    let s_clone = trim(&s_clone, "#");
    
    seppos = match s_clone.find(','){
        Some(a) => a,
        None => 0
    };
    let alt_seppos = match s_clone.find(';'){
        Some(a) => a,
        None => 0
    };

    if (alt_seppos != 0 && alt_seppos < seppos) {
        seppos = alt_seppos; // return the first ',' or ';'
    }
	    
	if (seppos == 0) {
		outs = s_clone;
		remains = "".to_string();
        
	}
	else {
		outs = (&s_clone[0..seppos]).to_string();
		remains = (&s_clone[seppos+1 .. s_clone.len()]).to_string();
	}
	return (outs, remains);
}

pub fn pop_first_int(s:&str) -> (isize, String) {
    let (outs, remains) = pop_first_string(s);
    (to_int(&outs), remains)
}

pub fn pop_first_float(s:&str) -> (f32, String) {
    let (outs, remains) = pop_first_string(s);
    (to_float(&outs), remains)
}

pub fn skip_line(s:&str) -> bool {
    if s.len() == 0 {
        return true;
    }

    if s.trim().starts_with("#") {
        return true;
    }

    return false;
}
