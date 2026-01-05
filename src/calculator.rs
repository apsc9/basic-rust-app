#[derive(Clone)]
pub enum OperationType {
    Add,
    Subtract,
    Multiply,
}

impl OperationType {
    pub fn get_sign(&self) -> &str {
        match self {
            OperationType::Add => "+",
            OperationType::Subtract => "-",
            OperationType::Multiply => "*",
        }
    }

    //perform the operations on 2 i64 values, return Some(result) on success
    // and None on overflow
    pub fn perform (&self, a: i64, b: i64) -> Option<i64> {
        match self {
            OperationType::Add => {
                let (result, overflow ) = a.overflowing_add(b);
                if overflow { None } else { Some(result)}
            }
            OperationType::Subtract => {
                let (result , overflow) = a.overflowing_sub(b);
                if overflow { None } else { Some(result)}
            }
            OperationType::Multiply => {
                let (result, overflow) = a.overflowing_mul(b);
                if overflow { None } else { Some(result)}
            }
        }
    }
}

#[derive(Clone)]
pub struct Operation {
    pub first_num: i64,
    pub second_num: i64,
    pub operation_type: OperationType,
}

impl Operation {
    pub fn new(first_num: i64, second_num:i64, operation_type: OperationType) -> Self {
        Self {
            first_num,
            second_num,
            operation_type,
        }
    }
}

pub struct Calculator {
    pub history: Vec<Operation>,
}

impl Calculator {
    pub fn new() -> Self {
        Self {
            history: Vec::new(),
        }
    }

    // perform addition and store the results in history 
    pub fn add(&mut self, x:i64, y: i64) -> Option<i64> {
        let operation = Operation::new(x,y, OperationType::Add);

        let result = operation.operation_type.perform(x, y);

        if let Some(result) = result {
            self.history.push(operation);
            Some(result)
        } else {
            None
        }
    }

    pub fn subtract(&mut self, x:i64, y:i64) -> Option<i64> {
        let operation = Operation::new(x,y, OperationType::Subtract);

        let result = operation.operation_type.perform(x, y);

        if let Some(res) = result {
            self.history.push(operation);
            Some(res)
        } else {
            None    
        }
    }

    pub fn multiply(&mut self, x:i64, y:i64) -> Option<i64> {

        let operation = Operation::new(x,y, OperationType::Multiply);

        let result = operation.operation_type.perform(x, y);    

        if let Some(res) = result {
            self.history.push(operation);
            Some(res)
        } else {
            None    
        }
    }

    // generate a formatted string returning all the operations in history
    // Format: "index: first_num operation_sign second_num = result\n"
    pub fn history(&self) -> String {
        let mut history_str = String::new();

        for (index, operation) in self.history.iter().enumerate() {
            let result = operation.operation_type.perform(operation.first_num, operation.second_num);

            if let Some(result) = result {
                let line = format!("{}: {} {} {} = {}\n",
                    index,
                    operation.first_num,
                    operation.operation_type.get_sign(),
                    operation.second_num,
                    result,
                );
                history_str.push_str(&line);
            }
        }

        history_str
    }





    // Repeat an operation from history by index
    // Add the repeated operation to history and return the result
    // Return None if the index is invalid

    pub fn repeat(&mut self, index: usize) -> Option<i64> {
        let prev = self.history.get(index)?;

        let result = prev.operation_type.perform(prev.first_num,prev.second_num);
        if let Some(result) = result {
            self.history.push(prev.clone());
            Some(result)
        } else {
            None
        }
    }



    // clear all the operations from history
    pub fn clear_history(&mut self) {
        self.history.clear();
    }
}







