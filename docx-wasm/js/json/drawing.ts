import { TextBoxContentJSON } from "./textbox-content";
import { Pic as InnerPic } from "./bindings/Pic";
import { ParagraphJSON, TableJSON } from "..";

export interface Pic extends Omit<InnerPic, "image"> {
  image: string;
}

export type DrawingJSON = {
  type: "drawing";
  data:
    | {
        type: "pic";
        data: Pic;
      }
    | {
        type: "textBox";
        data: {
          children: (ParagraphJSON | TableJSON)[];
        };
      };
};

export type WpAnchorJSON = {
  type: "anchor";
  data: {
    children: AGraphicJSON[];
  };
};

export type AGraphicJSON = {
  children: AGraphChildJSON[];
};

export type AGraphChildJSON = AGraphicDataJSON;

export type AGraphicDataJSON = {
  dataType: "wpShape";
  children: WpsShapeJSON[];
};

export type WpsShapeJSON = {
  type: "shape";
  data: {
    children: WpsShapeChildJSON[];
  };
};

export type WpsShapeChildJSON = WpsTextBoxJSON;

export type WpsTextBoxJSON = {
  type: "textbox";
  data: {
    children: TextBoxContentJSON[];
  };
};
