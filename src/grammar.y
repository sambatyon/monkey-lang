%token LET
%token IDENT
%token EOF

%%

program: stmts EOF

stmts: /* nothing */
  | stmt ';' stmts
  ;

stmt:
    letstmt
  | returnstmt;

letstmt: LET IDENT '=' expr;

returnstmt: RETURN expr;

expr: ;

%%
