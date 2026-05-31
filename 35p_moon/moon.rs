// 달의 거리를 시작으로 사칙연산하는 코딩 연습

fn main()
{
    let moon=384400.0;
    let car=80.0;
    let btrain=300.0;
// 파이썬과는 다르게, 시작부터 소수점을 확인하고 시작

    let moon_car=moon/car/24.0;
    println!("달까지 자동차로 {}일", moon_car);
    let moon_btrain=moon/btrain/24.0;
    println!("달까지 KTX로 {}일", moon_btrain);
}