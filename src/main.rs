extern  crate rand;
use rand::{Rng};
fn main() {

    println!("Win 95 product key generator by Exodow_RGB");
    let mut sa ;
    let mut sb ;
    let mut sc ;
    let mut sd ;
    let mut se ;

    loop {
        sa = rand::thread_rng().gen_range(0..9);
        sb = rand::thread_rng().gen_range(0..9);
        sc = rand::thread_rng().gen_range(0..9);
        sd = rand::thread_rng().gen_range(0..9);
        se = rand::thread_rng().gen_range(0..9);
        if (sa + sb + sc + sd + se)% 7 == 0  {
            break;
        }
    }
    println!("product key: {}{}-OEM-00{}{}{}{}{}-{}",rand::thread_rng().gen_range(100..366),rand::thread_rng().gen_range(95..99),sa,sb,sc,sd,se,rand::thread_rng().gen_range(10000..99999));
}
