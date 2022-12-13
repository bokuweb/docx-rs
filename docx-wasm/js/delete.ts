import { Run } from "./run";

import * as wasm from "./pkg";

export class Delete {
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
    let del = wasm.createDelete(run);
    if (this._author) {
      del = del.author(this._author);
    }
    if (this._date) {
      del = del.date(this._date);
    }
    return del;
  }
}
