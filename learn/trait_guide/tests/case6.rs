mod dynamic_dispatch {
    trait Widget {
        fn render(&self);
    }
    struct Button {
        text: String,
    }
    struct CheckBox {
        checked: bool,
    }
    impl Widget for Button {
        fn render(&self) {
            println!("Rendering buttom: {}", self.text);
        }
    }
    impl Widget for CheckBox {
        fn render(&self) {
            println!("CheckBox: {}", self.checked);
        }
    }

    // 使用动态派发，在容器中储存不同的类型对象
    fn build_ui() -> Vec<Box<dyn Widget>> {
        vec![
            Box::new(Button { text: "OK".into() }),
            Box::new(CheckBox { checked: true }),
        ]
    }

    #[test]
    fn test() {
        let ui = build_ui();
        for widget in ui {
            widget.render();
        }
    }
}
