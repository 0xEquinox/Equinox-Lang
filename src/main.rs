use std::io;


fn main() {

    println!("Equinox Lang v0.1");

    let mut running = true;
    let mut stack:Vec<i32> = Vec::new();

    while running {
        let line = console_input();
        let line = line.split(" ");
        let line: Vec<&str> = line.collect();

        for mut i in 0..line.len() {
            stack = execute(line[i],  &stack, &line, i);

        }
    }
}
fn do_loop(line: Vec<&str>, start: i32, end: i32){
    for i in start..end{

    }
}
fn if_statments(enumerator: i32, line: &Vec<&str>, current_index: i32) -> i32{
    if enumerator == 1{
        return 0;
    }else{
        let  i = current_index;
        let mut count = 0;
        for i in 0..line.len(){
            if line[i] == "then" {
                return count;
            }
            count += 1;
        }
        println!("Error");
    }
    return 0;
}

fn equals(num1: i32, num2: i32) -> i32{
    if num1 == num2 {
        return 1;
    }else{
        return 0;
    }
}

fn add(num1: i32, num2: i32) -> i32{
    return num1 + num2;
}

fn subtract(num1: i32, num2: i32) -> i32{
    return num1 - num2;
}

fn mulitply(num1: i32, num2: i32) -> i32{
    return num1 * num2;
}

fn divide(num1: i32, num2: i32) -> i32{
    return num1 / num2;
}


fn console_input() -> String{

    let mut input = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut input);

    return input;
}

fn execute(command: &str, stack1: &Vec<i32>, line: &Vec<&str>, i: usize) -> Vec<i32>{

    let mut stack = stack1.clone();

    if command.contains("+"){
        let ans = add(stack[stack.len() - 1], stack[stack.len() - 2]);
        stack.pop();
        stack.pop();
        stack.push(ans);
    }else if command.contains("-"){
        let ans = subtract(stack[stack.len() - 2], stack[stack.len() - 1]);
        stack.pop();
        stack.pop();
        stack.push(ans);
    }else if command.contains("*"){
        let ans = mulitply(stack[stack.len() - 2], stack[stack.len() - 1]);
        stack.pop();
        stack.pop();
        stack.push(ans);
    }else if command.contains("/"){
        let ans = divide(stack[stack.len() - 2], stack[stack.len() - 1]);
        stack.pop();
        stack.pop();
        stack.push(ans);
    }else if command.contains(".s"){
        println!("{:?}", stack);
    }else if command.contains(">>"){
        println!("{}", stack[stack.len() - 1]);
        stack.pop();
    }else if command.contains("."){
        stack.pop();
    }else if command.contains("="){
        let ans = equals(stack[stack.len() - 1], stack[stack.len() - 2]);
        stack.pop();
        stack.pop();
        stack.push(ans);
    }else if command.contains("if"){
        let ans = if_statments(stack[stack.len() - 1] as i32, &line, i as i32);
        if ans != 0{

        }
    }else if command.contains("then"){

    }else{
        let num: i32 = command.trim().parse().unwrap();
        stack.push(num);
    }

    return stack;
}