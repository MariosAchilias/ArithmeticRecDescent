# ArithmeticRecDescent

Simple program for arithmetic expression evaluation using recursive descent parsing.

## Usage

    cargo build
    cd target/debug/
    ./arithmetic_rec_descent
    1+2+3+4


## LL(1) Grammar

    expr -> term expr_cont.
    expr_cont -> + term expr_cont
               | - term expr_cont
               | ε.
    term -> factor term_cont.
    term_cont -> * factor term_cont
               | / factor term_cont
               | ε.
    factor -> (expr)
            | num
    	| -factor2.
    factor2 -> (expr)
    	 | num.
    num -> digit num_cont.
    num_cont -> digit num_cont | ε.
