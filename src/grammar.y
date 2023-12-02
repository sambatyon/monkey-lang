%token EOF
%token IDENT
%token LET
%token RETURN

%%

program : stmts EOF;

stmts : /* nothing */
  | stmt ';' stmts
  ;

stmt :
    letstmt
  | returnstmt;

letstmt : LET IDENT '=' expr;

returnstmt : RETURN expr;

expr : ;

%%
