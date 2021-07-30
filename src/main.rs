fn main() {
	// Read expression
	let mut expression: String = String::new();
	std::io::stdin().read_line(&mut expression);

	// Remove whitespace
	expression.retain(|c| !c.is_whitespace());

	// Split into tokens and store in vector
	let tokens: Vec<String> = tokenize(expression);

	// Position of lookahead token in vector
	let mut lookahead: usize = 0;

	let result = expr(&mut lookahead, &tokens);

	println!("Result: {}\n", result);
}

fn tokenize(s: String) -> Vec<String> {
	let mut result = Vec::new();
	let mut copied = 0;
	for (index, matched) in s.match_indices(|c: char| {
		c == '+' || c == '-' || c == '*' || c == '/' || c == '(' || c == ')'
	}) {
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
		// Append to vector the remaining text from last delimiter to end of string
		result.push(s[copied..].to_string());
	}

	result
}

// Returns true if lookahead type matches token type given as argument
fn matches(tok_type: &str, lookahead: usize, tokens: &Vec<String>) -> bool {
	// If token doesn't exist, return false
	if lookahead >= tokens.len() {
		return false;
	}
	match tok_type {
		// Lookahead is numeric
		"num" => tokens[lookahead].parse::<i32>().is_ok(),
		// Valid factor (num, parenthesis, or -factor2)
		"factor" => {
			tokens[lookahead].parse::<i32>().is_ok()
				|| tokens[lookahead] == "("
				|| tokens[lookahead] == ")"
				|| (tokens[lookahead] == "-" && matches("factor2", lookahead + 1, tokens))
		}
		"factor2" => {
			tokens[lookahead].parse::<i32>().is_ok()
				|| tokens[lookahead] == "("
				|| tokens[lookahead] == ")"
		}
		"left_par" => tokens[lookahead] == "(",
		"right_par" => tokens[lookahead] == ")",
		"minus" => tokens[lookahead] == "-",
		"plus" => tokens[lookahead] == "+",
		"mult" => tokens[lookahead] == "*",
		"div" => tokens[lookahead] == "/",
		_ => false,
	}
}

fn expr(lookahead: &mut usize, tokens: &Vec<String>) -> i32 {
	let term_value: i32 = term(lookahead, tokens);
	return expr_cont(lookahead, tokens, term_value);
}

fn expr_cont(lookahead: &mut usize, tokens: &Vec<String>, value_so_far: i32) -> i32 {
	// Last component of string is expr_cont,
	// check if end of original expression reached
	if *lookahead == tokens.len() {
		return value_so_far;
	}

	// + term expr_cont
	if matches("plus", *lookahead, tokens) {
		*lookahead = *lookahead + 1;
		let addition_result: i32 = value_so_far + term(lookahead, tokens);
		return expr_cont(lookahead, tokens, addition_result);
	}

	// - term expr_cont
	if matches("minus", *lookahead, tokens) {
		*lookahead = *lookahead + 1;
		let sub_result: i32 = value_so_far - term(lookahead, tokens);
		return expr_cont(lookahead, tokens, sub_result);
	}

	// ε
	return value_so_far;
}

fn term(lookahead: &mut usize, tokens: &Vec<String>) -> i32 {
	if matches("factor", *lookahead, tokens) {
		let factor_value: i32 = factor(lookahead, tokens);
		*lookahead = *lookahead + 1;
		return term_cont(lookahead, tokens, factor_value);
	}
	panic!("Error! Invalid expression\n");
}

fn term_cont(lookahead: &mut usize, tokens: &Vec<String>, value_so_far: i32) -> i32 {
	// Last component of string is expr_cont, last component of expr_cont is term_cont,
	// check here if end of original expression string reached, before using *lookahead as index
	if *lookahead == tokens.len() {
		return value_so_far;
	}

	// * factor term_cont
	if matches("mult", *lookahead, tokens) {
		*lookahead = *lookahead + 1;
		let mult_val: i32 = value_so_far * factor(lookahead, tokens);
		*lookahead = *lookahead + 1;
		return term_cont(lookahead, tokens, mult_val);
	}

	// / factor term_cont
	if matches("div", *lookahead, tokens) {
		*lookahead = *lookahead + 1;
		let divisor: i32 = factor(lookahead, tokens);
		*lookahead = *lookahead + 1;
		if divisor == 0 {
			panic!("Error! Division by zero\n");
		}
		return value_so_far / divisor;
	}

	// ε
	return value_so_far;
}

fn factor(lookahead: &mut usize, tokens: &Vec<String>) -> i32 {
	// (expr)
	if matches("left_par", *lookahead, tokens) {
		*lookahead = *lookahead + 1;
		let parenthesis_value: i32 = expr(lookahead, tokens);
		if !matches("right_par", *lookahead, tokens) {
			panic!("Error! Invalid expression\n");
		}
		return parenthesis_value;
	}
	// num

	if matches("num", *lookahead, tokens) {
		return tokens[*lookahead].parse().unwrap();
	}

	// -factor2
	if matches("minus", *lookahead, tokens) {
		*lookahead = *lookahead + 1;
		return factor2(lookahead, tokens) * -1;
	}

	panic!("Error! Invalid expression\n");
}

fn factor2(lookahead: &mut usize, tokens: &Vec<String>) -> i32 {
	// (expr)
	if matches("left_par", *lookahead, tokens) {
		*lookahead = *lookahead + 1;
		let parenthesis_value: i32 = expr(lookahead, tokens);
		if !matches("right_par", *lookahead, tokens) {
			panic!("Error! Invalid expression(unclosed parentheses)\n");
		}
		return parenthesis_value;
	}
	// num
	if matches("num", *lookahead, tokens) {
		return tokens[*lookahead].parse().unwrap();
	}

	panic!("Error! Invalid expression\n");
}
