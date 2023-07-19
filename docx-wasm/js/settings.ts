export type DocVar = {
  name: string;
  val: string;
};

export type CharacterSpacingValues =
  | "doNotCompress"
  | "compressPunctuation"
  | "compressPunctuationAndJapaneseKana";

export class Settings {
  _docId: string | null = null;
  _docVars: DocVar[] = [];
  _defaultTabStop = 840;
  _adjustLineHeightInTable = false;
  _characterSpacingControl: CharacterSpacingValues | null = null;

  docId(id: string) {
    this._docId = id;
    return this;
  }

  defaultTabStop(stop: number) {
    this._defaultTabStop = stop;
    return this;
  }

  addDocVar(name: string, val: string) {
    this._docVars.push({ name, val });
    return this;
  }

  adjustLineHeightInTable() {
    this._adjustLineHeightInTable = true;
    return this;
  }

  characterSpacingControl(t: CharacterSpacingValues) {
    this._characterSpacingControl = t;
    return this;
  }
}
