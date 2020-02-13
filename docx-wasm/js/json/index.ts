import { Styles } from "./styles";
import { DocumentJSON } from "./document";
import { AbstractNumberingJSON, NumberingJSON } from "./numbering";

export type DocxJSON = {
  contentType: {
    types: {
      [k: string]: string;
    };
  };
  rels: [string, string, string][];
  documentRels: {
    hasComments: boolean;
    hasNumberings: boolean;
  };
  docProps: {
    app: {};
    core: {
      config: {
        creator: string | null;
        description: string | null;
        language: string | null;
        lastModifiedBy: string | null;
        modified: string | null;
        revision: string | null;
        subject: string | null;
        title: string | null;
      };
    };
  };
  styles: Styles;
  document: DocumentJSON;
  comments: {
    comments: any[];
  };
  numberings: {
    abstractNums: AbstractNumberingJSON[];
    numberings: NumberingJSON[];
  };
  settings: {
    defaultTabStop: number;
    zoom: number;
  };
  fontTable: {};
};

export * from "./styles";
export * from "./border";
export * from "./document";
export * from "./paragraph";
export * from "./run";
export * from "./table";
export * from "./numbering";
