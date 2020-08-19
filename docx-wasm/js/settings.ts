export class Settings {
  _docId: string | null = null;

  docId(id: string) {
    this._docId = id;
    return this;
  }
}
