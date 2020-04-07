import { ParagraphJSON } from "./paragraph";
import { TableJSON } from "./table";

export type TextBoxContentChildJSON = ParagraphJSON | TableJSON;

export type TextBoxContentJSON = {
  children: TextBoxContentChildJSON[];
};
