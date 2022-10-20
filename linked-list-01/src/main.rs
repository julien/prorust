#[derive(Debug)]
struct ListItem<T> {
    data: Box<T>,
    next: Option<Box<ListItem<T>>>,
}

#[derive(Debug)]
struct SinglyLinkedList<T> {
    head: ListItem<T>,
}

impl<T> ListItem<T> {
    fn new(data: T) -> Self {
        Self {
            data: Box::new(data),
            next: None,
        }
    }

    fn next(&self) -> Option<&Self> {
        if let Some(next) = &self.next {
            Some(&*next)
        } else {
            None
        }
    }

    fn mut_tail(&mut self) -> &mut Self {
        if self.next.is_some() {
            self.next.as_mut().unwrap().mut_tail()
        } else {
            self
        }
    }

    fn data(&self) -> &T {
        self.data.as_ref()
    }
}

impl<T> SinglyLinkedList<T> {
    fn new(data: T) -> Self {
        Self {
            head: ListItem::new(data),
        }
    }

    fn append(&mut self, data: T) {
        let mut tail = self.head.mut_tail();
        tail.next = Some(Box::new(ListItem::new(data)));
    }

    fn head(&self) -> &ListItem<T> {
        &self.head
    }
}

fn main() {
    let mut list = SinglyLinkedList::<i32>::new(3);
    list.append(5);
    list.append(7);
    list.append(42);

    // let mut head = list.head();
    // println!("{:#?}", head.data());

    // while let Some(next) = head.next() {
    // println!("{:?}", next.data());
    // head = head.next().unwrap();
    // }
    let mut item = list.head();
    loop {
        println!("item: {}", item.data());
        if let Some(next) = item.next() {
            item = next;
        } else {
            break;
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        let mut list = SinglyLinkedList::new("head");
        list.append("middle");
        list.append("tail");

        let item = list.head();
        assert_eq!(*item.data(), String::from("head"));
    }
}
