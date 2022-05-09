use std::{cell::RefCell, cmp::Ordering, collections::HashMap, process, rc::Rc};

use crate::{
    ast::{
        node::{
            expressions::{
                array_expr::ArrayExpr, bool_expr::BoolExpr, call_expr::CallExpr,
                float_expr::FloatExpr, fn_expr::FnExpr, hash_expr::HashExpr, if_expr::IfExpr,
                index_expr::IndexExpr, infix_expr::InfixExpr, int_expr::IntExpr,
                prefix_expr::PrefixExpr, string_expr::StringExpr, while_expr::WhileExpr,
            },
            statements::{
                block_stmt::BlockStatement, expression_stmt::ExpressionStmt,
                let_stmt::LetStatement, ret_stmt::RetStatement, var_stmt::VarStatement,
            },
            NodeRef, OpCode,
        },
        Program,
    },
    builtin_map, downcast, downcast_any, downcast_option,
    evaluator::objects::Type,
};

use self::{
    environment::{Environment, EnvironmentRef},
    objects::{
        array::Array, boolean::Boolean, built_in::BuiltIn, eval_error::EvalError, float::Float,
        function::Function, hash::HashObj, integer::Integer, ret::Ret, string::Str, Object,
        ObjectRef,
    },
};

pub mod built_in;
pub mod environment;
pub mod objects;

pub type BuiltInMap = HashMap<String, ObjectRef>;

pub struct Evaluator {
    built_in: BuiltInMap,
}

impl Evaluator {
    pub fn new() -> Self {
        let built_in: BuiltInMap = builtin_map!["exit" => Rc::new(BuiltIn::new(built_in::exit)),
            "len" => Rc::new(BuiltIn::new(built_in::len)),
            "puts" => Rc::new(BuiltIn::new(built_in::puts)),
            "eputs" => Rc::new(BuiltIn::new(built_in::eputs)),
            "read" => Rc::new(BuiltIn::new(built_in::read)),
            "to_int" => Rc::new(BuiltIn::new(built_in::to_int)),
            "to_str" => Rc::new(BuiltIn::new(built_in::to_string)),
            "to_float" => Rc::new(BuiltIn::new(built_in::to_float)),
            "putsln" => Rc::new(BuiltIn::new(built_in::putsln)),
            "eputsln" => Rc::new(BuiltIn::new(built_in::eputsln)),
            "read_file_as_string" => Rc::new(BuiltIn::new(built_in::read_file_as_string))
        ];

        Self { built_in }
    }
}

impl Evaluator {
    pub fn eval(&self, node: Option<&NodeRef>, environment: EnvironmentRef) -> Option<ObjectRef> {
        if let Some(node) = node {
            match node.get_op_code() {
                OpCode::Hash => self.eval_hash(node, environment),
                OpCode::While => self.eval_while(node, environment),
                OpCode::Ret => self.eval_return_smtmt(node, environment),
                OpCode::Index => self.eval_index(node, environment),
                OpCode::If => self.eval_if(node, environment),
                OpCode::Array => Some(self.eval_array(node, environment)),
                OpCode::Let => {
                    let let_stmt = downcast_any!(node => LetStatement);
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
                    let let_stmt = downcast_any!(node => VarStatement);
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
                    let int_expr = downcast_any!(node => IntExpr);
                    Some(Rc::new(Integer::new(int_expr.value)))
                }
                OpCode::Bool => {
                    let int_expr = downcast_any!(node => BoolExpr);
                    Some(Rc::new(Boolean::new(int_expr.value)))
                }

                OpCode::Block => {
                    let block_stmt = downcast_any!(node => BlockStatement);
                    let sub_environment = Rc::new(RefCell::new(Environment::new(Some(Rc::clone(
                        &environment,
                    )))));
                    Some(self.eval_statements(&block_stmt.statements, sub_environment))
                }
                OpCode::Infix => Some(self.eval_infix(node, environment)),
                OpCode::Float => {
                    let int_expr = downcast_any!(node => FloatExpr);
                    Some(Rc::new(Float::new(int_expr.value)))
                }
                OpCode::Prefix => Some(self.eval_prefix(node, environment)),
                OpCode::String => {
                    let int_expr = downcast_any!(node => StringExpr);
                    Some(Rc::new(Str::new(int_expr.value.clone())))
                }
                OpCode::Program => {
                    let program = downcast_any!(node => Program);
                    Some(self.eval_statements(&program.statements, environment))
                }
                OpCode::Function => {
                    let function_expr = downcast_any!(node => FnExpr);
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
                    let expr = downcast_any!(node => ExpressionStmt);
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
                    let call_expr = downcast_any!(node => CallExpr);
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
        let prefix = downcast_any!(node => PrefixExpr);
        let value = self.eval(Some(&prefix.right), enviroment);
        match (&prefix.operator[..], value) {
            ("!", Some(value)) if value.get_type() == Type::Bool => {
                let boolean_value = downcast_any!(value => Boolean);
                Rc::new(Boolean::new(!boolean_value.value))
            }
            ("!", Some(value)) if value.get_type() == Type::Int => {
                let integer_value = downcast_any!(value => Integer);
                Rc::new(Integer::new(!integer_value.value))
            }
            ("-", Some(value)) if value.get_type() == Type::Int => {
                let integer_value = downcast_any!(value => Integer);
                Rc::new(Integer::new(-integer_value.value))
            }
            ("-", Some(value)) if value.get_type() == Type::Float => {
                let float_value = downcast_any!(value => Float);
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
        let call_expr = downcast_any!(call => CallExpr);
        let function_name = call_expr.function.to_string();
        match object.get_type() {
            Type::Array => {
                let arr = downcast_any!(object => Array);
                if let Some(function) = arr.functions.get(&function_name) {
                    let args = self.create_object_call_env(
                        Rc::clone(&object),
                        &call_expr.arguments,
                        environment,
                    );
                    let function = downcast_any!(function => BuiltIn);
                    (function.function)(&args)
                } else {
                    Rc::new(EvalError::new(format!(
                        "unknown function {}",
                        function_name
                    )))
                }
            }
            Type::String => {
                let string = downcast_any!(object => Str);
                if let Some(function) = string.functions.get(&function_name) {
                    let args = self.create_object_call_env(
                        Rc::clone(&object),
                        &call_expr.arguments,
                        environment,
                    );
                    let function = downcast_any!(function => BuiltIn);
                    (function.function)(&args)
                } else {
                    Rc::new(EvalError::new(format!(
                        "unknown function {}",
                        function_name
                    )))
                }
            }
            typ => Rc::new(EvalError::new(format!(
                "{} does not support functions for now",
                typ
            ))),
        }
    }

    fn create_object_call_env(
        &self,
        object: ObjectRef,
        arguments: &[NodeRef],
        environment: EnvironmentRef,
    ) -> Vec<ObjectRef> {
        let mut args: Vec<ObjectRef> = vec![object];
        for arg in arguments.iter() {
            if let Some(arg) = self.eval(Some(arg), Rc::clone(&environment)) {
                args.push(arg)
            } else {
                return vec![Rc::new(EvalError::new(
                    "error on parse arguments".to_string(),
                ))];
            }
        }
        args
    }

    #[inline]
    fn eval_infix(&self, node: &NodeRef, environment: EnvironmentRef) -> ObjectRef {
        let infix_expr = downcast_any!(node => InfixExpr);
        let left = self.eval(Some(&infix_expr.left), Rc::clone(&environment));
        if infix_expr.right.get_op_code() == OpCode::Call
            && ".".cmp(&infix_expr.operator) == Ordering::Equal
        {
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
                    let left = downcast_any!(left => Integer);
                    let right = downcast_any!(right => Integer);
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
                        "!=" => Rc::new(Boolean::new(left.value != right.value)),
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
                    let left = downcast_any!(left => Float);
                    let right = downcast_any!(right => Float);
                    match &infix_expr.operator[..] {
                        "+" => Rc::new(Float::new(left.value + right.value)),
                        "-" => Rc::new(Float::new(left.value - right.value)),
                        "*" => Rc::new(Float::new(left.value * right.value)),
                        "/" => Rc::new(Float::new(left.value / right.value)),
                        ">" => Rc::new(Boolean::new(left.value > right.value)),
                        "<" => Rc::new(Boolean::new(left.value < right.value)),
                        ">=" => Rc::new(Boolean::new(left.value >= right.value)),
                        "<=" => Rc::new(Boolean::new(left.value <= right.value)),
                        "!=" => Rc::new(Boolean::new(left.value != right.value)),
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
                    let left = downcast_any!(left => Boolean);
                    let right = downcast_any!(right => Boolean);
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
                    let left = downcast_any!(left => Str);
                    let right = downcast_any!(right => Str);
                    match &infix_expr.operator[..] {
                        "+" => Rc::new(Str::new(format!("{}{}", left.value, right.value))),
                        "!=" => Rc::new(Boolean::new(
                            left.value.cmp(&right.value) != Ordering::Equal,
                        )),
                        "==" => Rc::new(Boolean::new(
                            left.value.cmp(&right.value) == Ordering::Equal,
                        )),
                        ">" => Rc::new(Boolean::new(
                            left.value.cmp(&right.value) == Ordering::Greater,
                        )),
                        "<" => {
                            Rc::new(Boolean::new(left.value.cmp(&right.value) == Ordering::Less))
                        }
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
        let array_expr = downcast_any!(node => ArrayExpr);
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
        let if_expr = downcast_any!(node => IfExpr);
        let condition = self.eval(Some(&if_expr.condition), Rc::clone(&environment));
        if self.is_error(&condition) {
            return condition;
        }
        match downcast_option!(condition => Boolean) {
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
        let if_expr = downcast_any!(node => WhileExpr);
        let mut result: Option<ObjectRef> = None;
        loop {
            let condition = self.eval(Some(&if_expr.condition), Rc::clone(&environment));
            if self.is_error(&condition) {
                return condition;
            }
            match downcast_option!(condition => Boolean) {
                Some(condition) => {
                    if condition.value {
                        result = self.eval(Some(&if_expr.consequence), Rc::clone(&environment));
                        if let Some(ref result) = result {
                            if result.get_type() == Type::Return {
                                break;
                            } else if result.get_type() == Type::Error {
                                process::exit(1);
                            }
                        } else {
                            break;
                        }
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
        if let Some(function) = downcast!(function => Function) {
            let new_env = self.create_function_environment(function, arguments);
            let body = downcast_any!(function.body => BlockStatement);
            let body = Some(self.eval_statements(&body.statements, new_env));
            if self.is_error(&body) {
                return body;
            }
            self.extract_ret_val(body)
        } else if let Some(fnc) = downcast!(function => BuiltIn) {
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
        if let Some(ret) = downcast_option!(evaluated.as_ref() => Ret) {
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
        let ret_stmt = downcast_any!(node => RetStatement);
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

    #[inline(always)]
    fn extract_array_index(&self, obj: ObjectRef, position: usize) -> Option<ObjectRef> {
        let array = downcast_any!(obj => Array);
        let values = array.values.borrow();
        if position < values.len() {
            Some(Rc::clone(&values[position]))
        } else {
            Some(Rc::new(EvalError::new(format!(
                "invalid index {}",
                position
            ))))
        }
    }

    #[inline(always)]
    fn extract_string_index(&self, obj: ObjectRef, position: usize) -> Option<ObjectRef> {
        let string = downcast_any!(obj => Str);
        let value = &string.value;
        if position < value.len() {
            Some(Rc::new(Str::new(
                value.chars().nth(position).unwrap().to_string(),
            )))
        } else {
            Some(Rc::new(EvalError::new(format!(
                "invalid string position {}",
                position
            ))))
        }
    }

    //TODO: improve this code and check errors
    #[inline]
    fn eval_index(&self, node: &NodeRef, environment: EnvironmentRef) -> Option<ObjectRef> {
        let index_expr = downcast_any!(node => IndexExpr);

        let left = &index_expr.left;
        if let Some(index) = self.eval(Some(&index_expr.index), Rc::clone(&environment)) {
            match index.get_type() {
                Type::Int => {
                    let position = downcast_any!(index => Integer).value as usize;
                    match left.get_op_code() {
                        OpCode::Identifier => {
                            let eval = self.eval(Some(&index_expr.left), environment);
                            if let Some(eval) = eval {
                                if eval.get_type() == Type::Array {
                                    self.extract_array_index(eval, position)
                                } else if eval.get_type() == Type::String {
                                    self.extract_string_index(eval, position)
                                } else {
                                    todo!()
                                }
                            } else {
                                eval
                            }
                        }
                        OpCode::Array => {
                            let eval = self.eval(Some(left), environment);
                            if self.is_error(&eval) {
                                return eval;
                            }
                            self.extract_array_index(eval.unwrap(), position)
                        }
                        OpCode::String => {
                            let string = self.eval(Some(left), environment);
                            if self.is_error(&string) {
                                return string;
                            }
                            self.extract_string_index(string.unwrap(), position)
                        }
                        _ => Some(Rc::new(EvalError::new(format!(
                            "unsuported operation {:?}[{}]",
                            left.get_op_code(),
                            position
                        )))),
                    }
                }
                Type::String => {
                    let key = &downcast_any!(index => Str).value;
                    match left.get_op_code() {
                        OpCode::Hash => {
                            let hash = self.eval(Some(left), environment);
                            if self.is_error(&hash) {
                                return hash;
                            }
                            self.extract_hash_value(&hash.unwrap(), key)
                        }
                        OpCode::Identifier => {
                            let evalueted = self.eval(Some(left), environment);
                            match evalueted {
                                Some(expec) if expec.get_type() == Type::Hash => {
                                    self.extract_hash_value(&expec, key)
                                }
                                _ => Some(Rc::new(EvalError::new(format!(
                                    "unsuported operation {:?}[{}]",
                                    left.get_op_code(),
                                    key
                                )))),
                            }
                        }
                        _ => Some(Rc::new(EvalError::new(format!(
                            "unsuported operation {:?}[{}]",
                            left.get_op_code(),
                            key
                        )))),
                    }
                }
                _ => Some(Rc::new(EvalError::new(
                    "operation not supported".to_string(),
                ))),
            }
        } else {
            Some(Rc::new(EvalError::new(
                "operation not supported".to_string(),
            )))
        }
    }

    fn extract_hash_value(&self, hash_obj: &ObjectRef, key: &str) -> Option<ObjectRef> {
        let hash_obj = downcast_any!(hash_obj => HashObj);
        if let Some(value) = hash_obj.get(key) {
            Some(Rc::clone(value))
        } else {
            Some(Rc::new(EvalError::new(format!("unknown key {}", key))))
        }
    }

    #[inline]
    fn is_error(&self, to_check: &Option<ObjectRef>) -> bool {
        match to_check {
            Some(check) if check.get_type() == Type::Error => {
                true
            }
            None => true,
            _ => false,
        }
    }

    #[inline]
    fn eval_hash(
        &self,
        node: &NodeRef,
        environment: Rc<RefCell<Environment>>,
    ) -> Option<Rc<dyn Object>> {
        let hash_expr = downcast_any!(node => HashExpr);
        let mut hash_obj = HashObj::default();
        for (key, value) in hash_expr.pairs.iter() {
            let value = self.eval(Some(value), Rc::clone(&environment));
            if self.is_error(&value) {
                return value;
            }
            hash_obj.put(key.clone(), Rc::clone(value.as_ref().unwrap()));
        }
        Some(Rc::new(hash_obj))
    }
}

impl Default for Evaluator {
    fn default() -> Self {
        Self::new()
    }
}
