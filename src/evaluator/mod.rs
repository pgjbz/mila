use std::{cell::RefCell, collections::HashMap, process, rc::Rc};

use crate::{
    ast::{
        node::{
            expressions::{
                array_expr::ArrayExpr, bool_expr::BoolExpr, call_expr::CallExpr,
                float_expr::FloatExpr, fn_expr::FnExpr, if_expr::IfExpr, index_expr::IndexExpr,
                infix_expr::InfixExpr, int_expr::IntExpr, prefix_expr::PrefixExpr,
                string_expr::StringExpr, while_expr::WhileExpr,
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
        array::Array, boolean::Boolean, built_in::BuiltIn, eval_error::EvalError, float::Float,
        function::Function, integer::Integer, ret::Ret, string::Str, Object, ObjectRef,
    },
};

pub mod built_in;
pub mod environment;
pub mod objects;

pub struct Evaluator {
    built_in: HashMap<String, ObjectRef>,
}

impl Evaluator {
    pub fn new() -> Self {
        let mut built_in: HashMap<String, ObjectRef> = HashMap::new();
        built_in.insert("exit".to_string(), Rc::new(BuiltIn::new(built_in::exit)));
        built_in.insert("len".to_string(), Rc::new(BuiltIn::new(built_in::len)));
        built_in.insert("puts".to_string(), Rc::new(BuiltIn::new(built_in::puts)));
        built_in.insert("eputs".to_string(), Rc::new(BuiltIn::new(built_in::eputs)));
        built_in.insert("read".to_string(), Rc::new(BuiltIn::new(built_in::read)));
        built_in.insert("trim".to_string(), Rc::new(BuiltIn::new(built_in::trim)));
        built_in.insert(
            "to_int".to_string(),
            Rc::new(BuiltIn::new(built_in::to_int)),
        );
        built_in.insert(
            "to_str".to_string(),
            Rc::new(BuiltIn::new(built_in::to_string)),
        );
        built_in.insert(
            "to_float".to_string(),
            Rc::new(BuiltIn::new(built_in::to_float)),
        );
        built_in.insert(
            "putsln".to_string(),
            Rc::new(BuiltIn::new(built_in::putsln)),
        );
        built_in.insert(
            "eputsln".to_string(),
            Rc::new(BuiltIn::new(built_in::eputsln)),
        );
        Self { built_in }
    }
}

impl Evaluator {
    pub fn eval(&self, node: Option<&NodeRef>, environment: EnvironmentRef) -> Option<ObjectRef> {
        if let Some(node) = node {
            match node.get_op_code() {
                OpCode::While => self.eval_while(node, environment),
                OpCode::Ret => self.eval_return_smtmt(node, environment),
                OpCode::Index => self.eval_index(node, environment),
                OpCode::If => self.eval_if(node, environment),
                OpCode::Array => Some(self.eval_array(node, environment)),
                OpCode::Let => {
                    let let_stmt = node.as_any().downcast_ref::<LetStatement>().unwrap();
                    let value = Rc::clone(
                        &self
                            .eval(Some(&let_stmt.value), Rc::clone(&environment))
                            .unwrap(),
                    );
                    if self.is_error(&Some(Rc::clone(&value))) {
                        Some(value)
                    } else {
                        self.set_variable(
                            let_stmt.name.to_string(),
                            Rc::clone(&value),
                            environment,
                        );
                        Some(value)
                    }
                }
                OpCode::Var => {
                    let let_stmt = node.as_any().downcast_ref::<VarStatement>().unwrap();
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
                        self.set_variable(
                            let_stmt.name.to_string(),
                            Rc::clone(&value),
                            environment,
                        );
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
                    let function: ObjectRef =
                        Rc::new(Function::new(body, parameters, Rc::clone(&environment)));
                    if let Some(name) = &function_expr.name {
                        self.set_variable(name.to_string(), Rc::clone(&function), environment);
                    }
                    Some(function)
                }
                OpCode::Expression => {
                    let expr = node.as_any().downcast_ref::<ExpressionStmt>().unwrap();
                    self.eval(Some(&expr.expression), environment)
                }
                OpCode::Identifier => {
                    let identifier = node.to_string();
                    if let Some(value) = environment.borrow().get_variable(&identifier) {
                        Some(value)
                    } else if let Some(value) = self.built_in.get(&identifier) {
                        Some(Rc::clone(value))
                    } else {
                        Some(Rc::new(EvalError::new(format!(
                            "unknown word '{}'",
                            identifier
                        ))))
                    }
                }
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
                    if args.len() == 1 && self.is_error(args.first().unwrap()) {
                        return function;
                    }
                    self.apply_function(function.unwrap(), args)
                }
            }
        } else {
            None
        }
    }

    #[inline]
    fn set_variable(
        &self,
        name: String,
        value: ObjectRef,
        enviroment: EnvironmentRef,
    ) -> Option<ObjectRef> {
        enviroment.borrow_mut().set_variable(name, value)
    }

    #[inline]
    fn eval_statements(&self, stmts: &[NodeRef], enviroment: EnvironmentRef) -> ObjectRef {
        let mut result = None;
        for stmt in stmts.iter() {
            result = self.eval(Some(stmt), Rc::clone(&enviroment));
            //TODO: improve this

            if let Some(ref result) = result {
                if result.get_type() == Type::Return || self.is_error(&Some(Rc::clone(result))) {
                    break;
                }
            }
        }
        if let Some(result) = result {
            result
        } else {
            process::exit(1)
        }
    }

    #[inline]
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

    fn eval_object_function(
        &self,
        object: ObjectRef,
        call: &NodeRef,
        environment: EnvironmentRef,
    ) -> ObjectRef {
        let call_expr = call.as_any().downcast_ref::<CallExpr>().unwrap();
        let function_name = call_expr.function.to_string();
        match object.get_type() {
            Type::Array => {
                let arr = object.as_any().downcast_ref::<Array>().unwrap();
                if let Some(function) = arr.functions.get(&function_name) {
                    let mut args: Vec<ObjectRef> = Vec::new();
                    for arg in call_expr.arguments.iter() {
                        if let Some(arg) = self.eval(Some(arg), Rc::clone(&environment)) {
                            args.push(arg)
                        } else {
                            return Rc::new(EvalError::new("error on parse arguments".to_string()));
                        }
                    }
                    let function = function.as_any().downcast_ref::<BuiltIn>().unwrap();
                    args.insert(0, Rc::clone(&object));
                    (function.function)(&args);
                    object
                } else {
                    Rc::new(EvalError::new(format!(
                        "unknown function {}",
                        function_name
                    )))
                }
            }
            _ => todo!(),
        }
    }

    #[inline]
    fn eval_infix(&self, node: &NodeRef, environment: EnvironmentRef) -> ObjectRef {
        let infix_expr = node.as_any().downcast_ref::<InfixExpr>().unwrap();
        let left = self.eval(Some(&infix_expr.left), Rc::clone(&environment));
        if infix_expr.right.get_op_code() == OpCode::Call {
            return self.eval_object_function(
                left.unwrap(),
                &infix_expr.right,
                Rc::clone(&environment),
            );
        }
        let right = self.eval(Some(&infix_expr.right), environment);
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
                (Type::String, Type::String) => {
                    let left = left.as_any().downcast_ref::<Str>().unwrap();
                    let right = right.as_any().downcast_ref::<Str>().unwrap();
                    match &infix_expr.operator[..] {
                        "+" => Rc::new(Str::new(format!("{}{}", left.value, right.value))),
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
            _ => Rc::new(EvalError::new("unsoported operation ".to_string())),
        }
    }

    #[inline]
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
        Rc::new(Array::new(RefCell::new(values)))
    }

    #[inline]
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

    #[inline]
    fn eval_while(&self, node: &NodeRef, environment: EnvironmentRef) -> Option<ObjectRef> {
        let if_expr = node.as_any().downcast_ref::<WhileExpr>().unwrap();
        let mut result: Option<ObjectRef> = None;
        loop {
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
                        result = self.eval(Some(&if_expr.consequence), Rc::clone(&environment))
                    } else {
                        break;
                    }
                }
                None => break,
            }
        }
        result
    }

    #[inline]
    fn apply_function(
        &self,
        function: ObjectRef,
        arguments: Vec<Option<ObjectRef>>,
    ) -> Option<ObjectRef> {
        if let Some(function) = function.as_any().downcast_ref::<Function>() {
            let new_env = self.create_function_environment(function, arguments);
            let body = function
                .body
                .as_any()
                .downcast_ref::<BlockStatement>()
                .unwrap();
            let body = Some(self.eval_statements(&body.statements, new_env));
            if self.is_error(&body) {
                return body;
            }
            self.extract_ret_val(body)
        } else if let Some(fnc) = function.as_any().downcast_ref::<BuiltIn>() {
            let mut args: Vec<ObjectRef> = Vec::with_capacity(3);
            for arg in arguments.iter() {
                args.push(Rc::clone(arg.as_ref().unwrap()))
            }
            Some((fnc.function)(&args))
        } else {
            None
        }
    }

    #[inline]
    fn extract_ret_val(&self, evaluated: Option<ObjectRef>) -> Option<ObjectRef> {
        if let Some(ret) = evaluated.as_ref().unwrap().as_any().downcast_ref::<Ret>() {
            Some(Rc::clone(&ret.val))
        } else {
            evaluated
        }
    }

    #[inline]
    fn create_function_environment(
        &self,
        function: &Function,
        arguments: Vec<Option<ObjectRef>>,
    ) -> EnvironmentRef {
        let mut env = Environment::new(Some(Rc::clone(&function.environment)));
        for (idx, arg) in arguments.iter().enumerate() {
            let argument_name = function.parameters.get(idx).as_ref().unwrap().to_string();
            env.set_variable(argument_name, Rc::clone(arg.as_ref().unwrap()));
        }
        Rc::new(RefCell::new(env))
    }

    #[inline]
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

    //TODO: improve this code and check errors
    #[inline]
    fn eval_index(&self, node: &NodeRef, environment: EnvironmentRef) -> Option<ObjectRef> {
        let index_expr = node.as_any().downcast_ref::<IndexExpr>().unwrap();
        let position =
            if let Some(value) = self.eval(Some(&index_expr.index), Rc::clone(&environment)) {
                if value.get_type() != Type::Int {
                    return Some(Rc::new(EvalError::new(format!(
                        "operation not supported [{}]",
                        value.get_type()
                    ))));
                }
                value.as_any().downcast_ref::<Integer>().unwrap().value as usize
            } else {
                return Some(Rc::new(EvalError::new(
                    "operation not supported".to_string(),
                )));
            };
        let left = &index_expr.left;
        if left.get_op_code() == OpCode::Identifier {
            let eval = self.eval(Some(&index_expr.left), environment);
            if let Some(eval) = eval {
                if eval.get_type() == Type::Array {
                    let arr = eval.as_any().downcast_ref::<Array>().unwrap();
                    if position >= arr.values.borrow().len() {
                        Some(Rc::new(EvalError::new(format!(
                            "invalid index {}",
                            position
                        ))))
                    } else {
                        Some(Rc::clone(&arr.values.borrow()[position]))
                    }
                } else if eval.get_type() == Type::String {
                    let string = eval.as_any().downcast_ref::<Str>().unwrap();
                    if position >= string.value.len() {
                        Some(Rc::new(EvalError::new(format!(
                            "invalid index {}",
                            position
                        ))))
                    } else {
                        Some(Rc::new(Str::new(
                            string.value.chars().nth(position).unwrap().to_string(),
                        )))
                    }
                } else {
                    todo!()
                }
            } else {
                eval
            }
        } else if left.get_op_code() == OpCode::Array {
            let arr = left.as_any().downcast_ref::<ArrayExpr>().unwrap();
            if position >= arr.values.len() {
                Some(Rc::new(EvalError::new(format!(
                    "invalid index {}",
                    position
                ))))
            } else {
                self.eval(Some(&arr.values[position]), environment)
            }
        } else if left.get_op_code() == OpCode::String {
            let string = left.as_any().downcast_ref::<StringExpr>().unwrap();
            if position >= string.value.len() {
                Some(Rc::new(EvalError::new(format!(
                    "invalid index {}",
                    position
                ))))
            } else {
                Some(Rc::new(Str::new(
                    string.value.chars().nth(position).unwrap().to_string(),
                )))
            }
        } else {
            Some(Rc::new(EvalError::new(format!(
                "unsuported operation {:?}[{}]",
                left.get_op_code(),
                position
            ))))
        }
    }

    #[inline]
    fn is_error(&self, to_check: &Option<ObjectRef>) -> bool {
        match to_check {
            Some(check) if check.get_type() == Type::Error => {
                let eval_error = check.as_any().downcast_ref::<EvalError>().unwrap();
                eprintln!("{}", eval_error.message);
                true
            }
            None => true,
            _ => false,
        }
    }
}

impl Default for Evaluator {
    fn default() -> Self {
        Self::new()
    }
}
