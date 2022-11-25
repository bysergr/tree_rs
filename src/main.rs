use tree::Node;

fn main() {
    /*
    let mut z = Node::new("ハロー・ワールド");

    z.insert("Hallo");
    z.insert("Bye");
    z.insert("Good");
    z.insert("Nice");
    z.insert("Apple");
    */

    let mut z = Node::new(&55);

    z.insert(&34);
    z.insert(&32);
    z.insert(&74);
    z.insert(&30);
    z.insert(&54);
    z.insert(&54);

    z.show()
}
