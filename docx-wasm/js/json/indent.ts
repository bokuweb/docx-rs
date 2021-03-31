export type IndentJSON = {
  start: number | null;
  end: number | null;
  specialIndent: {
    type: "firstLine" | "hanging";
    val: number;
  } | null;
  startChars: number | null;
  hangingChars: number | null;
  firstLineChars: number | null;
};
