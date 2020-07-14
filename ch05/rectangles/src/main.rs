#[derive(Debug, Copy,Clone)]
struct Rectangle {
    width: f64,
    height: f64
}

impl Rectangle {
    fn area(&self)->f64{
        self.width * self.height
    }
    fn holds(&self,other:&Rectangle)->bool{
        (self.width > other.width) && (self.height > other.height)
    }
    fn square(size:f64)->Self{
        Self {width:size,height:size}
    }
    fn new(width:f64,height:f64)->Self{
        Self{width,height}
    }
    fn resize(&mut self, w: f64, h: f64){
        self.width+=w;
        self.height+=h;
    }
}


fn main() {
    let r1 = Rectangle::new(7.,8.);
    let r2 = Rectangle::square(6.);
    let mut r3 = r1;
    r3.resize(-2.,3.);
    println!("r1(area {}): {:#?}",r1.area(),r1);
    println!("r2(area {}): {:#?}",r2.area(),r2);
    println!("r3(area {}): {:#?}",r3.area(),r3);
    println!("Does r1 hold r2? {}",r1.holds(&r2));
    println!("Does r2 hold r3? {}",r2.holds(&r3));
}
