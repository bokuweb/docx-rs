export class Insert {
  _author: string | null = null;
  _date: string | null = null;

  author(author: string) {
    this._author = author;
    return this;
  }

  date(date: string) {
    this._date = date;
    return this;
  }
}
