include!("dom.rs");

fn main() {
    let c1 = text(String::from("hi"));
    let c2 = text(String::from("there"));
    let node = element(String::from("div"), AttrMap::new(), vec![c1, c2]);
    let node2 = element(String::from("div"), AttrMap::new(), vec![node]);
    let node4 = element(String::from("div"), AttrMap::new(), vec![]);
    let node3 = element(String::from("div"), AttrMap::new(), vec![node2, node4]);
    print(node3);
}

