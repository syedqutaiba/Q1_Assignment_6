use std::io;

fn main() {
    let mut actual_price = String::new();
    let mut discounted_price = String::new();
    let mut paid_amount = String::new();
        
    println!("\n-------");
    println!("Inputs");
    println!("-------\n");
    
    println!("Enter Actual Price");
    io::stdin().read_line(&mut actual_price).expect("Invalid value");
    let actual_price: i32 = actual_price.trim().parse().expect("Enter whole amount");

    println!("Enter Discounted Price");
    io::stdin().read_line(&mut discounted_price).expect("Invalid value");
    let discounted_price: i32 = discounted_price.trim().parse().expect("Enter whole amount");

    println!("Enter Customer payment amount");
    io::stdin().read_line(&mut paid_amount).expect("Invalid value");
    let paid_amount: i32 = paid_amount.trim().parse().expect("Enter whole amount");

    let (discounted_amount, percent_discount, balance) = calculation(actual_price, discounted_price, paid_amount);
    
    println!("\n-------");
    println!("Results");
    println!("-------\n");

    println!("Discounted amount : {}", discounted_amount);
    println!("Percentage discount : {}%", percent_discount);
    println!("Balance amount : {}\n", balance);

    if percent_discount <= 10 {
        println!("Azadi Offer\n");
    } else if percent_discount > 10 || percent_discount <= 20 {
        println!("Eid Offer\n");
    } else {
        println!("Clerance Sale\n");
    }
}
    

fn calculation(price: i32, disc_price: i32, amount: i32) -> (i32, i32, i32) {
    let discounted_amount: i32 = price - disc_price;
    let discount_factor: i32 = discounted_amount * 100;
    let percent_discount: i32 = discount_factor / price;
    let balance: i32 = amount - disc_price;
    (discounted_amount, percent_discount, balance)
}