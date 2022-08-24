pub fn is_stack_operation(reg: &str) -> bool {
    
    if reg != "sp" {
        return false;
    }
    true

}