# GOAL

```rs
let a = 1;
let b = 2;
let c = a + b;
print(c);
```

Tokens

```
KW_let, Identifier('a'), Equal, NumberLiteral(1), SemiColon,
KW_let, Identifier('b'), Equal, NumberLiteral(2), SemiColon,
KW_let, Identifier('c'), Equal, Identifier('a'), Plus, Identifier('b'), SemiColon,
Identifier('print'), LeftParen, Identifier('c'), RightParen, SemiColon
```

Parse Tree

```
{
    statements: [
        LetStatement {
            id: Identifier {
                name: 'a'
            },
            expr: NumberLiteral {
                value: '1'
            }
        },
        LetStatement {
            id: Identifier {
                name: 'b'
            },
            expr: NumberLiteral {
                value: '2'
            }
        },
        LetStatement {
            id: Identifier {
                name: 'c'
            },
            expr: BinaryOperation {
                op: '+',
                lhs: Identifier {
                    name: 'a'
                },
                rhs: Identifier {
                    name: 'b'
                }
            }
        },
        FunctionCall {
            id: Identifier {
                name: 'print'
            }
            args: [
                Identifier {
                    name: 'c'
                }
            ]
        }
    ]

}
```
