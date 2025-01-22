import * as wasm from "./pkg";

import { Text } from "./text";
import { DeleteText } from "./delete-text";
import { Tab } from "./tab";
import { Break, BreakType } from "./break";
import { BorderType } from "./border";
import { Image } from "./image";
import { PositionalTab } from "./positional-tab";
import {
  createDefaultRunProperty,
  RunFonts,
  RunProperty,
  setRunProperty,
  VertAlignType,
} from "./run-property";
import { Tc } from "./tc";

export type RunChild =
  | Text
  | DeleteText
  | Tab
  | Break
  | Image
  | PositionalTab
  | Tc;

export class Run {
  children: RunChild[] = [];
  property: RunProperty;

  addText(text: string) {
    this.children.push(new Text(text));
    return this;
  }

  addImage(image: Image) {
    this.children.push(image);
    return this;
  }

  addDeleteText(text: string) {
    this.children.push(new DeleteText(text));
    return this;
  }

  addTab() {
    this.children.push(new Tab());
    return this;
  }

  addPositionalTab(ptab: PositionalTab) {
    this.children.push(ptab);
    return this;
  }

  addBreak(type: BreakType) {
    this.children.push(new Break(type));
    return this;
  }

  addTc(tc: Tc) {
    this.children.push(tc);
    return this;
  }

  style(style: string) {
    this.property ??= createDefaultRunProperty();
    this.property.style(style);
    return this;
  }

  size(size: number) {
    this.property ??= createDefaultRunProperty();
    this.property.size(size);
    return this;
  }

  color(color: string) {
    this.property ??= createDefaultRunProperty();
    this.property.color(color);
    return this;
  }

  highlight(color: string) {
    this.property ??= createDefaultRunProperty();
    this.property.highlight(color);
    return this;
  }

  vertAlign(vertAlign: VertAlignType) {
    this.property ??= createDefaultRunProperty();
    this.property.vertAlign(vertAlign);
    return this;
  }

  bold() {
    this.property ??= createDefaultRunProperty();
    this.property.bold();
    return this;
  }

  strike() {
    this.property ??= createDefaultRunProperty();
    this.property.strike();
    return this;
  }

  italic() {
    this.property ??= createDefaultRunProperty();
    this.property.italic();
    return this;
  }

  underline(type: string) {
    this.property ??= createDefaultRunProperty();
    this.property.underline(type);
    return this;
  }

  vanish() {
    this.property ??= createDefaultRunProperty();
    this.property.vanish();
    return this;
  }

  fonts(fonts: RunFonts) {
    this.property ??= createDefaultRunProperty();
    this.property.fonts(fonts);
    return this;
  }

  spacing(characterSpacing: number) {
    this.property ??= createDefaultRunProperty();
    this.property.spacing(characterSpacing);
    return this;
  }

  delete(author: string, date: string) {
    this.property ??= createDefaultRunProperty();
    this.property.delete(author, date);
    return this;
  }

  insert(author: string, date: string) {
    this.property ??= createDefaultRunProperty();
    this.property.delete(author, date);
    return this;
  }

  textBorder(type: BorderType, size: number, space: number, color: string) {
    this.property ??= createDefaultRunProperty();
    this.property.textBorder(type, size, space, color);
    return this;
  }

  build() {
    let run = wasm.createRun();
    this.children.forEach((child) => {
      if (child instanceof Text) {
        run = run.add_text(child.text);
      } else if (child instanceof DeleteText) {
        run = run.add_delete_text(child.text);
      } else if (child instanceof Tab) {
        run = run.add_tab();
      } else if (child instanceof PositionalTab) {
        run = run.add_ptab(child.buildWasmObject());
      } else if (child instanceof Break) {
        if (child.type === "column") {
          run = run.add_break(wasm.BreakType.Column);
        } else if (child.type === "page") {
          run = run.add_break(wasm.BreakType.Page);
        } else if (child.type === "textWrapping") {
          run = run.add_break(wasm.BreakType.TextWrapping);
        }
      } else if (child instanceof Image) {
        let pic = wasm.createPic(child.data);
        if (child.w != null && child.h != null) {
          pic = pic.size(child.w, child.h);
        }
        if (child._floating) {
          pic = pic.floating();
        }
        if (child._offsetX != null) {
          pic = pic.offset_x(child._offsetX);
        }
        if (child._offsetY != null) {
          pic = pic.offset_x(child._offsetY);
        }
        if (child.rot != null) {
          pic = pic.rotate(child.rot);
        }
        run = run.add_image(pic);
      } else if (child instanceof Tc) {
        run = run.add_tc(
          child._text,
          child._omitPageNumber,
          child._level,
          child._identifier
        );
      }
    });

    if (this.property) {
      run = setRunProperty(run, this.property) as wasm.Run;
    }

    return run;
  }
}
