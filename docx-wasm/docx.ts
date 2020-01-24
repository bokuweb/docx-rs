const rust = import("./pkg");

export class Docx {
  children: Paragraph[] = [];

  addParagraph(p: Paragraph) {
    this.children.push(p);
    return this;
  }

  build() {
    rust.then(m => {
      let docx = m.createDocx();
      this.children.forEach(child => {
        if (child instanceof Paragraph) {
          let p = m.createParagraph();
          child.children.forEach(c => {
            let r = m.createRun();
            c.children.forEach(a => {
              r = r.add_text(a.text);
            });
            p = p.add_run(r);
          });
          docx = docx.add_paragraph(p);
        }
      });
      console.log(m, docx.build());
      docx.free();
    });
  }

  /*
  addTable(t: Table) {
    this.children.push(t.intoJSON());
    return this;
  }
  
    pub fn add_numbering(mut self, num: Numbering) -> Docx {
        self.0.numberings = self.0.numberings.add_numbering(num.take());
        self
    }

    
    pub fn build(&mut self) -> Result<Vec<u8>, JsValue> {
        let buf = Vec::new();
        let mut cur = std::io::Cursor::new(buf);
        let res = self.0.build().pack(&mut cur);
        if res.is_err() {
            return Err(format!("{:?}", res).into());
        }
        Ok(cur.into_inner())
    }
    */
}

export class Paragraph {
  children: Run[] = [];

  addRun(run: Run) {
    this.children.push(run);
    return this;
  }
}

export class Run {
  children: Text[] = [];

  addText(text: Text) {
    this.children.push(text);
    return this;
  }
}

export class Text {
  text = "";

  constructor(text: string) {
    this.text = text;
  }
}

const d = new Docx().addParagraph(
  new Paragraph().addRun(new Run().addText(new Text("Hello")))
);

d.build();
d.build();
