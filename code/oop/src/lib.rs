//定义一个trait, 类似与基类定义的各个方法
pub trait Draw {
    fn draw(&self);
}
//模仿基类, 保存各个子类对象到vector中
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}
//依次调用各个子类的draw方法
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

//定义一个子类, 并实现基类的方法draw
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
    }
}
//定义另一个子类,并实现基类的方法draw
pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
    }
}