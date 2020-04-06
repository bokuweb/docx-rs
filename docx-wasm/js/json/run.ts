import { DrawingJSON } from "./drawing";

export type RunPropertyJSON = {
  sz: number | null;
  szCs: number | null;
  color: string | null;
  highlight: string | null;
  underline: string | null;
  bold: boolean | null;
  boldCs: boolean | null;
  italic: boolean | null;
  italicCs: boolean | null;
  vanish: boolean | null;
};

export type RunChildJSON =
  | TextJSON
  | DeleteTextJSON
  | TabJSON
  | BreakJSON
  | DrawingJSON;

export type TextJSON = {
  type: "text";
  data: {
    preserveSpace: boolean;
    text: string;
  };
};

export type DeleteTextJSON = {
  type: "deleteText";
  data: {
    preserveSpace: boolean;
    text: string;
  };
};

export type TabJSON = {
  type: "tab";
};

export type BreakJSON = {
  type: "break";
  data: {
    breakType: "page" | "column" | "textWrapping";
  };
};

export type RunJSON = {
  type: "run";
  data: {
    runProperty: RunPropertyJSON;
    children: RunChildJSON[];
  };
};
