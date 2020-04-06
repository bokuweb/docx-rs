export type DrawingJSON = {
  type: "drawing";
  data: {
    children: DrawingChildJSON[];
  };
};

export type DrawingChildJSON = WpAnchorJSON;

export type WpAnchorJSON = {
  children: DrawingChildJSON[];
};

export type AGraphJSON = {
  children: AGraphChildJSON[];
};

export type AGraphChildJSON = WpAnchorJSON;

export type WpShapeJSON = {
  dataType: "WpShape";
  data: {
    children: DrawingChildJSON[];
  };
};
