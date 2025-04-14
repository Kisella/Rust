/* 问题背景：
 *     超市允许顾客使用一个体积大小为13的背包，选择一件或多件商品带走。问，如何才能带走总价最多的商品
 *
 *     商品     价格     体积
 *
 *     啤酒     24       10
 *     汽水     2        3
 *     饼干     9        4
 *     面包     10       5
 *     牛奶     9        4
 */

#[derive(Debug, Clone, Copy)]
pub struct Commodity {
    pub name: &'static str,
    pub price: u32,
    pub volume: usize,
}

pub const COMMODITY_NUM: usize = 6;
pub const COMMODITY_DATA: [(&'static str, u32, usize); COMMODITY_NUM] = [
    ("汽水", 2, 3),
    ("饼干", 9, 4),
    ("啤酒", 24, 10),
    ("面包", 10, 5),
    ("牛奶", 9, 4),
    ("葡萄", 4, 1),
];

impl Commodity {
    pub fn new(name: &'static str, price: u32, volume: usize) -> Commodity {
        Commodity {
            name,
            price,
            volume,
        }
    }

    pub fn get_commodity_list_from_data(
        commodity_data: [(&'static str, u32, usize); COMMODITY_NUM],
    ) -> Vec<Commodity> {
        let mut commodity_list = vec![];
        for item in commodity_data.iter() {
            commodity_list.push(Commodity::new(item.0, item.1, item.2));
        }
        commodity_list
    }
}
