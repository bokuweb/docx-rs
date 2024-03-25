import type { BorderType } from "./BorderType";
import type { TableCellBorderPosition } from "./TableCellBorderPosition";

export interface TableCellBorder { borderType: BorderType, size: number, color: string, position: TableCellBorderPosition, space: number, }