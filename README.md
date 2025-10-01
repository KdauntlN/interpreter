# SPEC GRAMMAR
* Program ::= Statement* ;
* Statement ::= "print" Expr ";" ;
* Expr ::= Term { ("+" | "-") Term } ;
* Term ::= Factor { ("*" | "/") Factor } ;
* Factor ::= Literal | "(" Expr ")" ;