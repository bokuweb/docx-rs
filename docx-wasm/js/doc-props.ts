export class DocProps {
  _createdAt: string | null = null;
  _updatedAt: string | null = null;

  createdAt(date: string) {
    this._createdAt = date;
    return this;
  }

  updatedAt(date: string) {
    this._updatedAt = date;
    return this;
  }
}
