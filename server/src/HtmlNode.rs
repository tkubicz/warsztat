
use select::node::Node;
use select::document::Document;
use select::predicate::{Predicate, Class, Name, Attr};


#[derive(Debug)]
pub struct HtmlNode<'a> {
    element: Node<'a>,
}

impl<'a> HtmlNode<'a> {
    pub fn fromDocument(document: &'a Document) -> HtmlNode<'a> {

        let nodes: Vec<Node<'a>> = document.find(Name("html")).collect();

        let first = nodes.get(0);

        if let Some(first) = first {
            return HtmlNode {
                element: *first
            };
        }

        panic!("Expected html element");
    }

    pub fn fromNode(element: Node<'a>) -> HtmlNode<'a> {
        HtmlNode {
            element
        }
    }

    pub fn findByClass(&self, className: &str) -> Vec<HtmlNode<'a>> {                                    //TODO - zamienić Vec na iterator
        let list: Vec<HtmlNode<'a>> = self.element.find(Class(className)).map(HtmlNode::fromNode).collect();
        list
    }

    pub fn findByClassOne(&self, className: &str) -> HtmlNode<'a> {                                     //TODO - zamienić Vec na iterator
        let mut list: Vec<HtmlNode<'a>> = self.element.find(Class(className)).map(HtmlNode::fromNode).collect();
        
        let len = list.len();

        if len != 1 {
            panic!("Expectet one element {} -({:?})", className, len);
        }

        let element = list.pop();

        if let Some(element) = element {
            return element;
        }

        panic!("Expectet one element - find 0 elements");
    }

    pub fn findByName(&self, name: &str) -> Vec<HtmlNode<'a>> {                                     //TODO - zamienić Vec na iterator
        let list: Vec<HtmlNode<'a>> = self.element.find(Name(name)).map(HtmlNode::fromNode).collect();
        list
    }

    pub fn findByNameExpectOne(&self, name: &str) -> HtmlNode<'a> {                                     //TODO - zamienić Vec na iterator
        let mut list: Vec<HtmlNode<'a>> = self.element.find(Name(name)).map(HtmlNode::fromNode).collect();
        
        let len = list.len();

        if len != 1 {
            panic!("Expectet one element {} -({:?})", name, len);
        }

        let element = list.pop();

        if let Some(element) = element {
            return element;
        }

        panic!("Expectet one element - find 0 elements");
    }

    pub fn attr(&self, name: &str) -> Option<&'a str> {
        let result: Option<&'a str> = self.element.attr(name);
        result
    }

    pub fn hasClass(&self, className: &str) -> bool {
        self.element.is(Class(className))
    }

    pub fn text(&self) -> String {
        self.element.text()
    }
}
