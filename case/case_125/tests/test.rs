mod test {
    use std::{
        sync::{Arc, Mutex},
        thread,
    };

    /*
     * case 125
     * Mutex互斥锁与Arc原子引用计数
     * 若要访问互斥锁里面的数据，需要调用Mutex.lock().unwrap()方法获取锁MutexGuard
     * MutexGuard<T>智能指针存在期间，其他线程调用Mutex.lock().unwrap()方法将被阻塞，知道MutexGuard走出作用域，Mutex互斥锁解锁数据，其他线程可以获取锁
     * Arc<T>原子引用计数指针，可在并发环境中安全实现多重所有权功能（类似单线程中的Rc<T>）
     * Arc<T>通常嵌套Mutex<T>使用，以便在多个线程间共享同一个互斥锁
     */
    #[test]
    fn case_125() {
        let counter = Arc::new(Mutex::new(0));
        let mut handles = Vec::new();

        for _ in 0..10 {
            // 调用Arc.clone方法生成多个互斥锁指针，实现多重所有权，方便线程间共享互斥锁
            let counter = counter.clone();
            let handle = thread::spawn(move || {
                let mut num = counter.lock().unwrap();
                *num += 1;
            }); //    线程作用域结束，MutexGuard释放，Mutex解锁
            handles.push(handle);
        }
        // 遍历每一个线程句柄，为每个句柄调用join方法，确保所有子线程完成
        for handle in handles {
            handle.join().unwrap();
        }
        println!("Result: {}", *counter.lock().unwrap());
    }

    /*
     * case 126
     * 在作用域线程中可以由多个线程获取Mutex的引用而无需使用Arc<T>类型
     */
    #[test]
    fn case_126() {
        let counter = Mutex::new(0);

        thread::scope(|s| {
            for _ in 0..10 {
                s.spawn(|| {
                    let mut num = counter.lock().unwrap();
                    *num += 1;
                });
            }
        });
        println!("Result: {}", counter.lock().unwrap());
    }
}
