mod test {
    use std::{thread, time::Duration};

    /*
     * case 118
     * 使用thread::spawn方法创建线程
     * 当主线程完成时，所有已经创建的其他线程，无论是否已经完成，都会被强制结束
     * 对于多线程的程序，程序员无法保证线程运行的顺序，线程的顺序由操作系统调度决定
     */
    #[test]
    fn case_118() {
        thread::spawn(|| {
            for i in 1..10 {
                println!("spawned thread: {i}");
                thread::sleep(Duration::from_millis(1));
            }
        });

        for i in 1..5 {
            println!("main thread: {i}");
            thread::sleep(Duration::from_millis(1));
        }
    }

    /*
     * case 119
     * 使用JoinHandle.join方法等待线程运行结束
     */
    #[test]
    fn case_119() {
        //  将线程句柄赋值给一个变量
        let handle = thread::spawn(|| {
            for i in 1..10 {
                println!("spawned thread: {i}");
                thread::sleep(Duration::from_millis(1));
            }
        });

        for i in 1..5 {
            println!("main thread: {i}");
            thread::sleep(Duration::from_millis(1));
        }
        handle.join().unwrap(); //  调用线程句柄的join方法阻塞线程（当前为主线程）直到句柄代表的线程结束
    }

    /*
     * case 120
     * 在spawn子线程中需要使用主线程的数据，需要使用move关键字使闭包捕获所有权
     */
    #[test]
    fn case_120() {
        let v = vec![1, 2, 3];
        let handle = thread::spawn(move || {
            println!("{v:?}");
        });
        handle.join().unwrap()
    }

    /*
     * case 121
     * 使用thread::scope方法创建作用域线程
     * 使用作用域线程允许子线程安全地借用父线程的数据而无需获取其所有权，也无需数据满足'static生命周期
     * 当作用域线程作用域结束时，父线程会自动等待子线程完成而无需手动调用join方法
     */
    #[test]
    fn case_121() {
        let data = vec![1, 2, 3];
        thread::scope(|s| {
            s.spawn(|| {
                for i in 0..10 {
                    println!("spawned thread 1: {i:?}");
                    thread::sleep(Duration::from_millis(1));
                }
            });
            // 一个作用域线程可以生成多个子线程
            s.spawn(|| {
                println!("spawned thread 2: {data:?}"); //  子线程可安全借用父线程数据
            });
        }); // 线程作用域结束，父线程等待所有子线程结束

        println!("All spawn thread finished");
    }
}
