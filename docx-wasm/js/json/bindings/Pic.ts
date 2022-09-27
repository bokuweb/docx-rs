import type { DrawingPosition } from "./DrawingPosition";
import type { DrawingPositionType } from "./DrawingPositionType";
import type { RelativeFromHType } from "./RelativeFromHType";
import type { RelativeFromVType } from "./RelativeFromVType";

export interface Pic { id: string, image: Array<number>, size: [number, number], positionType: DrawingPositionType, simplePos: boolean, simplePosX: number, simplePosY: number, layoutInCell: boolean, relativeHeight: number, allowOverlap: boolean, positionH: DrawingPosition, positionV: DrawingPosition, relativeFromH: RelativeFromHType, relativeFromV: RelativeFromVType, distT: number, distB: number, distL: number, distR: number, rot: number, }