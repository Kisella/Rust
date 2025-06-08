mod test {
    use std::{sync::mpsc, thread, time::Duration};

    /*
     * case 122
     * 使用mpsc::channel方法来创建通道，进行线程间通讯
     * 发送端(Sender)和接收端(Receiver)的数据类型是唯一的，一旦确定就不能更改
     */
    #[test]
    fn case_122() {
        // 使用mpsc::channel函数来创建通道
        let (tx, rx) = mpsc::channel();
        thread::scope(|s| {
            s.spawn(|| {
                let val = String::from("hello world");
                println!("spawned thread send: {val:?}");
                tx.send(val).unwrap(); //  调用send方法会获得数据的所有权
            });
        });

        let recv = rx.recv().unwrap(); //  recv方法会阻塞当前线程，若不想阻塞线程可使用try_recv方法
        println!("main thead recv: {recv:?}");
    }

    /*
     * case 123
     * 使用通道发送多个消息
     * 接收端可以使用迭代器来接受多条消息
     * 使用for循环来处理接收端的多条消息时，若发送端未关闭则会阻塞线程
     */
    #[test]
    fn case_123() {
        let (tx, rx) = mpsc::channel();
        thread::scope(|s| {
            s.spawn(|| {
                let vals = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
                for val in vals {
                    tx.send(val).unwrap();
                    thread::sleep(Duration::from_millis(1));
                }
            });
        });

        drop(tx); //  关闭发送端
        for received in rx.iter() {
            println!("main thread recv: {received:?}");
        }
    }

    /*
     * case_124
     * 多个生产者通过管道发送消息
     */
    #[test]
    fn case_124() {
        let (tx, rx) = mpsc::channel();
        thread::scope(|s| {
            s.spawn(|| {
                let vals = [5, 4, 3, 2, 1];
                for val in vals {
                    tx.send(val).unwrap();
                    println!("spawned thread 1 send: {val:?}");
                    thread::sleep(Duration::from_micros(5));
                }
            });
            s.spawn(|| {
                let vals = [5, 4, 3, 2, 1];
                for val in vals {
                    tx.send(val).unwrap();
                    println!("spawned thread 2 send: {val:?}");
                    thread::sleep(Duration::from_micros(1));
                }
            });
        });

        drop(tx); //  关闭发送端
        for received in rx.iter() {
            println!("main thread recv: {received:?}");
        }
    }
}
