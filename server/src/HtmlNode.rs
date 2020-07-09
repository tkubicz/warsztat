
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

    pub fn findElementByClass(&self, className: &str) -> Vec<HtmlNode<'a>> {                                    //TODO - zamienić Vec na iterator
        let list: Vec<HtmlNode<'a>> = self.element.find(Class(className)).map(HtmlNode::fromNode).collect();
        list
    }

    pub fn findElementByName(&self, className: &str) -> Vec<HtmlNode<'a>> {                                     //TODO - zamienić Vec na iterator
        let list: Vec<HtmlNode<'a>> = self.element.find(Name(className)).map(HtmlNode::fromNode).collect();
        list
    }

    pub fn attr(&self, name: &str) -> Option<&'a str> {
        let result: Option<&'a str> = self.element.attr(name);
        result
    }

    pub fn hasClass(&self, className: &str) -> bool {
        self.element.is(Class(className))
    }
}
