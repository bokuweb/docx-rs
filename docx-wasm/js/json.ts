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

export type RunChildJSON = TextJSON;

export type TextJSON = {
  type: "text";
  data: {
    preserveSpace: boolean;
    text: string;
  };
};

export type RunJSON = {
  type: "run";
  data: {
    runProperty: RunPropertyJSON;
    children: RunChildJSON[];
  };
};

export type ParagraphChildJSON = RunJSON;

export type NumberingPropertyJSON = {
  id: number;
  level: number;
};

export type ParagraphPropertyJSON = {
  runProperty: RunChildJSON;
  style: string | null;
  numberingProperty: NumberingPropertyJSON | null;
  alignment: "left" | "center" | "right" | "justified";
};

export type ParagraphJSON = {
  type: "paragraph";
  data: {
    property: ParagraphPropertyJSON;
    children: ParagraphChildJSON[];
  };
};
