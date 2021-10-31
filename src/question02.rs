fn checked_add(a: u32, b: u32) -> Option<u32> {
    if a + b < a || a + b < b {
        None
    } else {
        Some(a + b)
    }
}


fn sumLst(list: &[u32]) -> Option<u32>{
    let mut _sum: u32 = 0;

    for &item in list{
        println!("item: {}", item);
        let result: Option<u32> = checked_add(_sum, item);
        if (result.is_none()) {
            return None
        }

        _sum = _sum + item;
    }

    println!("sum: {}", _sum);
    return Some(_sum);
}



fn main() {
    let number_lst =[34, 50, 25, 100, 65];

    sumLst(&number_lst);
}
