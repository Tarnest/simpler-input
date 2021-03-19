pub fn input() -> String {
	let mut val = String::new();
	std::io::stdin().read_line(&mut val).expect("Something went wrong with the input.");
	
	return val[0..val.len() - 1].to_string();
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn input_test() {
        let a = input();
        let b = input();
        
        assert_eq!(a, "hello");
        assert_eq!(b, "test");
    }
}