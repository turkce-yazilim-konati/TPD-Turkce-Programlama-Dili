use crate::types::*;
use crate::compiler::value::*;
use crate::compiler::Storage;

use std::collections::HashMap;
use std::rc::Rc;


#[derive(PartialEq, Debug)]
pub struct DynamicStorage {
    pub constants             : Vec<VmObject>,
    pub constant_size         : u8,
    pub temp_size             : u8,
    pub temp_counter          : u8,
    pub variables             : HashMap<String, u8>,
    pub memory                : Vec<VmObject>,
    pub total_const_variables : u8,
    pub builded               : bool
}

impl Storage for DynamicStorage {
    fn new() -> DynamicStorage {
        DynamicStorage {
            constants: Vec::new(),
            constant_size: 0,
            temp_size: 0,
            temp_counter: 0,
            total_const_variables: 0,
            memory: Vec::new(),
            variables: HashMap::new(),
            builded: false
        }
    }

    fn build(&mut self) {
        self.constant_size = self.constants.len() as u8;

        /* Allocate memory */
        let new_memory_size = self.get_constant_size() + self.get_variable_size() + self.get_temp_size();
        let current_memory_size = self.memory.len();
        
        for _ in current_memory_size..new_memory_size.into() {
            self.memory.push(VmObject::convert(Rc::new(BramaPrimative::Empty)));
        }
    }

    fn get_memory(&mut self) -> &mut Vec<VmObject> { &mut self.memory }
    fn get_constant_size(&self) -> u8 { self.constant_size }
    fn get_variable_size(&self) -> u8 { self.variables.len() as u8 }
    fn get_temp_size(&self) -> u8     { self.temp_size }
    fn set_temp_size(&mut self, value: u8) { self.temp_size = value; }
    fn get_free_temp_slot(&mut self) -> u8 { 
        let index = self.temp_counter;
        self.temp_counter += 1;
        return self.get_constant_size() + self.get_variable_size() + index;
    }

    fn get_temp_counter(&self) -> u8 { self.temp_counter }
    fn set_temp_counter(&mut self, counter: u8) { self.temp_counter = counter; }
    fn inc_temp_counter(&mut self)    { self.temp_counter += 1; }
    fn reset_temp_counter(&mut self)  { self.temp_counter = 0; }

    fn add_constant(&mut self, value: Rc<BramaPrimative>) {
        let found = self.constants.iter().any(|x| {
            *x.deref() == *value
        });
        
        if !found { 
            self.memory.push(VmObject::convert(value.clone()));
        };
    }

    fn add_variable(&mut self, name: &String) -> u8 { 
        if !self.variables.contains_key(&name[..]) {
            self.memory.push(VmObject::convert(Rc::new(BramaPrimative::Empty)));
            self.variables.insert(name.to_string(), 0);
        }

        return self.get_variable_location(name).unwrap();
    }

    fn set_variable_value(&mut self, name: &String, object: VmObject) {
        let location = match self.get_variable_location(name) {
            Some(location) => location,
            _ => self.add_variable(name)
        };

        self.memory[location as usize] = object;
    }

    fn get_variable_location(&self, name: &String) -> Option<u8> {
        if self.variables.contains_key(name) {
            return Some(*self.variables.get(name).unwrap());
        }
        return None;
    }

    fn get_variable_value(&self, name: &String) -> Option<Rc<BramaPrimative>> {
        match self.get_variable_location(name) {
            Some(loc) => Some(self.memory[loc as usize].deref()),
            _ => None
        }
    }

    fn get_constant_location(&self, value: Rc<BramaPrimative>) -> Option<u8> {
        return match self.memory.iter().position(|x| { return *x.deref() == *value; }) {
            Some(number) => Some(number as u8),
            _ => None
        };
    }

    fn dump(&self) {
        println!("-------------------------------");
        println!("        MEMORY DUMP");
        println!("-------------------------------");
        for (index, item) in self.memory.iter().enumerate() {
            println!("| {:?} | {:?}", index, *item.deref());
        }
        println!("-------------------------------");
        println!("        VARIABLE DUMP");
        println!("-------------------------------");
        for (variable, value) in &self.variables {
            println!("| {:?}  [{:?}]", variable, value);
        }
        println!("-------------------------------");
    }
}
