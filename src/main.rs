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
            if line[i].contains("+"){
                let ans = add(stack[stack.len() - 1], stack[stack.len() - 2]);
                stack.pop();
                stack.pop();
                stack.push(ans);
            }else if line[i].contains("-"){
                let ans = subtract(stack[stack.len() - 2], stack[stack.len() - 1]);
                stack.pop();
                stack.pop();
                stack.push(ans);
            }else if line[i].contains("*"){
                let ans = mulitply(stack[stack.len() - 2], stack[stack.len() - 1]);
                stack.pop();
                stack.pop();
                stack.push(ans);
            }else if line[i].contains("/"){
                let ans = divide(stack[stack.len() - 2], stack[stack.len() - 1]);
                stack.pop();
                stack.pop();
                stack.push(ans);
            }else if line[i].contains(".s"){
                println!("{:?}", stack);
            }else if line[i].contains(">>"){
                println!("{}", stack[stack.len() - 1]);
                stack.pop();
            }else if line[i].contains("."){
                stack.pop();
            }else if line[i].contains("="){
                let ans = equals(stack[stack.len() - 1], stack[stack.len() - 2]);
                stack.pop();
                stack.pop();
                stack.push(ans);
            }else if line[i].contains("if"){
                let ans = if_statments(stack[stack.len() - 1] as i32, &line, i as i32);
                if ans != 0{
                    i = ans as usize;
                }
            }else if line[i].contains("then"){
                continue;
            }else{
                let num: i32 = line[i].trim().parse().unwrap();
                stack.push(num);
            }
            println!("{}", i)
        }
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
