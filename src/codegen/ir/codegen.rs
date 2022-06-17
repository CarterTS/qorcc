use crate::parser::*;
use crate::tokenizer::*;
use crate::errors::*;

use super::*;

/// Convert a parse tree into intermediate representation
pub fn parse_tree_to_ir(tree: ParseTreeNode) -> CompilerResult<IR>
{
    trace!("Convert to Intermediate Representation");

    let mut ir = IR::new();

    if let ParseTreeNode::CompilationUnit { children } = tree
    {
        for child in children
        {
            ir.functions.push(parse_tree_function_to_ir(child)?);
        }
    }
    else
    {
        panic!("Expected a CompilationUnit parse tree node, got {}", tree);
    }

    Ok(ir)
}

/// Convert a function parse tree node into an IRFunction
pub fn parse_tree_function_to_ir(tree: ParseTreeNode) -> CompilerResult<IRFunction>
{
    if let ParseTreeNode::Function { name, return_type, child, arguments, .. } = tree
    {
        IRFunction::with_statement_and_args(name, return_type, *child, arguments)
    }
    else
    {
        panic!()
    }
}

impl IRFunction
{
    pub fn with_statement_and_args(name: String, return_type: ValueType, statement: ParseTreeNode, arguments: Vec<(String, ValueType, Token)>) -> CompilerResult<Self>
    {
        let mut result = Self
        {
            name,
            return_type,
            blocks: vec![IRBlock::new(0)],
            current_block: 0,
            scope_stack: Vec::new(),
            next_register: 0,
            next_block: 1
        };

        let scope = IRScope::from_arguments(arguments, &mut result);

        result.scope_stack.push(scope);

        result.add_statement(statement)?;

        Ok(result)
    }

    pub fn get_variable_value(&mut self, expression: &ParseTreeNode) -> CompilerResult<IRValue>
    {
        if let ParseTreeNode::VariableExpression { name, token } = expression
        {
            let mut result = None;

            for scope in self.scope_stack.iter().rev()
            {
                result = result.or(scope.access_variable(&name));
                if result.is_some()
                {
                    break;
                }
            }

            if let Some(result_value) = result
            {
                Ok(result_value)
            }
            else
            {
                Err(CodegenError::compile_error(format!("Variable {} is not defined", name), &token).into())
            }
        }
        else
        {
            panic!()
        }
    }

    pub fn add_jump(&mut self, block_index: usize) -> CompilerResult<()>
    {
        self.mut_current_block().add_instruction(IRInstruction::Jump { dest: block_index });

        Ok(())
    }

    pub fn add_three_op_instruction(&mut self, children: &Vec<ParseTreeNode>) -> CompilerResult<(IRValue, IRValue, IRValue)>
    {
        let src1 = self.generate_expression(&children[0])?;
        let src2 = self.generate_expression(&children[1])?;

        let dest = IRValue::Register(self.alloc_next_register());

        Ok((dest, src1, src2))
    }

    pub fn generate_expression(&mut self, expression: &ParseTreeNode) -> CompilerResult<IRValue>
    {
        match expression
        {
            ParseTreeNode::ConstantExpression{ value, .. } => Ok(IRValue::Immediate(value.clone())),
            ParseTreeNode::VariableExpression { .. } => self.get_variable_value(expression),
            ParseTreeNode::AdditiveExpression{operation, children, .. } =>
            {
                let (dest, src1, src2) = self.add_three_op_instruction(children)?;

                match operation
                {
                    AdditiveExpressionOperation::Addition =>  self.mut_current_block().add_instruction(
                            IRInstruction::Add { dest: dest.clone(), src1, src2 }),
                    AdditiveExpressionOperation::Subtraction =>  self.mut_current_block().add_instruction(
                        IRInstruction::Sub { dest: dest.clone(), src1, src2 }),
                }

                Ok(dest)
            },
            ParseTreeNode::MultiplicativeExpression{operation, children, .. } =>
            {
                let (dest, src1, src2) = self.add_three_op_instruction(children)?;

                match operation
                {
                    MultiplicativeExpressionOperation::Multiplication =>  self.mut_current_block().add_instruction(
                        IRInstruction::Mul { dest: dest.clone(), src1, src2 }),
                    MultiplicativeExpressionOperation::Division =>  self.mut_current_block().add_instruction(
                        IRInstruction::Div { dest: dest.clone(), src1, src2 }),
                    MultiplicativeExpressionOperation::Modulus =>  self.mut_current_block().add_instruction(
                        IRInstruction::Mod { dest: dest.clone(), src1, src2 }),
                }

                Ok(dest)
            },
            ParseTreeNode::EqualityExpression { operation, children, .. } =>
            {
                let (dest, src1, src2) = self.add_three_op_instruction(children)?;

                let cond = match operation
                {
                    EqualityExpressionOperation::Equality => IRBranchCondition::Equal,
                    EqualityExpressionOperation::Nonequality => IRBranchCondition::NotEqual,  
                };

                self.mut_current_block().add_instruction(
                    IRInstruction::Conditional { condition: cond, dest: dest.clone(), src1, src2 });

                Ok(dest)
            },
            ParseTreeNode::RelationalExpression { operation, children, .. } =>
            {
                let (dest, src1, src2) = self.add_three_op_instruction(children)?;

                let cond = match operation
                {
                    RelationalExpressionOperation::GreaterThan => IRBranchCondition::GreaterThan,
                    RelationalExpressionOperation::GreaterThanOrEqual => IRBranchCondition::GreaterThanEqualTo,
                    RelationalExpressionOperation::LessThan => IRBranchCondition::LessThan,  
                    RelationalExpressionOperation::LessThanOrEqual => IRBranchCondition::LessThanEqualTo,
                };

                self.mut_current_block().add_instruction(
                    IRInstruction::Conditional { condition: cond, dest: dest.clone(), src1, src2 });

                Ok(dest)
            }
            ParseTreeNode::PostfixExpression { operation, children, .. } =>
            {
                match operation
                {
                    PostfixExpressionOperation::ArrayIndexing => todo!(),
                    PostfixExpressionOperation::MemberAccess => todo!(),
                    PostfixExpressionOperation::IndirectMemberAccess => todo!(),
                    PostfixExpressionOperation::Increment => todo!(),
                    PostfixExpressionOperation::Decrement => todo!(),
                    PostfixExpressionOperation::InitializerList => todo!(),
                    PostfixExpressionOperation::FunctionCall => 
                    {
                        let dest_reg_num = self.alloc_next_register();
                        let dest = IRValue::Register(dest_reg_num);

                        let argument_count = children.len() - 1;

                        assert!(argument_count < 4); // TODO: Make sure we can handle more registers than just four arguments
                        
                        // Get all of the arguments
                        let mut argument_values = Vec::new();

                        for arg in &children[1..]
                        {
                            let arg_val = self.generate_expression(arg)?;

                            let arg_reg = IRValue::Register(self.alloc_next_register());

                            self.mut_current_block().add_instruction(IRInstruction::Add { dest: arg_reg.clone(), src1: arg_val, src2: IRValue::Immediate(Value::code_constant(0)) });

                            argument_values.push(arg_reg);
                        }

                        // Backup the argument registers
                        for reg_number in 0..argument_count
                        {
                            if reg_number == dest_reg_num { continue; }
                            self.mut_current_block().add_instruction(IRInstruction::Backup { register: reg_number })
                        }

                        self.mut_current_block().add_instruction(IRInstruction::FunctionCall { name: children[0].get_variable_name().unwrap(), arguments: argument_values.clone() });

                        self.mut_current_block().add_instruction(IRInstruction::LoadRet { dest: dest.clone() });

                        // Restore the argument registers
                        for reg_number in (0..argument_count).rev()
                        {
                            if reg_number == dest_reg_num { continue; }
                            self.mut_current_block().add_instruction(IRInstruction::Restore { register: reg_number })
                        }

                        Ok(dest)
                    }
                }
            },
            _ => 
            {
                error!("Unhandled Expression Type {}", expression);
                todo!()
            }
        }
    }

    pub fn add_statement(&mut self, statement: ParseTreeNode) -> CompilerResult<()>
    {
        match statement
        {
            ParseTreeNode::StatementBlock { children } => 
            {
                for child in children
                {
                    self.add_statement(child)?;
                }

                Ok(())
            },
            ParseTreeNode::ReturnStatement { child } => 
            {
                if let Some(expression) = child
                {
                    let value = self.generate_expression(&*expression)?;
                    self.mut_current_block().add_instruction(IRInstruction::Return { value });
                }
                else
                {
                    self.mut_current_block().add_instruction(IRInstruction::Return { value: IRValue::Immediate(Value::code_constant(0)) });
                }

                Ok(())
            },
            ParseTreeNode::IfStatement { children } =>
            {
                // Backup the current block index
                let initial_block = self.current_block;
                
                // Extract the condition and the statement to be executed if that condition evaluates to true
                let condition = children.get(0).expect("IfStatement needs at least two children");
                let statement = children.get(1).expect("IfStatement needs at least two children");

                // Allocate the true branch
                let true_branch = self.alloc_next_block();

                // Write the true statement to the true branch
                self.current_block = true_branch;
                self.add_statement(statement.clone())?;

                // Allocate the false branch
                let false_branch = self.alloc_next_block();

                // Add the conditional jump instruction to the initial block
                self.current_block = initial_block;
                let condition_value = self.generate_expression(condition)?;
                self.mut_current_block().add_instruction(IRInstruction::Branch { condition: IRBranchCondition::NotEqual, src1: condition_value, src2: IRValue::Immediate(Value::code_constant(0)), dest_true: true_branch, dest_false: false_branch });

                // We need to know what branch should be skipped to when we write to the true branch
                let skip_branch;

                if let Some(false_statement) = children.get(2)
                {
                    skip_branch = self.alloc_next_block();

                    // Add the false statement to the false branch
                    self.current_block = false_branch;
                    self.add_statement(false_statement.clone())?;
                    self.add_jump(skip_branch)?;
                }
                else
                {
                    skip_branch = false_branch;
                }

                // Add the jump to the skip branch to the true branch
                self.current_block = true_branch;
                self.add_jump(skip_branch)?;

                self.current_block = skip_branch;

                Ok(())
            },
            _ => 
            {
                error!("Unhandled statement {}", statement);
                panic!()
            },
        }
    }
}