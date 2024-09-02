
// You are free to add more error variants if you need them.

#[derive(Debug,PartialEq)]
pub enum AsaErrorKind {
  UndefinedFunction,
  VariableNotDefined(String),
  DivisionByZero,
  NumberOverflow,
  NumberUnderflow,
  Generic(String),
  // added below
  InvalidExpression, 
  UndefinedVariable(String),
  InvalidIdentifier(String),
  EmptyStatement,
  InvalidVariable,
  NoStackFrame,
  UnimplementedNode(String),
  TypeError
}