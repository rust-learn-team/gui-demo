/// 一个简单的GUI框架探索
/// 1：一个GUI页面拥有哪些属性？
/// 渲染 [Render], 继承 [Inherit],
/// Inherit<Render<T>> 可以继承的可渲染的对象，同时拥有 Render 和 Inherit 属性
/// Render<T> 必须是大小确定的 [SizedBox] 可以获得长宽属性。

struct OptionBox {
    // 渲染的宽度和高度
    width: Option<i32>,
    height: Option<i32>,
    // 渲染的左边
    x: Option<i32>,
    y: Option<i32>,
}

trait SizedBox {
    fn get_width(&self) -> i32;
    fn get_height(&self) -> i32;
}

trait Renderable {
    fn renader(&self);
}

struct Button {
    option_box: OptionBox,
    on_click: fn(),
    child: Box<dyn Renderable>,
}
impl SizedBox for Button {
    fn get_height(&self) -> i32 {
        self.option_box.height.expect("组件没有高度")
    }
    fn get_width(&self) -> i32 {
        self.option_box.width.expect("组件没有宽度")
    }
}

impl Renderable for Button {
    fn renader(&self) {
        // todo!()
    }
}

fn main() {
    //
}

// trait RenderTrait {
//     fn init(&self);

//     fn render(&self);

//     fn destroy(&self);
// }

// trait LifeCycle {
//     fn update(&self);
// }

// ///
// /// 可以点击的对象
// ///
// trait PressableTrait {
//     fn on_press(&self);
// }

// ///
// /// 可继承对象 需要满足哪些条件？
// ///
// /// [RenderTrait] and [LifeCycle]
// struct InheritedObject {
//     render_object: Box<dyn RenderTrait>,
// }

// impl RenderTrait for InheritedObject {
//     fn init(&self) {
//         todo!()
//     }

//     fn render(&self) {
//         todo!()
//     }

//     fn destroy(&self) {
//         todo!()
//     }
// }

// struct Button<'a> {
//     // key: String, // key 用于区分不同的对象。
//     name: &'a str,
//     child: InheritedObject,
// }

// impl PressableTrait for Button<'_> {
//     fn on_press(&self) {
//         println!("> Press {} button", self.name);
//     }
// }

// // impl RenderTrait for Button<'_> {
// //     fn render(&self) {
// //         print!("[Button]");
// //     }

// //     fn init(&self) {
// //         println!("Button {} init", self.name);
// //         // TODO: 绑定监控点击
// //         // self.on_press();

// //         // child
// //         self.child.init();
// //     }

// //     fn destroy(&self) {
// //         println!("Button {} destroy", self.name);
// //         // TODO: Destroy this button.

// //         // Destroy this child
// //         self.child.destroy();
// //     }
// // }

// struct Inherited<T: RenderTrait + LifeCycle>(T);

// impl InheritedObject for Button<'_> {}

// struct Empty {}
// impl RenderTrait for Empty {
//     fn init(&self) {
//         todo!()
//     }

//     fn render(&self) {
//         todo!()
//     }

//     fn destroy(&self) {
//         todo!()
//     }
// }

// fn main() {
//     // println!("Hello, world!");
//     let a = Button {
//         name: "adf",
//         child: Button {
//             name: "2",
//             child: Empty {},
//         },
//     };
// }
