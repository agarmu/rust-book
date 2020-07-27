use trait_objects::*;

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("Drawing box with Options:\n\t{:?}", self.options);
    }
}

fn main() {
    let a = Button {
        width: 32,
        height: 32,
        label: "Hi".to_owned(),
    };
    let b = SelectBox {
        width: 32,
        height: 32,
        options: vec!["Abc", "Def", "Ghi"]
            .into_iter()
            .map(|a| a.to_owned())
            .collect(),
    };
    let c = Screen {
        components: vec![Box::new(a), Box::new(b)],
    };
    c.run();
}
