import { Run } from "./run";

import * as wasm from "./pkg";

export class Insert {
  run: Run;
  _author: string | null = null;
  _date: string | null = null;
  constructor(run: Run) {
    this.run = run;
  }

  author(author: string) {
    this._author = author;
    return this;
  }

  date(date: string) {
    this._date = date;
    return this;
  }

  build() {
    const run = this.run.build();
    let insert = wasm.createInsert(run);
    if (this._author) {
      insert = insert.author(this._author);
    }
    if (this._date) {
      insert = insert.date(this._date);
    }
    return insert;
  }
}
