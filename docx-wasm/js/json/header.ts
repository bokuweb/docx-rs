import { ParagraphJSON } from "./paragraph";
import { TableJSON } from "./table";

export type HeaderJSON = {
  children: (ParagraphJSON | TableJSON)[];
};

export type HeaderReferenceJSON = {
  headerType: string;
  id: string;
};
