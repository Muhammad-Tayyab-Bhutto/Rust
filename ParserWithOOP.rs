#[derive(Debug)]
enum ASTNode {
    ClassNode {
        name: String,
        parent_class: Option<String>,
        methods: Vec<ASTNode>,
        fields: Vec<String>, // field names
    },
    MethodNode {
        name: String,
        parameters: Vec<String>, // method parameters
        body: Vec<ASTNode>, // method body (a list of statements)
    },
    VarDeclNode { var_name: String, value: ASTNode },
    AssignNode { var_name: String, value: ASTNode },
    IfNode { condition: Box<ASTNode>, body: Vec<ASTNode>, else_body: Option<Vec<ASTNode>> },
    WhileNode { condition: Box<ASTNode>, body: Vec<ASTNode> },
    PrintNode(String),
    ReturnNode(Box<ASTNode>),
    CallNode { object: Box<ASTNode>, method: String, args: Vec<ASTNode> },
    Identifier(String),
    Number(i64),
    StringLiteral(String),
}

struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens,
            position: 0,
        }
    }

    fn current_token(&self) -> &Token {
        &self.tokens[self.position]
    }

    fn next_token(&mut self) {
        self.position += 1;
    }

    fn parse(&mut self) -> Vec<ASTNode> {
        let mut ast = Vec::new();

        while self.position < self.tokens.len() && *self.current_token() != Token::EOF {
            ast.push(self.parse_statement());
        }

        ast
    }

    fn parse_statement(&mut self) -> ASTNode {
        match self.current_token() {
            Token::Darja => self.parse_class(),
            Token::Var => self.parse_var_decl(),
            Token::Agar => self.parse_if(),
            Token::Jabtak => self.parse_while(),
            Token::Wapas => self.parse_return(),
            Token::Identifier(_) => self.parse_expression(),
            _ => panic!("Unexpected token: {:?}", self.current_token()),
        }
    }

    fn parse_class(&mut self) -> ASTNode {
        self.next_token(); // Skip 'darja'

        if let Token::Identifier(class_name) = self.current_token() {
            self.next_token(); // Consume class name
            let mut parent_class = None;

            if let Token::Super = self.current_token() {
                self.next_token(); // Consume 'baWaqt'
                if let Token::Identifier(parent_name) = self.current_token() {
                    parent_class = Some(parent_name.clone());
                    self.next_token(); // Consume parent class name
                }
            }

            self.expect_token(Token::LBrace);

            let mut methods = Vec::new();
            let mut fields = Vec::new();

            while let Token::Identifier(_) = self.current_token() {
                methods.push(self.parse_method());
            }

            self.expect_token(Token::RBrace);

            ASTNode::ClassNode {
                name: class_name.clone(),
                parent_class,
                methods,
                fields,
            }
        } else {
            panic!("Expected class name after 'darja'");
        }
    }

    fn parse_method(&mut self) -> ASTNode {
        self.next_token(); // Skip 'tareeqa'

        if let Token::Identifier(method_name) = self.current_token() {
            self.next_token(); // Consume method name
            self.expect_token(Token::LParen);

            let mut parameters = Vec::new();
            while let Token::Identifier(param) = self.current_token() {
                parameters.push(param.clone());
                self.next_token();
                if let Token::Comma = self.current_token() {
                    self.next_token(); // Skip comma
                }
            }

            self.expect_token(Token::RParen);
            self.expect_token(Token::LBrace);

            let mut body = Vec::new();
            while let Token::RBrace = self.current_token() {
                body.push(self.parse_statement());
            }

            self.expect_token(Token::RBrace);

            ASTNode::MethodNode {
                name: method_name.clone(),
                parameters,
                body,
            }
        } else {
            panic!("Expected method name after 'tareeqa'");
        }
    }

    fn parse_if(&mut self) -> ASTNode {
        self.next_token(); // Skip 'agar'
        let condition = self.parse_expression();
        self.expect_token(Token::LBrace);

        let body = self.parse_block();
        let else_body = if let Token::Warna = self.current_token() {
            self.next_token();
            self.expect_token(Token::LBrace);
            Some(self.parse_block())
        } else {
            None
        };

        ASTNode::IfNode {
            condition: Box::new(condition),
            body,
            else_body,
        }
    }

    fn parse_while(&mut self) -> ASTNode {
        self.next_token(); // Skip 'jabtak'
        let condition = self.parse_expression();
        self.expect_token(Token::LBrace);
        let body = self.parse_block();

        ASTNode::WhileNode {
            condition: Box::new(condition),
            body,
        }
    }

    fn parse_return(&mut self) -> ASTNode {
        self.next_token(); // Skip 'wapas'
        let expr = self.parse_expression();
        ASTNode::ReturnNode(Box::new(expr))
    }

    fn parse_block(&mut self) -> Vec<ASTNode> {
        let mut block = Vec::new();
        while *self.current_token() != Token::RBrace {
            block.push(self.parse_statement());
        }
        self.next_token(); // Skip closing brace
        block
    }

    fn parse_expression(&mut self) -> ASTNode {
        // Handle numbers, variables, method calls, etc.
        match self.current_token() {
            Token::Number(value) => {
                self.next_token();
                ASTNode::Number(*value)
            }
            Token::Identifier(var_name) => {
                self.next_token(); // Move to next token
                ASTNode::Identifier(var_name.clone())
            }
            _ => panic!("Unexpected expression"),
        }
    }

    fn expect_token(&mut self, expected: Token) {
        if *self.current_token() == expected {
            self.next_token();
        } else {
            panic!("Expected {:?}, found {:?}", expected, self.current_token());
        }
    }
}
