use label::labeled::Labeled;
use label::dclabel::DCLabel;
use label::runtime::*;

use std::collections::HashMap;
use std::thread;

fn main() {
    let mut map: Labeled<HashMap<String, Labeled<String, DCLabel>>, DCLabel> =
        unsafe { Labeled::labeled(HashMap::new(), DCLabel::public()) };
    {
        let m = map.unlabel_write();
        m.insert("amit".to_string(), unsafe { Labeled::labeled("levy".to_string(), DCLabel::new("amit", "amit")) } );
    }

    let mut map2 = map.clone();

    let t = thread::spawn(move || {
        {
            let m = map2.unlabel_write();
            if let Some(md) = m.get(&"amit".to_string()) {
                let value = md.unlabel_read();
                let value2 = value.clone();
                m.insert("bar".to_string(), unsafe { Labeled::labeled(value2, DCLabel::public()) });
            }
            let keys: Vec<&String> = m.keys().collect();
        }
        //println!("{:?}", map2.label());
    });

    t.join().unwrap();

    {
        println!("Current label: {:?}", current_label());
        let m = map.unlabel_write();
        m.insert("foo".to_string(), unsafe { Labeled::labeled("bar".to_string(), DCLabel::new("foo", "foo")) } );

        let keys: Vec<&String> = m.keys().collect();
        println!("Done {:?} {:?}", keys, current_label());
    }
}

