#[derive(Debug, Clone)]
pub enum Statement {
    VarDecl(VarDecl),
    DataDecl(DataDecl),
    ConstDecl(ConstDecl),
    GenDecl(GenDecl),
    MainDecl(MainDecl)
}

#[derive(Debug, Clone)]
pub struct VarDecl {
    pub id: String,
    pub value: Expr
}

#[derive(Debug, Clone)]
pub struct DataDecl {
    pub id: String,
    pub args: Vec<String>,
    pub members: Vec<DataMemberDecl>
}

#[derive(Debug, Clone)]
pub struct DataMemberDecl {
    pub id: String,
    pub value: Expr
}

#[derive(Debug, Clone)]
pub struct ConstDecl {
    pub id: String,
    pub value: Expr
}

#[derive(Debug, Clone)]
pub struct GenDecl {
    pub id: String,
    pub value: Expr
}

#[derive(Debug, Clone)]
pub struct MainDecl {
    value: Expr
}

#[derive(Debug, Clone)]
pub enum Expr {
    Id(String),
    IntLiteral(i64),
    FloatLiteral(f64),
    BoolLiteral(bool),
    StringLiteral(String),
    FunctionCall(FunctionCall),
    UnionOp(UnionOp),
    PipeOp(PipeOp)
}

#[derive(Debug, Clone)]
pub struct FunctionCall {}

#[derive(Debug, Clone)]
pub struct UnionOp {
    pub exprs: Vec<Expr>
}

#[derive(Debug, Clone)]
pub struct PipeOp {
    pub exprs: Vec<Expr>
}