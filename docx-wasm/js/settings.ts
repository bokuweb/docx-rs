export type DocVar = {
  name: string;
  val: string;
};

export class Settings {
  _docId: string | null = null;
  _docVars: DocVar[] = [];

  docId(id: string) {
    this._docId = id;
    return this;
  }

  addDocVar(name: string, val: string) {
    this._docVars.push({ name, val });
    return this;
  }
}
