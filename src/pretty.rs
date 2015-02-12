use std::fmt::{self, Debug};

use syntax::parse;

pub struct Pretty<D: Debug> {
    data: D
}

impl<D: Debug> Pretty<D> {
    pub fn new(data: D) -> Self {
        Pretty {
            data: data
        }
    }
}

impl<D: Debug> Debug for Pretty<D> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let src = format!("{:?}", self.data);
        // FIXME: special Handler
        let sess = parse::new_parse_sess();
        let tokens = parse::parse_tts_from_source_str(
            "<anon>".to_string(),
            src,
            vec![],
            &sess
        );
        println!("{:?}", tokens);
        Ok(())
    }
}
