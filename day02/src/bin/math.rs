fn main() {
    let originnum =11974834;
    let numdigits:f32 = f32::log(originnum as f32 , 10.0).ceil();
    let tophalfnum = originnum / 10i32.pow(numdigits as u32 / 2u32);
    let bottomhalfnum = originnum % 10i32.pow(numdigits as u32 / 2u32);
    print!("top: {}, bottom: {}", {tophalfnum}, {bottomhalfnum});
}