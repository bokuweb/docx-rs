export type LineSpacingJSON = {
    before: number | null;
    after: number | null;
    line: number | null;
    lineRule: {
        type: "atLeast" | "auto" | "exact";
        val: number;
    } | null;
};
