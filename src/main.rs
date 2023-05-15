extern  crate rand;
use rand::{thread_rng, Rng};
fn main() {

    println!("Win 95 product key generator by Exodow_RGB");
    let mut xxx = rand::thread_rng().gen_range(100..366);
    let mut yy = rand::thread_rng().gen_range(95..99);
    let mut sa = rand::thread_rng().gen_range(0..9);
    let mut sb = rand::thread_rng().gen_range(0..9);
    let mut sc = rand::thread_rng().gen_range(0..9);
    let mut sd = rand::thread_rng().gen_range(0..9);
    let mut se = rand::thread_rng().gen_range(0..9);
    let mut zzzzz = rand::thread_rng().gen_range(10000..99999);

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
    println!("product key: {}{}-OEM-00{}{}{}{}{}-{}",xxx,yy,sa,sb,sc,sd,se,zzzzz);
}
