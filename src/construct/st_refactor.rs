symbol
data
block


#[derive(Debug, Clone, PartialEq)]
pub enum AST {
    Symbol(SharedSymbol),
    Data(Data),

    Block(Vec<Spanned<AST>>),
    Form(Vec<Spanned<AST>>),
    Group(Box<Spanned<AST>>),

    Pattern(ASTPattern),
    ArgPattern(ArgPattern),

    Label(Spanned<SharedSymbol>, Box<Spanned<AST>>),
    Tuple(Vec<Spanned<AST>>),
    Record(Vec<Spanned<AST>>),

    Is {
        field:      Box<Spanned<AST>>,
        expression: Box<Spanned<AST>>,
    },

    Assign {
        pattern:    Box<Spanned<ASTPattern>>,
        expression: Box<Spanned<AST>>,
    },
    Lambda {
        pattern:    Box<Spanned<ASTPattern>>,
        expression: Box<Spanned<AST>>,
    },
    Composition {
        argument: Box<Spanned<AST>>,
        function: Box<Spanned<AST>>,
    },

    Syntax {
        arg_pat:    Box<Spanned<ArgPattern>>,
        expression: Box<Spanned<AST>>,
    },

    Type {
        label: Spanned<SharedSymbol>,
        type_: Box<Spanned<Type>>,
    },

    // TODO: Use a symbol or the like?
    FFI {
        name:       String,
        expression: Box<Spanned<AST>>,
    },
}

pub enum CST {
    Symbol(SharedSymbol),
    Data(Data),
    Block(Vec<Spanned<CST>>),
    Assign {
        pattern:    Box<Spanned<CSTPattern>>,
        expression: Box<Spanned<CST>>,
    },
    Lambda {
        pattern:    Box<Spanned<CSTPattern>>,
        expression: Box<Spanned<CST>>,
    },
    Call {
        fun: Box<Spanned<CST>>,
        arg: Box<Spanned<CST>>,
    },
    Label(Spanned<SharedSymbol>, Box<Spanned<CST>>),
    Tuple(Vec<Spanned<CST>>),
    FFI {
        name:       String,
        expression: Box<Spanned<CST>>,
    },
}

pub enum SST {
    Symbol(UniqueSymbol),
    Data(Data),
    Block(Vec<Spanned<SST>>),
    Assign {
        pattern:    Box<Spanned<SSTPattern>>,
        expression: Box<Spanned<SST>>,
    },
    Lambda {
        pattern:    Box<Spanned<SSTPattern>>,
        expression: Box<Spanned<SST>>,
        scope:      Scope,
    },
    Call {
        fun: Box<Spanned<SST>>,
        arg: Box<Spanned<SST>>,
    },
    Label(UniqueSymbol, Box<Spanned<SST>>),
    Tuple(Vec<Spanned<SST>>),
    FFI {
        name:       String,
        expression: Box<Spanned<SST>>,
    },
}
