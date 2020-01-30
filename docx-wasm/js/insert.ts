import { Run } from "./run";

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
}
