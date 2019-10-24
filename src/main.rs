mod iters;

use iters::Octals;
use crate::iters::OctalPrinter;

#[derive(Clone, Debug)]
struct Node([u16; 8]);

const FRESH_NODE: Node = Node([0xFF; 8]);

pub struct ChildIter<'a>{
    node: &'a Node,
    nth: u8,
}

impl<'a> Iterator for ChildIter<'a> {
    type Item = (u8, u16);

    fn next(&mut self) -> Option<(u8, u16)> {
        while self.nth < 8 && 0xFF == self.node.0[self.nth as usize] {
            self.nth += 1;
        }
        if self.nth == 8 {
            return None;
        }
        self.nth += 1;
        Some((self.nth-1, self.node.0[self.nth as usize - 1]))
    }
}

#[derive(Clone, Debug)]
pub struct Trie {
    nodes: Vec<Node>,
}

impl Trie {
    pub fn new() -> Self {
        Self {
            nodes: vec![FRESH_NODE],
        }
    }

    pub fn navigate(&mut self, addr: u16, val: u8) -> u16 {
        let addr = addr as usize;
        let val = val as usize;
        let mut next = self.nodes[addr].0[val];
        if next == 0xFF {
            self.nodes.push(FRESH_NODE);
            next = (self.nodes.len() - 1) as u16;
            self.nodes[addr].0[val] = next;
        }
        next
    }

    fn get(&self, addr: u16) -> &Node {
        &self.nodes[addr as usize]
    }

    pub fn childcount(&self, addr: u16) -> u8 {
        self.get(addr).0.iter().filter(|&&v| v != 0xFF).count() as u8
    }

    pub fn children(&self, addr: u16) -> ChildIter<'_> {
        ChildIter {
            node: self.get(addr),
            nth: 0,
        }
    }

    pub fn debug_print(&self) {
        fn debug_print_impl(this: &Trie, addr: u16, prefix: &str, printer: OctalPrinter) {
            let count = this.childcount(addr);
            for (i, (val, child_addr)) in this.children(addr).enumerate() {
                let mut printer = printer.clone();
                print!("{}", prefix);
                println!(r#"\{}"#, printer.add(val).unwrap_or(b' ') as char);
                if i < count as usize - 1 {
                    debug_print_impl(this, child_addr, &format!("{}|", prefix), printer);
                } else {
                    debug_print_impl(this, child_addr, &format!("{} ", prefix), printer);
                }
            }
        }

        let printer = OctalPrinter::new();
        println!("root");
        debug_print_impl(self, 0, "", printer);
    }

    pub fn add_string(&mut self, string: &str) {
        let mut pointer = 0;
        for oct in Octals::new(string.bytes()) {
            pointer = self.navigate(pointer, oct);
        }
    }
}

fn main() {
    let mut trie = Trie::new();

    let input = "Lorem Ipsum is simply dummy text of the printing and typesetting industry.\
    Lorem Ipsum has been the industry's standard dummy text ever since the 1500s,\
    when an unknown printer took a galley of type and scrambled it to make a type specimen book.\
    It has survived not only five centuries, but also the leap into electronic typesetting,\
    remaining essentially unchanged.\
    It was popularised in the 1960s with the release of Letraset sheets containing Lorem Ipsum passages,\
    and more recently with desktop publishing software like Aldus PageMaker including versions of Lorem Ipsum.";

    let mut trie = Trie::new();
    trie.add_string("a");
    trie.debug_print();
    trie.add_string("aa");
    trie.debug_print();
    trie.add_string("au");
    trie.debug_print();
    trie.add_string("aaa");
    trie.debug_print();
    trie.add_string("aab");
    trie.debug_print();
    trie.add_string(input);
    trie.debug_print();
}
