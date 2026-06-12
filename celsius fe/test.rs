use std::io::{self, Write};
fn main()
{
    print!{"화씨 온도를 입력하시오 : \n"};
// print! 매크로는 버퍼를 비워줘야 화면에 바로 나온다.
    io::stdout().flush().unwrap();

    let mut input=String::new();
    io::stdin().read_line(&mut input).unwrap();
// 입력받은 문자의 공백을 제거하고, 64비트 실수로 변환
    let f_temp:f64 = input.trim().parse().unwrap();
    let c_temp=(f_temp-32.0)*(5.0/9.0);

    println!("섭씨 온도는 {:.2} 입니다", c_temp)
}