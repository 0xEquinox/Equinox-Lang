use std::io;


fn main() {

    println!("Equinox Lang v0.1");

    let mut running = true;
    let mut stack:Vec<i32> = Vec::new();

    while running {
        let line = console_input();
        let line = line.split(" ");
        let line: Vec<&str> = line.collect();

        for i in line{
            if i.contains("+"){
                let ans = add(stack[stack.len() - 1], stack[stack.len() - 2]);
                stack.pop();
                stack.pop();
                stack.push(ans);
            }else if i.contains("-"){
                let ans = subtract(stack[stack.len() - 2], stack[stack.len() - 1]);
                stack.pop();
                stack.pop();
                stack.push(ans);
            }else if i.contains("*"){
                let ans = mulitply(stack[stack.len() - 2], stack[stack.len() - 1]);
                stack.pop();
                stack.pop();
                stack.push(ans);
            }else if i.contains("/"){
                let ans = divide(stack[stack.len() - 2], stack[stack.len() - 1]);
                stack.pop();
                stack.pop();
                stack.push(ans);
            }else if i.contains(".s"){
                println!("{:?}", stack);
            }else if i.contains(">>"){
                println!("{}", stack[stack.len() - 1]);
            }else{
                let num: i32 = i.trim().parse().unwrap();
                stack.push(num);
            }
        }
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
