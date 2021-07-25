fn main() {

	// Read expression
	let mut s: String = String::new();
	std::io::stdin().read_line(&mut s);

	// Remove whitespace
	s.retain(|c| !c.is_whitespace());	

	// Split into tokens and store in vector
    let tokens: Vec<String> = tokenize(s);

    // Position of lookahead token in vector
    let mut lookahead: usize = 0;

    let result = expr(&mut lookahead, &tokens);

    println!("{:?}", tokens);
    println!("Result: {}", result);
}

fn tokenize(s: String) -> Vec<String>{
    let mut result = Vec::new();
    let mut copied = 0;
    for (index, matched) in s.match_indices(|c: char| c == '+' || c=='-' || c=='*' || c=='/' || c=='(' || c==')') {
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

// Returns true if lookahead type matches token type given as argument
fn matches(tok_type: &str, lookahead: usize, tokens: &Vec<String>) -> bool{
    match tok_type{
        // Lookahead is numeric
        "num" => tokens[lookahead].parse::<i32>().is_ok(),
        // Valid operator
        "op" => {tokens[lookahead] == "+" || tokens[lookahead] == "-"},
        // Valid factor (either num or parenthesis)
        "factor" => tokens[lookahead].parse::<i32>().is_ok() || tokens[lookahead] == "(" || tokens[lookahead] == ")" || tokens[lookahead] == "-",
        // Empty string ε
        "empty" => lookahead == tokens.len(),
        // Left parenthesis
        "left_par" => tokens[lookahead] == "(",
        // Right parenthesis
        "right_par" => tokens[lookahead] == ")",
        _ => false
    }

}

fn expr(lookahead:&mut usize, tokens: &Vec<String>) -> i32{
    println!("In expr lookahead:{} \n", *lookahead);
    let term_value:i32 = term(lookahead, tokens);
    return expr_cont(lookahead, tokens, term_value);
}

fn expr_cont(lookahead:&mut usize, tokens: &Vec<String>, value_so_far: i32) -> i32{
    println!("In expr_cont lookahead: {}\n", *lookahead);
    if *lookahead == tokens.len(){
        return value_so_far;
    }
    
    if tokens[*lookahead] == "+"{
        *lookahead = *lookahead + 1;
        let addition_result: i32 = value_so_far + term(lookahead, tokens);
        return expr_cont(lookahead, tokens, addition_result);
    }

    if tokens[*lookahead] == "-"{
        *lookahead = *lookahead + 1;
        let sub_result: i32 = value_so_far - term(lookahead, tokens);
        return expr_cont(lookahead, tokens, sub_result);
    }

    value_so_far

}

fn term(lookahead:&mut usize, tokens: &Vec<String>) -> i32{
    if matches("factor", *lookahead, tokens){
        let factor_value:i32 = factor(lookahead, tokens);
        *lookahead = *lookahead + 1;
        return term_cont(lookahead, tokens, factor_value);
    }
    panic!("Error! Improperly formatted expression");
}

fn term_cont(lookahead:&mut usize, tokens: &Vec<String>, value_so_far: i32) -> i32{
    
    // End of string reached, check before using *lookahead as index
    if matches("empty", *lookahead, tokens){
        return value_so_far;
    }

    if tokens[*lookahead] == "*"{
        *lookahead = *lookahead + 1;
        let mult_val: i32 = value_so_far * factor(lookahead, tokens);
        *lookahead = *lookahead + 1;
        return term_cont(lookahead, tokens, mult_val);
    }
    
    if tokens[*lookahead] == "/"{
        *lookahead = *lookahead + 1;
        let fact_val: i32 = factor(lookahead, tokens);
        *lookahead = *lookahead + 1;
        return value_so_far / fact_val;
    }

    // ε case
    value_so_far
}

fn factor(lookahead:&mut usize, tokens: &Vec<String>) -> i32{
    if matches("left_par", *lookahead, tokens){
        *lookahead = *lookahead + 1;
        let parenthesis_value: i32 = expr(lookahead, tokens);
        if !matches("right_par", *lookahead, tokens){
            println!("Error! Unclosed parenthesis\n");
            return 0;
        }
        return parenthesis_value;
    }
    if matches("op", *lookahead, tokens){
        if(tokens[*lookahead] == "-"){
            *lookahead = *lookahead+1;
            return tokens[*lookahead].parse::<i32>().unwrap() * -1;
        }
        *lookahead = *lookahead + 1;
    }
    return tokens[*lookahead].parse().unwrap();
}