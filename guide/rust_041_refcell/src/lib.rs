pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: 'a + Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value:usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;
        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use super::*;

    struct MockMessenger {
     // sent_messages:Vec<String>,
        sent_messages:RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
             // sent_messages: vec![],
                sent_messages: RefCell::new(vec![]) ,
            }
        }
    }

    impl Messenger for MockMessenger {
     // fn send(&mut self, message:&str) {
        fn send(&self, message:&str) {
         // self.sent_messages.push(String::from(message));
            self.sent_messages.borrow_mut().push(String::from(message));    //  borrow_mut方法：获得内部值的可变引用
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messager = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messager, 100);

        limit_tracker.set_value(80);
     // assert_eq!(mock_messager.sent_messages.len(),1);
        assert_eq!(mock_messager.sent_messages.borrow().len(),1);  // borrow方法：获得内部值的不可变引用
    }

}

/*
 * borrow方法：
 * 返回实现了Deref特性的智能指针Ref<T>
 * 
 * borrow——mut方法：
 * 返回实现了Deref特性的智能指针RefMut<T>
 * 
 * RefCell<T>会记录当前存在多少个活跃的Ref<T>和RefMut<T>，每次调用borrow不可变借用计数加1, 调用borrow_mut可变借用计数加1, Ref<T>离开作用域时不可变借用计数减1，RefMut<T>离开作用域时可变借用计数减1。
 * 
 * RefCell<T>通过以下形式来维护借用检查规则：
 * 在任何一个给定的时间里，只允许拥有多个不可变借用或一个可变借用。否则将触发panic
 */ 
