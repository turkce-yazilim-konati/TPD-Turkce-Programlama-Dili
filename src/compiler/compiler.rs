use std::vec::Vec;
use std::rc::Rc;

use crate::types::*;
use crate::compiler::*;
use crate::core::*;
use crate::compiler::value::BramaPrimative;
use crate::compiler::ast::BramaAstType;
use crate::compiler::storage_builder::StorageBuilder;

use std::marker::PhantomData;

pub struct BramaCompilerOption<S>
where S: Storage {
    pub opcodes : Vec<u8>,
    pub storages: Vec<StaticStorage>,
    pub modules: ModuleCollection,
    pub _marker: PhantomData<S>
}

impl<S>  BramaCompilerOption<S>
where S: Storage
{
    pub fn new() -> BramaCompilerOption<S> {
        BramaCompilerOption {
            opcodes: Vec::new(),
            storages: vec![StaticStorage::new()],
            modules: ModuleCollection::new(),
            _marker: PhantomData
        }
    }
}


struct CompileInfo {
    /*index: Option<u8>*/
}

pub trait Compiler<S> where S: Storage
{
    fn compile(&self, ast: &BramaAstType, options: &mut BramaCompilerOption<S>) -> CompilerResult;
}


pub struct InterpreterCompiler;
impl<S> Compiler<S> for InterpreterCompiler where S: Storage {   
    fn compile(&self, ast: &BramaAstType, options: &mut BramaCompilerOption<S>) -> CompilerResult {
        let storage_builder: StorageBuilder<S> = StorageBuilder::new();
        storage_builder.prepare_variable_store(ast, options);
        
        let mut main_compile_info = CompileInfo { };

        self.generate_opcode(ast, &BramaAstType::None, &mut main_compile_info, options, 0)?;
        Ok(0)
    }
}

impl InterpreterCompiler {
    fn generate_opcode<S>(&self, ast: &BramaAstType, upper_ast: &BramaAstType, compiler_info: &mut CompileInfo, options: &mut BramaCompilerOption<S>, storage_index: usize) -> CompilerResult where S: Storage {
        match ast {
            BramaAstType::Assignment { variable, operator, expression } => self.generate_assignment(variable.clone(), operator, expression, compiler_info, options, storage_index),
            BramaAstType::Symbol(variable)                              => self.generate_symbol(variable, upper_ast, compiler_info, options, storage_index),
            BramaAstType::Control { left, operator, right }             => self.generate_control(left, operator, right, upper_ast, compiler_info, options, storage_index),
            BramaAstType::Binary { left, operator, right }              => self.generate_binary(left, operator, right, upper_ast, compiler_info, options, storage_index),
            BramaAstType::Block(asts)                                   => self.generate_block(asts, upper_ast, compiler_info, options, storage_index),
            BramaAstType::Primative(primative)                          => self.generate_primative(primative.clone(), compiler_info, options, storage_index),
            BramaAstType::FuncCall { names, arguments }                 => self.generate_func_call(names, arguments, upper_ast, compiler_info, options, storage_index),
            BramaAstType::PrefixUnary (operator, expression)            => self.generate_prefix_unary(operator, expression, upper_ast, compiler_info, options, storage_index),
            BramaAstType::SuffixUnary (operator, expression)            => self.generate_suffix_unary(operator, expression, upper_ast, compiler_info, options, storage_index),
            BramaAstType::None => {
                println!("{:?}", ast);
                Err("Not implemented")
            }
        }
    }

    fn generate_primative<S>(&self, primative: Rc<BramaPrimative>, _: &mut CompileInfo, options: &mut BramaCompilerOption<S>, storage_index: usize) -> CompilerResult where S: Storage {
        let storage = &options.storages[storage_index];

        let result = storage.get_constant_location(primative);
        match result {
            Some(index) => {
                options.opcodes.push(VmOpCode::Load as u8);
                options.opcodes.push(index as u8);
                Ok(index)
            },
            _ => Err("Value not found in storage")
        }
    }

    fn generate_func_call<S>(&self, names: &Vec<String>, arguments: &Vec<Box<BramaAstType>>,  upper_ast: &BramaAstType, compiler_info: &mut CompileInfo, options: &mut BramaCompilerOption<S>, storage_index: usize) -> CompilerResult where S: Storage {
        /* Build arguments */
        for argument in arguments {
            self.generate_opcode(argument, upper_ast, compiler_info, options, storage_index)?;
        }

        let func = options.modules.find_method(names);
        return match func {
            Some(function) => {
                if let Some(location) = options.storages[storage_index].get_constant_location(Rc::new(BramaPrimative::FuncNativeCall(function))) {
                    options.opcodes.push(VmOpCode::NativeCall as u8);
                    options.opcodes.push(location as u8);
                    options.opcodes.push((arguments.len() as u8) as u8);
                    Ok(0 as u8)
                } else {
                    Err("Function not found")
                }
            },
            None => Err("Function not found")
        };
    }

    fn generate_symbol<S>(&self, variable: &String, _: &BramaAstType, _: &mut CompileInfo, options: &mut BramaCompilerOption<S>, storage_index: usize) -> CompilerResult where S: Storage {
        match options.storages.get_mut(storage_index).unwrap().get_variable_location(variable) {
            Some(location) => {
                options.opcodes.push(VmOpCode::Load as u8);
                options.opcodes.push(location as u8);
                Ok(location)
            },
            _ => return Err("Variable not found in storage")
        }
    }

    fn generate_control<S>(&self, left_ast: &BramaAstType, operator: &BramaOperatorType, right_ast: &BramaAstType, _: &BramaAstType, compiler_info: &mut CompileInfo, options: &mut BramaCompilerOption<S>, storage_index: usize) -> CompilerResult where S: Storage {
        self.generate_opcode(left_ast, &BramaAstType::None, compiler_info, options, storage_index)?;
        self.generate_opcode(right_ast, &BramaAstType::None, compiler_info, options, storage_index)?;

        let opcode = match operator {
            BramaOperatorType::Or               => VmOpCode::Or as u8,
            BramaOperatorType::And              => VmOpCode::And as u8,
            BramaOperatorType::Equal            => VmOpCode::Equal as u8,
            BramaOperatorType::NotEqual         => VmOpCode::NotEqual as u8,
            BramaOperatorType::GreaterThan      => VmOpCode::GreaterThan as u8,
            BramaOperatorType::LessThan         => VmOpCode::LessThan as u8,
            BramaOperatorType::GreaterEqualThan => VmOpCode::GreaterEqualThan as u8,
            BramaOperatorType::LessEqualThan    => VmOpCode::LessEqualThan as u8,
            _ => VmOpCode::None as u8
        };

        options.opcodes.push(opcode);
        Ok(0)
    }

    fn generate_assignment<S>(&self, variable: Rc<String>, operator: &BramaOperatorType, expression_ast: &BramaAstType, compiler_info: &mut CompileInfo, options: &mut BramaCompilerOption<S>, storage_index: usize) -> CompilerResult where S: Storage {
        options.storages.get_mut(storage_index).unwrap().add_variable(&*variable);

        let opcode = match operator {
            BramaOperatorType::Assign               => VmOpCode::Store as u8,
            BramaOperatorType::AssignAddition       => VmOpCode::AssignAddition as u8,
            BramaOperatorType::AssignDivision       => VmOpCode::AssignDivision as u8,
            BramaOperatorType::AssignMultiplication => VmOpCode::AssignMultiplication as u8,
            BramaOperatorType::AssignSubtraction    => VmOpCode::AssignSubtraction as u8,
            _ => VmOpCode::None as u8
        };
        
        options.opcodes.push(opcode);
        Ok(0)
    }

    fn generate_binary<S>(&self, left_ast: &BramaAstType, operator: &BramaOperatorType, right_ast: &BramaAstType, _: &BramaAstType, compiler_info: &mut CompileInfo, options: &mut BramaCompilerOption<S>, storage_index: usize) -> CompilerResult where S: Storage { let left = self.generate_opcode(left_ast, &BramaAstType::None, compiler_info, options, storage_index)?;
        self.generate_opcode(right_ast, &BramaAstType::None, compiler_info, options, storage_index)?;
        let opcode = match operator {
            BramaOperatorType::Addition       => VmOpCode::Addition as u8,
            BramaOperatorType::Subtraction    => VmOpCode::Subraction as u8,
            BramaOperatorType::Multiplication => VmOpCode::Multiply as u8,
            BramaOperatorType::Division       => VmOpCode::Division as u8,
            _ => VmOpCode::None as u8
        };

        options.opcodes.push(opcode);
        Ok(0)
    }

    fn generate_prefix_unary<S>(&self, operator: &BramaOperatorType, expression: &BramaAstType, _: &BramaAstType, compiler_info: &mut CompileInfo, options: &mut BramaCompilerOption<S>, storage_index: usize) -> CompilerResult where S: Storage { 
        self.generate_opcode(expression, &BramaAstType::None, compiler_info, options, storage_index)?;
        let opcode = match operator {
            BramaOperatorType::Increment  => VmOpCode::Increment as u8,
            BramaOperatorType::Deccrement => VmOpCode::Decrement as u8,
            BramaOperatorType::Not        => VmOpCode::Not as u8,
            _ => return Err("Unary operator not found")
        };

        options.opcodes.push(opcode);
        Ok(0)
    }

    fn generate_suffix_unary<S>(&self, operator: &BramaOperatorType, expression: &BramaAstType, _: &BramaAstType, compiler_info: &mut CompileInfo, options: &mut BramaCompilerOption<S>, storage_index: usize) -> CompilerResult where S: Storage { 
        self.generate_opcode(expression, &BramaAstType::None, compiler_info, options, storage_index)?;

        let opcode = match operator {
            BramaOperatorType::Increment  => VmOpCode::Increment as u8,
            BramaOperatorType::Deccrement => VmOpCode::Decrement as u8,
            _ => return Err("Unary operator not found")
        };

        options.opcodes.push(opcode);
        Ok(0)
    }

    fn generate_block<S>(&self, asts: &Vec<BramaAstType>, upper_ast: &BramaAstType, compiler_info: &mut CompileInfo, options: &mut BramaCompilerOption<S>, storage_index: usize) -> CompilerResult where S: Storage {
        for ast in asts {
            self.generate_opcode(&ast, upper_ast, compiler_info, options, storage_index)?;
        }
        Ok(0)
    }
}