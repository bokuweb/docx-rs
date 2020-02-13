import { Styles } from "./styles";

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
  document: {
    children: any[];
  };
  comments: {
    comments: any[];
  };
  numberings: {};
  settings: {
    defaultTabStop: number;
    zoom: number;
  };
  fontTable: {};
};
