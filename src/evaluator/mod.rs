use std::{cell::RefCell, process, rc::Rc};

use crate::{
    ast::{
        node::{
            expressions::{
                array_expr::ArrayExpr, bool_expr::BoolExpr, call_expr::CallExpr,
                float_expr::FloatExpr, fn_expr::FnExpr, identifier_expr::IdentifierExpr,
                if_expr::IfExpr, infix_expr::InfixExpr, int_expr::IntExpr, prefix_expr::PrefixExpr,
                string_expr::StringExpr,
            },
            statements::{
                block_stmt::BlockStatement, expression_stmt::ExpressionStmt,
                let_stmt::LetStatement, ret_stmt::RetStatement, var_stmt::VarStatement,
            },
            NodeRef, OpCode,
        },
        Program,
    },
    evaluator::objects::Type,
};

use self::{
    environment::{Environment, EnvironmentRef},
    objects::{
        array::Array, boolean::Boolean, eval_error::EvalError, float::Float, function::Function,
        integer::Integer, ret::Ret, string::Str, Object, ObjectRef,
    },
};

pub mod environment;
pub mod objects;

#[derive(Default)]
pub struct Evaluator;

impl Evaluator {
    pub fn eval(&self, node: Option<&NodeRef>, environment: EnvironmentRef) -> Option<ObjectRef> {
        if let Some(node) = node {
            match node.get_op_code() {
                OpCode::While => todo!(),
                OpCode::Call => {
                    let call_expr = node.as_any().downcast_ref::<CallExpr>().unwrap();
                    let function = self.eval(Some(&call_expr.function), Rc::clone(&environment));
                    if self.is_error(&function) {
                        return function;
                    }
                    let mut args = Vec::with_capacity(3);
                    for arg in call_expr.arguments.iter() {
                        args.push(self.eval(Some(arg), Rc::clone(&environment)));
                    }
                    match args.first() {
                        Some(first) if self.is_error(first) => {
                            return Some(Rc::clone(first.as_ref().unwrap()))
                        }
                        _ => {}
                    }
                    self.apply_function(function.unwrap(), args, environment)
                }
                OpCode::Ret => self.eval_return_smtmt(node, environment),
                OpCode::Index => todo!(),
                OpCode::If => self.eval_if(node, environment),
                OpCode::Array => Some(self.eval_array(node, environment)),
                OpCode::Let => {
                    let let_stmt = node.as_any().downcast_ref::<LetStatement>().unwrap();
                    let name = let_stmt
                        .name
                        .as_any()
                        .downcast_ref::<IdentifierExpr>()
                        .unwrap();
                    let value = Rc::clone(
                        &self
                            .eval(Some(&let_stmt.value), Rc::clone(&environment))
                            .unwrap(),
                    );
                    if self.is_error(&Some(Rc::clone(&value))) {
                        Some(value)
                    } else {
                        self.set_immutable(name.value.clone(), Rc::clone(&value), environment);
                        Some(value)
                    }
                }
                OpCode::Var => {
                    let let_stmt = node.as_any().downcast_ref::<VarStatement>().unwrap();
                    let name = let_stmt
                        .name
                        .as_any()
                        .downcast_ref::<IdentifierExpr>()
                        .unwrap();
                    let value = Rc::clone(
                        &self
                            .eval(Some(&let_stmt.value), Rc::clone(&environment))
                            .unwrap(),
                    );
                    if self.is_error(&Some(Rc::clone(&value))) {
                        Some(value)
                    } else {
                        if value.get_type() == Type::Function {
                            return Some(Rc::new(EvalError::new(
                                "function only be set with let".to_string(),
                            )));
                        }
                        self.set_mutable(name.value.clone(), Rc::clone(&value), environment);
                        Some(value)
                    }
                }
                OpCode::Int => {
                    let int_expr = node.as_any().downcast_ref::<IntExpr>().unwrap();
                    Some(Rc::new(Integer::new(int_expr.value)))
                }
                OpCode::Bool => {
                    let int_expr = node.as_any().downcast_ref::<BoolExpr>().unwrap();
                    Some(Rc::new(Boolean::new(int_expr.value)))
                }

                OpCode::Block => {
                    let block_stmt = node.as_any().downcast_ref::<BlockStatement>().unwrap();
                    let sub_environment = Rc::new(RefCell::new(Environment::new(Some(Rc::clone(
                        &environment,
                    )))));
                    Some(self.eval_statements(&block_stmt.statements, sub_environment))
                }
                OpCode::Infix => Some(self.eval_infix(node, environment)),
                OpCode::Float => {
                    let int_expr = node.as_any().downcast_ref::<FloatExpr>().unwrap();
                    Some(Rc::new(Float::new(int_expr.value)))
                }
                OpCode::Prefix => Some(self.eval_prefix(node, environment)),
                OpCode::String => {
                    let int_expr = node.as_any().downcast_ref::<StringExpr>().unwrap();
                    Some(Rc::new(Str::new(int_expr.value.clone())))
                }
                OpCode::Program => {
                    let program = node.as_any().downcast_ref::<Program>().unwrap();
                    Some(self.eval_statements(&program.statements, environment))
                }
                OpCode::Function => {
                    let function_expr = node.as_any().downcast_ref::<FnExpr>().unwrap();
                    let body = Rc::clone(&function_expr.body);
                    let parameters = Rc::clone(&function_expr.parameters);
                    let function: ObjectRef = Rc::new(Function::new(body, parameters));
                    if let Some(name) = &function_expr.name {
                        let name_string = name
                            .as_any()
                            .downcast_ref::<IdentifierExpr>()
                            .unwrap()
                            .value
                            .clone();
                        self.set_function(name_string, Rc::clone(&function), environment);
                    }
                    Some(function)
                }
                OpCode::Expression => {
                    let expr = node.as_any().downcast_ref::<ExpressionStmt>().unwrap();
                    self.eval(Some(&expr.expression), environment)
                }
                OpCode::Identifier => {
                    let identifier = node.as_any().downcast_ref::<IdentifierExpr>().unwrap();
                    match environment.borrow().get_immutabble(&identifier.value) {
                        Some(value) => Some(Rc::clone(&value)),
                        None => match environment.borrow().get_mutabble(&identifier.value) {
                            Some(value) => Some(Rc::clone(&value)),
                            None => match environment.borrow().get_function(&identifier.value) {
                                Some(value) => Some(Rc::clone(&value)),
                                None => Some(Rc::new(EvalError::new(format!(
                                    "unknown word '{}'",
                                    identifier.value
                                )))),
                            },
                        },
                    }
                }
            }
        } else {
            None
        }
    }

    fn set_immutable(
        &self,
        name: String,
        value: ObjectRef,
        enviroment: EnvironmentRef,
    ) -> Option<ObjectRef> {
        enviroment.borrow_mut().set_immutable(name, value)
    }

    fn set_mutable(
        &self,
        name: String,
        value: ObjectRef,
        enviroment: EnvironmentRef,
    ) -> Option<ObjectRef> {
        enviroment.borrow_mut().set_mutable(name, value)
    }

    fn set_function(
        &self,
        name: String,
        value: ObjectRef,
        enviroment: EnvironmentRef,
    ) -> Option<ObjectRef> {
        enviroment.borrow_mut().set_function(name, value)
    }

    fn eval_statements(&self, stmts: &[NodeRef], enviroment: EnvironmentRef) -> ObjectRef {
        let mut result = None;
        for stmt in stmts.iter() {
            result = self.eval(Some(stmt), Rc::clone(&enviroment));
        }
        if let Some(result) = result {
            result
        } else {
            process::exit(1)
        }
    }

    fn eval_prefix(&self, node: &NodeRef, enviroment: EnvironmentRef) -> ObjectRef {
        let prefix = node.as_any().downcast_ref::<PrefixExpr>().unwrap();
        let value = self.eval(Some(&prefix.right), enviroment);
        match (&prefix.operator[..], value) {
            ("!", Some(value)) if value.get_type() == Type::Bool => {
                let boolean_value = value.as_any().downcast_ref::<Boolean>().unwrap();
                Rc::new(Boolean::new(!boolean_value.value))
            }
            ("!", Some(value)) if value.get_type() == Type::Int => {
                let integer_value = value.as_any().downcast_ref::<Integer>().unwrap();
                Rc::new(Integer::new(!integer_value.value))
            }
            ("-", Some(value)) if value.get_type() == Type::Int => {
                let integer_value = value.as_any().downcast_ref::<Integer>().unwrap();
                Rc::new(Integer::new(-integer_value.value))
            }
            ("-", Some(value)) if value.get_type() == Type::Float => {
                let float_value = value.as_any().downcast_ref::<Float>().unwrap();
                Rc::new(Float::new(-float_value.value))
            }
            (op, Some(value)) => Rc::new(EvalError::new(format!(
                "unsuported operation '{}' with '{}'",
                op,
                value.get_type()
            ))),
            _ => Rc::new(EvalError::new("unexpected error".to_string())),
        }
    }

    fn eval_infix(&self, node: &NodeRef, enviroment: EnvironmentRef) -> ObjectRef {
        let infix_expr = node.as_any().downcast_ref::<InfixExpr>().unwrap();
        let left = self.eval(Some(&infix_expr.left), Rc::clone(&enviroment));
        let right = self.eval(Some(&infix_expr.right), enviroment);
        match (left, right) {
            (Some(left), Some(right)) => match (left.get_type(), right.get_type()) {
                (Type::Int, Type::Int) => {
                    let left = left.as_any().downcast_ref::<Integer>().unwrap();
                    let right = right.as_any().downcast_ref::<Integer>().unwrap();
                    match &infix_expr.operator[..] {
                        "+" => Rc::new(Integer::new(left.value + right.value)),
                        "-" => Rc::new(Integer::new(left.value - right.value)),
                        "*" => Rc::new(Integer::new(left.value * right.value)),
                        "/" => Rc::new(Integer::new(left.value / right.value)),
                        "%" => Rc::new(Integer::new(left.value % right.value)),
                        "<<" => Rc::new(Integer::new(left.value << right.value)),
                        ">>" => Rc::new(Integer::new(left.value >> right.value)),
                        "&" => Rc::new(Integer::new(left.value & right.value)),
                        "|" => Rc::new(Integer::new(left.value | right.value)),
                        "^" => Rc::new(Integer::new(left.value ^ right.value)),
                        ">" => Rc::new(Boolean::new(left.value > right.value)),
                        "<" => Rc::new(Boolean::new(left.value < right.value)),
                        ">=" => Rc::new(Boolean::new(left.value >= right.value)),
                        "<=" => Rc::new(Boolean::new(left.value <= right.value)),
                        "==" => Rc::new(Boolean::new(left.value == right.value)),
                        _ => Rc::new(EvalError::new(format!(
                            "unsoported operation {} {} {}",
                            left.get_type(),
                            infix_expr.operator,
                            right.get_type()
                        ))),
                    }
                }
                (Type::Float, Type::Float) => {
                    let left = left.as_any().downcast_ref::<Float>().unwrap();
                    let right = right.as_any().downcast_ref::<Float>().unwrap();
                    match &infix_expr.operator[..] {
                        "+" => Rc::new(Float::new(left.value + right.value)),
                        "-" => Rc::new(Float::new(left.value - right.value)),
                        "*" => Rc::new(Float::new(left.value * right.value)),
                        "/" => Rc::new(Float::new(left.value / right.value)),
                        ">" => Rc::new(Boolean::new(left.value > right.value)),
                        "<" => Rc::new(Boolean::new(left.value < right.value)),
                        ">=" => Rc::new(Boolean::new(left.value >= right.value)),
                        "<=" => Rc::new(Boolean::new(left.value <= right.value)),
                        "==" => Rc::new(Boolean::new(left.value == right.value)),
                        _ => Rc::new(EvalError::new(format!(
                            "unsoported operation {} {} {}",
                            left.get_type(),
                            infix_expr.operator,
                            right.get_type()
                        ))),
                    }
                }
                (Type::Bool, Type::Bool) => {
                    let left = left.as_any().downcast_ref::<Boolean>().unwrap();
                    let right = right.as_any().downcast_ref::<Boolean>().unwrap();
                    match &infix_expr.operator[..] {
                        "&&" => Rc::new(Boolean::new(left.value && right.value)),
                        "||" => Rc::new(Boolean::new(left.value || right.value)),
                        _ => Rc::new(EvalError::new(format!(
                            "unsoported operation {} {} {}",
                            left.get_type(),
                            infix_expr.operator,
                            right.get_type()
                        ))),
                    }
                }
                (left, right) => Rc::new(EvalError::new(format!(
                    "unsoported operation {} {} {}",
                    left, infix_expr.operator, right
                ))),
            },
            _ => todo!(),
        }
    }

    fn eval_array(&self, node: &NodeRef, enviroment: EnvironmentRef) -> ObjectRef {
        let array_expr = node.as_any().downcast_ref::<ArrayExpr>().unwrap();
        let mut values: Vec<ObjectRef> = Vec::with_capacity(10);
        for expr in array_expr.values.iter() {
            let evaluated = self.eval(Some(expr), Rc::clone(&enviroment));
            if self.is_error(&evaluated) {
                return evaluated.unwrap();
            }
            values.push(evaluated.unwrap());
        }
        Rc::new(Array::new(values))
    }

    fn eval_if(&self, node: &NodeRef, environment: EnvironmentRef) -> Option<ObjectRef> {
        let if_expr = node.as_any().downcast_ref::<IfExpr>().unwrap();
        let condition = self.eval(Some(&if_expr.condition), Rc::clone(&environment));
        if self.is_error(&condition) {
            return condition;
        }
        match condition
            .as_ref()
            .unwrap()
            .as_any()
            .downcast_ref::<Boolean>()
        {
            Some(condition) => {
                if condition.value {
                    self.eval(Some(&if_expr.consequence), environment)
                } else if let Some(ref el_if) = if_expr.el_if {
                    self.eval(Some(el_if), environment)
                } else if let Some(ref alternative) = if_expr.alternative {
                    self.eval(Some(alternative), environment)
                } else {
                    None
                }
            }
            None => None,
        }
    }

    fn apply_function(
        &self,
        function: ObjectRef,
        arguments: Vec<Option<ObjectRef>>,
        environment: EnvironmentRef,
    ) -> Option<ObjectRef> {
        if let Some(function) = function.as_any().downcast_ref::<Function>() {
            let new_env = self.create_function_environment(function, arguments, environment);
            let body = self.eval(Some(&function.body), new_env);
            if self.is_error(&body) {
                return body;
            }
            self.extract_ret_val(body)
        } else {
            None
        }
    }

    fn extract_ret_val(&self, evaluated: Option<ObjectRef>) -> Option<ObjectRef> {
        if let Some(ret) = evaluated.as_ref().unwrap().as_any().downcast_ref::<Ret>() {
            Some(Rc::clone(&ret.val))
        } else {
            evaluated
        }
    }

    fn create_function_environment(
        &self,
        function: &Function,
        arguments: Vec<Option<ObjectRef>>,
        environment: EnvironmentRef,
    ) -> EnvironmentRef {
        let mut env = Environment::new(Some(Rc::clone(&environment)));
        for (idx, arg) in arguments.iter().enumerate() {
            let argument_name = function
                .parameters
                .get(idx)
                .as_ref()
                .unwrap()
                .as_any()
                .downcast_ref::<IdentifierExpr>()
                .unwrap()
                .value
                .clone();
            env.set_immutable(argument_name, Rc::clone(arg.as_ref().unwrap()));
        }
        Rc::new(RefCell::new(env))
    }

    fn eval_return_smtmt(&self, node: &NodeRef, environment: EnvironmentRef) -> Option<ObjectRef> {
        let ret_stmt = node.as_any().downcast_ref::<RetStatement>().unwrap();
        match &ret_stmt.value {
            Some(expr) => {
                let val = self.eval(Some(expr), environment);
                match val {
                    _ if self.is_error(&val) => val,
                    Some(val) => Some(Rc::new(Ret::new(val))),
                    None => None,
                }
            }
            None => None,
        }
    }

    fn is_error(&self, to_check: &Option<ObjectRef>) -> bool {
        match to_check {
            Some(check) if check.get_type() == Type::Error => {
                let eval_error = check.as_any().downcast_ref::<EvalError>().unwrap();
                eprintln!("{}", eval_error.message);
                true
            }
            _ => false,
        }
    }
}
