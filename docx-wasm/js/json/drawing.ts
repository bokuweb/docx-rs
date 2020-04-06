import { TextBoxContentJSON } from "./textbox-content";

export type DrawingJSON = {
  type: "drawing";
  data: {
    children: DrawingChildJSON[];
  };
};

export type DrawingChildJSON = WpAnchorJSON;

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
