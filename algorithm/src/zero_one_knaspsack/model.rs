#[derive(Debug, Clone, Copy)]
pub struct Commodity {
    pub name: &'static str,
    pub price: u32,
    pub volume: usize,
}

impl Commodity {
    pub fn new(name: &'static str, price: u32, volume: usize) -> Commodity {
        Commodity {
            name,
            price,
            volume,
        }
    }

    pub fn get_commodity_list_from_data(
        commodity_data: &[(&'static str, u32, usize)],
    ) -> Vec<Commodity> {
        let mut commodity_list = vec![];
        for item in commodity_data {
            commodity_list.push(Commodity::new(item.0, item.1, item.2));
        }
        commodity_list
    }
}