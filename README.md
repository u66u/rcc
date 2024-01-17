Example usage of the parser:

Input:
```
int add(int a, int b) {
    return a + b;
}
```
Returned AST:

```
FunctionDefinition {
    return_type: TypeSpecifier::Int,
    name: "add".to_string(),
    parameters: vec![
        Parameter { type_specifier: TypeSpecifier::Int, name: "a".to_string() },
        Parameter { type_specifier: TypeSpecifier::Int, name: "b".to_string() }
    ],
    body: CompoundStatement {
        statements: vec![

            Statement::ReturnStmt(
                Expression::BinaryOp {
                    left: Box::new(Expression::Identifier("a".to_string())),
                    operator: Operator::Add,
                    right: Box::new(Expression::Identifier("b".to_string()))
                }
            )
        ]
    }
}
```
TODO:
- Separate lexer?
- clang-like AST?