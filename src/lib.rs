pub fn input<'a>(s: impl Into<Option<&'a str>>) -> String {
	let s_into = s.into();
	
	if s_into != None {
		println!("{}", s_into.unwrap());
	}
	
	let mut val = String::new();
	std::io::stdin().read_line(&mut val).expect("Something went wrong with the input.");
	
	return val[0..val.len() - 1].to_string();
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn input_test() {
        let a = input(None);
        let b = input("Hello there");
        
        assert_eq!(a, "hello\r");
        assert_eq!(b, "test\r");
    }
}