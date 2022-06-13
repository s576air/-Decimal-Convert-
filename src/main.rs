fn main() {loop {
    println!("현재 진법을 입력해주세요(2,4,8,10,16)");
    let radix1 = string_input().parse().unwrap_or(10.0);

    println!("바꿀 진법을 입력해주세요(2,4,8,10,16)");
    let radix2 = string_input().parse().unwrap_or(10.0);

    println!("숫자를 입력해주세요");
    let input_str = string_input();

    println!("결과");
    println!("  {}진법 -> {}진법", radix1, radix2);
    println!("  기존 숫자: {}", input_str);

    let result = 진법_변환(input_str, radix1, radix2, debug_flag);
    
    println!("  바뀐 숫자: {}", result);
    println!("\n------------------\n")
}}

// radix진법으로 표현된 num에서 new_radix진법으로 표현된 결과물을 리턴합니다.
fn 진법_변환(mut input: String, radix: f64, new_radix: f64) -> String {
    let mut n: f64 = 0.0;

    // 소수점을 입력받았을 경우에만 소수점을 처리해 n 값에 더합니다.
    match input.find('.') {
        Some(_) => {
            let (mut n1, mut n2) = input.split_once('.').unwrap();
            let mut size: f64 = 1.0;
            for c_char in n2.as_bytes() {
                let i = c_char.to_u8() as f64;
                size /= radix;
                n += size * i;
            }

            input = n1.to_string();
            // println!("debug: 소수점을 f64로 변환: {}", n);
        },
        None => {},
    }

    // 정수 부분을 처리해, n 값에 더합니다.
    let c_chars = input.as_bytes();
    let mut index = c_chars.len() - 1;
    let mut size: f64 = 1.0;
    loop {
        n += c_chars[index].to_u8() as f64 * size;

        size *= radix;
        
        if index == 0 { break }
        index -= 1;
    }
    // println!("debug: 정수(와 소수점)을 f64로 변환: {}", n);

    // n 값의 정수 부분을 문자열로 변환합니다.(이름은 정수지만 타입은 f64입니다.)
    size = 1.0;
    let mut remainder = 0.0; // 여기서는 소수점을 제외한 나머지를 의미합니다.(123.45 -> 3, 20, 100)
    let mut v = Vec::with_capacity(20); // 문자열이 될 예정입니다.
    loop {
        remainder = n % (new_radix * size);
        remainder -= remainder % size;
        v.push(((remainder / size + 0.5) as u8).to_c_char());
        n -= remainder;

        if n < 1.0 { break }
        size *= new_radix;
    }
    v.reverse();
    // println!("debug: f64의 정수 부분을 문자열로 변환: {}", String::from_utf8(v.clone()).expect("debug 에러1"));

    // n 값의 소수 부분을 문자열로 변환합니다.
    size = 1.0 / new_radix;
    let mut count = 0;
    if n > 0.001 { v.push(b'.') }
    while n > 0.001 && count < 10 {
        n *= new_radix;
        remainder = n - n % 1.0;
        v.push(((remainder + 0.5) as u8).to_c_char());
        n -= remainder;

        count += 1;
    }
    // println!("debug: f64를 완전히 문자열로 변환: {}", String::from_utf8(v.clone()).expect("debug 에러2"));

    String::from_utf8(v).expect("벡터를 문자열로 변환하는데 실패했습니다")
}

trait ConvertCCharAndU8 {
    fn to_u8(&self) -> u8;
    fn to_c_char(&self) -> u8;
}

impl ConvertCCharAndU8 for u8 {
    fn to_u8(&self) -> u8 {
        match *self {
            b'0'..=b'9' => *self - b'0',
            b'a'..=b'z' => *self - b'a' + 10,
            b'A'..=b'Z' => *self - b'A' + 10,
            _ => panic!("c_char가 아닌 유형을 입력했습니다."),
        }
    }

    fn to_c_char(&self) -> u8 {
        match *self {
            0..=9 => b'0' + *self,
            10..=35 => b'a' + *self - 10,
            _ => panic!("36진법 이상은 변환할 수 없습니다."),
        }
    }
}

fn string_input() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).expect("input Error");
    s = s[..s.len()-2].to_string();
    s
}