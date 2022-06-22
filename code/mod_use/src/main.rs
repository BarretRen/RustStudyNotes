use barretlib::barret1::barret2;
mod bin;//前置声明

fn main() {
    let tmp = barret2::A{width: 4, height: 5};
    println!("{}", tmp.aera());
    bin::barret3::print_name();
}
