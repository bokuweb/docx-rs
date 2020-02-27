export type IndentJSON = {
  start: number;
  end: number | null;
  specialIndent: {
    type: "firstLine" | "hanging";
    val: number;
  } | null;
};
