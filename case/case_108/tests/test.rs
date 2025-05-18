mod case_108 {
    use std::collections::HashMap;

    /*
     * case 108
     * 创建HashMap
     * HashMap的Key类型必须相同，Value类型也必须相同
     * 使用insert方法插入新的键值对
     */ 
    #[test]
    fn case_108() {
        let mut scores =  HashMap::new();

        // 使用insert方法在HashMap中插入键值对
        scores.insert("blue", 10);
        scores.insert("yellow", 50);
        println!("{scores:?}");
    }

    /*
     * case 109
     * 使用迭代器创建HashMap
     * 使用into_iter().collect()方法，将Vec<(key, value)>转化为HashMap
     */ 
    #[test]
    fn case_109() {
        // 使用一个Vec<(key, value)>变量来写入键值对
        let vec = vec![("key1", "value1"), ("key2", "value2")];
        let hashmap = vec.into_iter().collect::<HashMap<_,_>>();
        println!("{hashmap:?}");
    }

    /*
     * case 110
     * 读取HashMap中的元素
     * 使用get或get_mut来获取值的引用
     */ 
    #[test]
    fn case_110() {
        let vec = vec![(1, "value1".to_string()), (2, "value2".to_string())];
        let hashmap = vec.into_iter().collect::<HashMap<_,_>>();
        let key = 1;
        // get方法传入key值的引用，返回value值引用的Option
        assert_eq!(hashmap.get(&key).unwrap(), hashmap.get(&1).unwrap());

        // 使用get().copied()或get().cloned()方法直接获取value的拷贝值
        let value_v = hashmap.get(&1).cloned().unwrap();
        assert_eq!(value_v, "value1".to_string());
    }

    /*
     * case_111
     * 遍历HashMap
     * for循环中使用hashmap.iter()或&hashmap来遍历键值对
     * HashMap中的键值对是无序的
     */ 
    #[test]
    fn case_111() {
        let vec = vec![(1, "value1".to_string()), (2, "value2".to_string())];
        let hashmap = vec.into_iter().collect::<HashMap<_,_>>();
        for (key, value) in &hashmap {
            println!("{}: {:?}", key, value);
        }
        println!("{hashmap:?}");
    }

    /*
     * case 112
     * 更新HashMap里面的值
     * 可使用entry().and_modify().or_intert()进行流程化处理
     * 也可使用*hashmap.get_mut(key).unwrap()对值进行单点更新
     */ 
    #[test]
    fn case_112() {
        let vec = vec![("blue", 7), ("yellow", 13)];
        let mut hashmap = vec.into_iter().collect::<HashMap<_,_>>();

        // 本例中，使用entry().and_modify.or_insert()对hashMap的值进行更新
        // 若键值对存在则值加1，若键值对不存在则插入新键值对并初始化值为1
        hashmap.entry("blue").and_modify(|value| *value += 1).or_insert(1);
        hashmap.entry("yellow").and_modify(|value| *value += 1).or_insert(1);
        hashmap.entry("green").and_modify(|value| *value += 1).or_insert(1);
        println!("{hashmap:?}");

        *hashmap.get_mut("green").unwrap() = 19;
        println!("{hashmap:?}");
    }
}
