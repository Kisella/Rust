use zero_one_knaspsack_problem::algorithm::knapsack;
use zero_one_knaspsack_problem::model::COMMODITY_DATA;
use zero_one_knaspsack_problem::model::Commodity;
const CAPACITY: usize = 15;

fn main() {
    let commodity_list = Commodity::get_commodity_list_from_data(&COMMODITY_DATA);
    for commodity in commodity_list.iter() {
        println!("{commodity:?}");
    }
    let (optimum, elects) = knapsack(&commodity_list, CAPACITY);
    print!("\n背包容量{CAPACITY}, 选择商品:");
    for &index in elects.iter() {
        print!(" {}", &commodity_list[index].name);
    }
    println!("\n可获得最大价值{}元", optimum);
}
