fn main() {
    
    // the inpout procces was taken from ChatGPt, since it wasn't teached on the lessons

    use std::io;

    


    let mut input_num1 = String::new();
    let mut input_num2 = String::new();
    let mut calc_func_input = String::new();


   
    io::stdin().read_line(&mut input_num1)
        .expect("Failed to read line");
    io::stdin().read_line(&mut calc_func_input)
        .expect("Failed to read line");    
    io::stdin().read_line(&mut input_num2)
        .expect("Failed to read line");

    let input_num1: f32 = input_num1.trim().parse()
        .expect("Please type a number!");
    
    let input_num2: f32 = input_num2.trim().parse()
        .expect("Please type a number!");

    enum CaclFunc {
        Plus,
        Minus,
        Mul,
        Div,
    }    

    let calc_func = match calc_func_input.trim() {
        "+" => CaclFunc::Plus,
        "-" => CaclFunc::Minus,
        "*" => CaclFunc::Mul,
        "/" => CaclFunc::Div,
        _ => panic!("Invalid operation"),

    };

   
    

    match calc_func {
        CaclFunc::Plus => {
            let calc_result: f32 = input_num1 + input_num2;
            println!("Calc result is: {} + {} = {}", input_num1,  input_num2 , calc_result);
        }
        CaclFunc::Minus => {
            let calc_result: f32 = input_num1 - input_num2;
            println!("Calc result is: {} - {} = {}", input_num1,  input_num2 , calc_result);
        }
        CaclFunc::Mul => {
            let calc_result: f32 = input_num1 * input_num2;
            println!("Calc result is: {} * {} = {}", input_num1,  input_num2 , calc_result);
        }
        CaclFunc::Div => {
            let calc_result: f32 = input_num1 / input_num2;
            println!("Calc result is: {} / {} = {}", input_num1,  input_num2 , calc_result);
        }
    }


    

   
    


}
