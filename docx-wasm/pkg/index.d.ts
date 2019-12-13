/* tslint:disable */
/**
* @returns {Run} 
*/
export function createRun(): Run;
/**
* @param {string} id 
* @returns {Comment} 
*/
export function createComment(id: string): Comment;
/**
* @returns {TableCell} 
*/
export function createTableCell(): TableCell;
/**
* @returns {TableRow} 
*/
export function createTableRow(): TableRow;
/**
* @returns {Docx} 
*/
export function createDocx(): Docx;
/**
* @param {number} id 
* @param {number} start 
* @param {string} format 
* @param {string} text 
* @param {string} jc 
* @returns {Level} 
*/
export function createLevel(id: number, start: number, format: string, text: string, jc: string): Level;
/**
* @returns {Insert} 
*/
export function createInsert(): Insert;
/**
* @returns {Table} 
*/
export function createTable(): Table;
/**
* @param {number} id 
* @returns {Numbering} 
*/
export function createNumbering(id: number): Numbering;
/**
* @returns {Delete} 
*/
export function createDelete(): Delete;
/**
* @returns {Paragraph} 
*/
export function createParagraph(): Paragraph;
export enum TableAlignmentType {
  Center,
  Left,
  Right,
}
/**
*/
export enum SpecialIndentKind {
  FirstLine,
  Hanging,
}
/**
*/
export enum VMergeType {
  Continue,
  Restart,
}
/**
*/
export enum BreakType {
  Page,
  Column,
  TextWrapping,
}
/**
*/
export enum FontPitchType {
  Default,
  Fixed,
  Variable,
}
/**
*/
export enum WidthType {
  DXA,
  Auto,
}
/**
*/
export enum BorderType {
  None,
  Single,
  Thick,
  Double,
  Dotted,
  Dashed,
  DotDash,
  DotDotDash,
  Triple,
}
/**
*/
export enum AlignmentType {
  Center,
  Left,
  Right,
  Justified,
}
/**
*/
export enum StyleType {
  Paragraph,
  Character,
}
/**
*/
/**
*/
export class Comment {
  free(): void;
/**
* @param {string} author 
* @returns {Comment} 
*/
  author(author: string): Comment;
/**
* @param {string} date 
* @returns {Comment} 
*/
  date(date: string): Comment;
/**
* @param {Paragraph} p 
* @returns {Comment} 
*/
  paragraph(p: Paragraph): Comment;
/**
* @returns {string} 
*/
  id(): string;
}
/**
*/
export class Delete {
  free(): void;
}
/**
*/
export class Docx {
  free(): void;
/**
* @param {Paragraph} p 
* @returns {Docx} 
*/
  add_paragraph(p: Paragraph): Docx;
/**
* @param {Table} t 
* @returns {Docx} 
*/
  add_table(t: Table): Docx;
/**
* @param {Numbering} num 
* @returns {Docx} 
*/
  add_numbering(num: Numbering): Docx;
/**
* @returns {Uint8Array} 
*/
  build(): Uint8Array;
}
/**
*/
export class Insert {
  free(): void;
}
/**
*/
export class Level {
  free(): void;
/**
* @param {number} left 
* @param {number | undefined} special_indent_kind 
* @param {number | undefined} special_indent_size 
* @returns {Level} 
*/
  indent(left: number, special_indent_kind?: number, special_indent_size?: number): Level;
}
/**
*/
export class Numbering {
  free(): void;
/**
* @param {Level} level 
* @returns {Numbering} 
*/
  add_level(level: Level): Numbering;
}
/**
*/
export class Paragraph {
  free(): void;
/**
* @param {Run} run 
* @returns {Paragraph} 
*/
  add_run(run: Run): Paragraph;
/**
* @param {Insert} i 
* @returns {Paragraph} 
*/
  add_insert(i: Insert): Paragraph;
/**
* @param {Delete} d 
* @returns {Paragraph} 
*/
  add_delete(d: Delete): Paragraph;
/**
* @param {string} id 
* @param {string} name 
* @returns {Paragraph} 
*/
  add_bookmark_start(id: string, name: string): Paragraph;
/**
* @param {string} id 
* @returns {Paragraph} 
*/
  add_bookmark_end(id: string): Paragraph;
/**
* @param {Comment} comment 
* @returns {Paragraph} 
*/
  add_comment_start(comment: Comment): Paragraph;
/**
* @param {string} id 
* @returns {Paragraph} 
*/
  add_comment_end(id: string): Paragraph;
/**
* @param {number} alignment_type 
* @returns {Paragraph} 
*/
  align(alignment_type: number): Paragraph;
/**
* @param {string} style_id 
* @returns {Paragraph} 
*/
  style(style_id: string): Paragraph;
/**
* @param {number} left 
* @param {number | undefined} special_indent_kind 
* @param {number | undefined} special_indent_size 
* @returns {Paragraph} 
*/
  indent(left: number, special_indent_kind?: number, special_indent_size?: number): Paragraph;
/**
* @param {number} id 
* @param {number} level 
* @returns {Paragraph} 
*/
  numbering(id: number, level: number): Paragraph;
}
/**
*/
export class Run {
  free(): void;
/**
* @param {string} text 
* @returns {Run} 
*/
  add_text(text: string): Run;
/**
* @param {string} text 
* @returns {Run} 
*/
  add_delete_text(text: string): Run;
/**
* @returns {Run} 
*/
  add_tab(): Run;
/**
* @param {number} break_type 
* @returns {Run} 
*/
  add_break(break_type: number): Run;
/**
* @param {number} size 
* @returns {Run} 
*/
  size(size: number): Run;
/**
* @param {string} color 
* @returns {Run} 
*/
  color(color: string): Run;
/**
* @param {string} color 
* @returns {Run} 
*/
  highlight(color: string): Run;
/**
* @returns {Run} 
*/
  bold(): Run;
/**
* @returns {Run} 
*/
  italic(): Run;
/**
* @param {string} line_type 
* @returns {Run} 
*/
  underline(line_type: string): Run;
}
/**
*/
export class Table {
  free(): void;
/**
* @param {TableRow} row 
* @returns {Table} 
*/
  add_row(row: TableRow): Table;
/**
* @param {Uint32Array} grid 
* @returns {Table} 
*/
  set_grid(grid: Uint32Array): Table;
/**
* @param {number} v 
* @returns {Table} 
*/
  indent(v: number): Table;
/**
* @param {number} v 
* @returns {Table} 
*/
  align(v: number): Table;
}
/**
*/
export class TableCell {
  free(): void;
/**
* @param {Paragraph} p 
* @returns {TableCell} 
*/
  add_paragraph(p: Paragraph): TableCell;
/**
* @param {number} t 
* @returns {TableCell} 
*/
  vertical_merge(t: number): TableCell;
/**
* @param {number} v 
* @returns {TableCell} 
*/
  grid_span(v: number): TableCell;
}
/**
*/
export class TableRow {
  free(): void;
/**
* @param {TableCell} cell 
* @returns {TableRow} 
*/
  add_cell(cell: TableCell): TableRow;
}
