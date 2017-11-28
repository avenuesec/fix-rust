// Not generated. 
// These types are the backbone for the message parsing and generation

use std::{slice};
// use std::collections::{HashSet};
use frame::{FieldVal};
// use chrono::prelude::*;

// For message parsing

pub struct FixConsumer<'a> {
    iter : slice::Iter<'a, FieldVal<'a>>,
//    consumed_tags : HashSet<u32>,
    peeked : Option<Option<&'a FieldVal<'a>>>,
}

impl<'a> FixConsumer<'a> {

    pub fn new( flds: &'a Vec<FieldVal> ) -> FixConsumer<'a> {
        let iter = flds.iter();
        FixConsumer { iter: iter, peeked: None }
    }

    #[inline]
    pub fn peek(&mut self) -> Option<&'a FieldVal<'a>> {
        if self.peeked.is_none() {
            loop  {
                let n = self.iter.next();
                match n {
                    Some(_) => {
                        // is it being ignored?
                        // if self.consumed_tags.contains(&v.id) { continue; }
                        self.peeked = Some(n);
                        break;
                    },
                    None => { 
                        self.peeked = Some(None); 
                        break; 
                    }
                }
            }
            // self.peeked = Some(self.iter.next());
        }

        match self.peeked {
            Some(Some(ref v)) => {
                Some(v)
            },
            Some(None) => None,
            None => unreachable!(),
        }
    }

    #[inline]
    pub fn next(&mut self) -> Option<&'a FieldVal<'a>> {
        match self.peeked.take() {
            Some(a) => a, // assumes it was vetted by peek()
            None => {
                // returns something out of ignore list
                while let Some(n) = self.iter.next() {
                    // if self.consumed_tags.contains(&n.id) { continue; }
                    return Some(n)
                };
                None
            }
        }
    }

    // indicates that these tags should be ignored from this point forward
    // pub fn consume(&mut self, tags:&[u32])  {
    //     for tag in tags {
    //         self.consumed_tags.insert(*tag);
    //     }
    // }
}

// For message generation

// use std::string::{ToString};
// use std::str::{FromStr};

// const FIX_BEGIN : &str= "FIX.4.4";
// const FIX_MESSAGE_DELIMITER: char = '\x01';
// const FIX_EQUAL: char = '=';


#[cfg(test)]
mod tests {
  	use super::*;

    #[test]
    fn fixconsumer_peek_and_next() {
        let flds = vec![ FieldVal{id: 1, val: "" }, FieldVal{id: 2, val: "" } ];
        let mut consumer = FixConsumer::new(&flds);

        let peeked_1 = consumer.peek();
        let peeked_2 = consumer.peek();
        assert_eq!(peeked_1, peeked_2);
        
        let next_1 = consumer.next();
        assert_eq!(next_1, peeked_2);
        let next_2 = consumer.next();
        assert_ne!(next_1, next_2);

        let peeked_3 = consumer.peek();
        assert!(peeked_3.is_none());
        let next_3 = consumer.next();
        assert!(next_3.is_none());
    }

    // #[test]
    // fn fixconsumer_peek_and_next_with_preconsume() {
    //     let flds = vec![ FieldVal{id: 1, val: "" }, 
    //                      FieldVal{id: 2, val: "" }, 
    //                      FieldVal{id: 3, val: "" },
    //                      FieldVal{id: 4, val: "" } ];
    //     let mut consumer = FixConsumer::new(&flds);

    //     let peeked_1 = consumer.peek();
    //     let peeked_2 = consumer.peek();
    //     assert_eq!(peeked_1, peeked_2);
        
    //     let next_1 = consumer.next();
    //     assert_eq!(next_1, peeked_2);
        
    //     consumer.consume(&[2, 3]);

    //     let peeked_3 = consumer.peek();
    //     assert!(peeked_3.is_some());
    //     assert!(peeked_3.map( |fld| fld.id == 4).unwrap() ); // 4th element, 2-3 skipped

    //     let next_3 = consumer.next();
    //     assert!(next_3.is_some());
    //     assert!(next_3.map( |fld| fld.id == 4).unwrap() ); // 4th element, 2-3 skipped
    // }
}
