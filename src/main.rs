fn main() {

	// Read expression
	let mut s: String = String::new();
	std::io::stdin().read_line(&mut s);

	// Remove whitespace
	s.retain(|c| !c.is_whitespace());	

	// Split into tokens and store in vector
    let tokens: Vec<String> = tokenize(s);

    println!("{:?}", tokens);
}

fn tokenize(s: String) -> Vec<String>{
    let mut result = Vec::new();
    let mut copied = 0;
    for (index, matched) in s.match_indices(|c: char| c == '+' || c=='-') {
        if copied != index {
            // Append delimiter to vector
            // Convert slice to string, and append
            result.push(s[copied..index].to_string());
        }
        // Append text between delimiters to vector
        result.push(matched.to_string());
        // Copied match.len() characters starting from index 
        // => copied up to character index+match.len() from start of string
        copied = index + matched.len();
    }

    // Not all characters copied
    if copied < s.len() {
        // Append to vector the text from last delimiter to end of string
        result.push(s[copied..].to_string());
    }

    result
}