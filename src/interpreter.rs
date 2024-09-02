/*
Sources:

https://github.com/rust-lang/rust/issues/64727
https://yosefk.com/blog/getting-the-call-stack-without-a-frame-pointer.html


*/


use crate::{identifier, parser::Node};
use std::collections::HashMap;
use crate::error::*;
#[derive(Debug, PartialEq, Clone)]
pub enum Value {
  String(String),
  Number(i32),
  Bool(bool),
}

type Frame = HashMap<String, Value>;

#[derive(Debug)]
pub struct Interpreter {
  // Stack:
  // Each element in the stack is a function stack frame.
  // Crate a new stack frame on function entry.
  // Pop stack frame on function return.
  // Key - Variable name
  // Value - Variable value
  stack: Vec<Frame>,
}


impl Interpreter {

  pub fn new() -> Interpreter {
    Interpreter {
      stack: Vec::new(),
    }
  }

  pub fn exec(&mut self, node: &Node) -> Result<Value,AsaErrorKind> {
    match node {
      Node::Program{children} => {
        for n in children {
          match n {
            Node::Expression{..} |
            Node::VariableDefine{..} |
            Node::String{..} |
            Node::Number{..} |
            Node::Bool{..} => {
              return self.exec(n);
            }
            _ => return Err(AsaErrorKind::UnimplementedNode(format!("Unsupported node type in Program: {:?}", n))),
          }
        }
        Ok(Value::Bool(true))
      },
      
      // Evaluates a mathematical expression based on the elements in the children argument. If the expression is valid, the code evaluates it and returns a new Value object with the resulting value. If the expression is not valid, the code returns an error message.
      Node::MathExpression{name, children} => {
        if children.len() != 2 {
            return Err(AsaErrorKind::InvalidExpression);
        }
    
        let left = self.exec(&children[0])?;
        let right = self.exec(&children[1])?;
    
        let op = match String::from_utf8(name.clone()) {
            Ok(s) => s,
            Err(_) => return Err(AsaErrorKind::InvalidExpression),
        };
    
        match (op.as_str(), left, right) {
            ("+", Value::Number(l), Value::Number(r)) => Ok(Value::Number(l + r)),
            ("-", Value::Number(l), Value::Number(r)) => Ok(Value::Number(l - r)),
            ("*", Value::Number(l), Value::Number(r)) => Ok(Value::Number(l * r)),
            ("/", Value::Number(l), Value::Number(r)) => {
                if r == 0 {
                    Err(AsaErrorKind::DivisionByZero)
                } else {
                    Ok(Value::Number(l / r))
                }
            },
            _ => Err(AsaErrorKind::InvalidExpression),
        }
    },


      
      // Retrieves the value of the identifier from the current frame on the stack. If the variable is defined in the current frame, the code returns its value. If the variable is not defined in the current frame, the code returns an error message.
      Node::Identifier{value} => {
        let identifier_name = match String::from_utf8(value.clone()) {
            Ok(name) => name,
            Err(_) => return Err(AsaErrorKind::InvalidIdentifier(String::from_utf8_lossy(value).to_string())),
        };
    
        if let Some(current_frame) = self.stack.last() {
            if let Some(val) = current_frame.get(&identifier_name) {
                return Ok(val.clone());
            }
        }
    
        Err(AsaErrorKind::UndefinedVariable(identifier_name))
    },
    
      // checking the type of the first element in the children argument and deciding what to do based on that type





      // if the type is a VariableDefine or FunctionReturn node the code runs the run method on that node and returns the result
      Node::Statement{children} => {
        if let Some(first_child) = children.first() {
            self.exec(first_child)
        } else {
            Err(AsaErrorKind::EmptyStatement)
        }
    },





      // Defines a new variable by assigning a name and a value to it. The name is retrieved from the first element of the children argument, and the value is retrieved by running the run method on the second element of the children argument. The key-value pair is then inserted into the last frame on the stack field of the current runtime object.
      Node::VariableDefine{children} => {
        if children.len() != 2 {
            return Err(AsaErrorKind::InvalidVariable);
        }
    
        let identifier_node = &children[0];
        let identifier_name = match identifier_node {
            Node::Identifier{value} => {
                match String::from_utf8(value.clone()) {
                    Ok(name) => name,
                    Err(_) => return Err(AsaErrorKind::InvalidIdentifier(String::from_utf8_lossy(value).to_string())),
                }
            },
            _ => return Err(AsaErrorKind::InvalidVariable),
        };
    
        let value_node = &children[1];
        let value = self.exec(value_node)?;
    
        if let Some(current_frame) = self.stack.last_mut() {
            current_frame.insert(identifier_name, value.clone());
            Ok(value)  // Return the value after defining it
        } else {
            Err(AsaErrorKind::NoStackFrame)
        }
    },

      // Conditional Expression Node
      Node::ConditionalExpression { operator, children } => {
        if children.len() != 2 {
            return Err(AsaErrorKind::InvalidExpression);
        }

        let left = self.exec(&children[0])?;
        let right = self.exec(&children[1])?;

        let op = match String::from_utf8(operator.clone()) {
            Ok(s) => s,
            Err(_) => return Err(AsaErrorKind::InvalidExpression),
        };

        match op.as_str() {
            ">" => match (left, right) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Bool(l > r)),
                _ => Err(AsaErrorKind::TypeError),
            },
            "<" => match (left, right) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Bool(l < r)),
                _ => Err(AsaErrorKind::TypeError),
            },
            ">=" => match (left, right) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Bool(l >= r)),
                _ => Err(AsaErrorKind::TypeError),
            },
            "<=" => match (left, right) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Bool(l <= r)),
                _ => Err(AsaErrorKind::TypeError),
            },
            "==" => match (left, right) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Bool(l == r)),
                (Value::Bool(l), Value::Bool(r)) => Ok(Value::Bool(l == r)),
                (Value::String(l), Value::String(r)) => Ok(Value::Bool(l == r)),
                _ => Err(AsaErrorKind::TypeError),
            },
            "!=" => match (left, right) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Bool(l != r)),
                (Value::Bool(l), Value::Bool(r)) => Ok(Value::Bool(l != r)),
                (Value::String(l), Value::String(r)) => Ok(Value::Bool(l != r)),
                _ => Err(AsaErrorKind::TypeError),
            },
            _ => Err(AsaErrorKind::InvalidExpression),
        }
      },





      // evaluating the child node using the exec() method.
      Node::Expression{children} => {
        self.exec(&children[0])
      },
      Node::Number{value} => {
        Ok(Value::Number(*value))
      },
      Node::String{value} => {
        Ok(Value::String(value.clone()))
      },
      Node::Bool{value} => {
        Ok(Value::Bool(*value))
      },



      // Return an error message.
      x => {
        Err(AsaErrorKind::UnimplementedNode(format!("Unsupported or unimplemented node: {:?}", x)))
      },
    }
  }

}