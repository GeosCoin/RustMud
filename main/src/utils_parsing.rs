
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
