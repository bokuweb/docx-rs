export class DocProps {
  _createdAt: string | null = null;
  _updatedAt: string | null = null;
  _customProperties: { [name: string]: string } = {};

  createdAt(date: string) {
    this._createdAt = date;
    return this;
  }

  updatedAt(date: string) {
    this._updatedAt = date;
    return this;
  }

  customProperty(name: string, item: string) {
    this._customProperties[name] = item;
    return this;
  }
}
