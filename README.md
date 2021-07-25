# ArithmeticRecDescent

Simple program for arithmetic expression evaluation using recursive descent parsing.

## LL(1) Grammar

expr -> term expr_cont
expr_cont -> + term expr_cont
           | - term expr_cont
           | ε
term -> factor term_cont
term_cont -> * factor term_cont
           | / factor term_cont
           | ε
factor -> (expr)
        | num
num -> digit num_cont
num_cont -> digit num_cont | ε