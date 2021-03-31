export type IndentJSON = {
  start: number | null;
  end: number | null;
  specialIndent: {
    type: "firstLine" | "hanging";
    val: number;
  } | null;
  startChars: number | null;
  // Read only
  hangingChars: number | null;
  firstLineChars: number | null;
};
