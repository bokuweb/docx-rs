export type LineSpacingJSON = {
  before?: number | null;
  after?: number | null;
  beforeLines?: number | null;
  afterLines?: number | null;
  line?: number | null;
  lineRule?: "atLeast" | "auto" | "exact" | null;
};
