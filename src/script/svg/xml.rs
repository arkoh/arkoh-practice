pub struct Element {
    name: ~str,
    attributes: ~[Attribute],
    children: ~[Element]
}

#[deriving(Clone)]
pub struct Attribute {
    name: ~str,
    value: ~str
}

pub enum State{
    OutSideTag,
    InSideTag,
    InSideAttrName,
    ParsingTagName,
    ParsingAttrValue,
    TagInSide,
    CloseTag,
}

pub enum Msg {
    EOF,
}

pub struct SVGParser {
    priv line: uint,
    priv col: uint,
    priv data: ~str,
    priv buf: ~str,
    priv name: ~str,
    priv attrName: ~str,
    priv attributes: ~[Attribute],
    priv st: State,
    priv level: uint
}


impl SVGParser {

    pub fn new() -> SVGParser {
        let xml_parser = SVGParser {
            line: 1,
            col: 0,
            data: ~"",
            buf: ~"",
            name: ~"",
            attrName: ~"",
            attributes: ~[],
            st: OutSideTag,
            level: 0
        };

        xml_parser
    }


    pub fn push_str(&mut self, buf: &str) {
        self.data.push_str(buf);
    }

    pub fn parse(&mut self) -> Msg {
        while self.data.len() > 0 {
            let c = self.data.shift_char();
            if c == '\n' {
                self.line += 1u;
                self.col = 0u;
            } else {
                self.col += 1u;
            }

            self.parse_character(c);
        }
        
        EOF
    }

    fn parse_character(&mut self, c: char) {
        match self.st {
            OutSideTag => self.outside_tag(c),
            InSideTag => self.inside_tag(c),
            InSideAttrName => self.inside_attr_name(c),
            ParsingTagName => self.parsing_tag_name(c),
            ParsingAttrValue => self.parsing_attr_value(c),
            TagInSide => self.tag_in_side(c),
            CloseTag => self.close_tag(c),
        }
    }

    fn outside_tag(&mut self, c: char) {
        match c {
            '<' => { self.level += 1; self.st = InSideTag; },
            _ => {},
        }
    }

    fn inside_tag(&mut self, c: char) {
        match c {
            '?' => {},
            '/' => { self.st = CloseTag;},
            _ => { self.st = ParsingTagName; },
        }
    }

    fn parsing_tag_name(&mut self, c: char) {
        match c {
            ' ' =>{ self.st = InSideAttrName;  },
            '>' =>{ self.st = TagInSide; }
            _=>{ }, 
        }
    }

    fn inside_attr_name(&mut self, c: char) {
        match c {
            '/' => { self.st = CloseTag; },
            '>' => { self.st = TagInSide; },
            '"' => { self.st = ParsingAttrValue; },
            _=>{ }, 
        }
    }

    fn close_tag(&mut self, c: char) {
        match c {
            '>' => { if(self.level == 0 ) {  self.st = OutSideTag;  } else { self.level -= 1; self.st = TagInSide; }
                 },
            _ => { },
        }
    }

    fn parsing_attr_value(&mut self, c: char ) {
        match c {
            '"' => { self.st = InSideAttrName; },
            _ =>{ },
        }
    }

    fn tag_in_side(&mut self, c: char) {
        match c {
            '<' => { self.st=InSideTag; },
            ' ' | '\n' => {},
             _ => {},
        }
    }
}

fn main () {

    let mut p = SVGParser::new();
    p.push_str(
        "
         <svg width=\"12cm\" height=\"4cm\" viewBox=\"0 0 1200 400\"
         xmlns=\"http://www.w3.org/2000/svg\" version=\"1.1\">

        <desc>Example rect01 - rectangle with sharp corners</desc>
        <rect x=\"1\" y=\"1\" width=\"1198\" height=\"398\"
        fill=\"none\" stroke=\"blue\" stroke-width=\"2\"/>

        <rect x=\"400\" y=\"100\" width=\"400\" height=\"200\"
        fill=\"yellow\" stroke=\"navy\" stroke-width=\"10\"  />

        </svg>"
    );
    
    p.parse();

}
