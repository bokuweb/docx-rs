# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## docx-wasm@0.0.276-rc27 (26. Oct, 2022)

- improve js/json types

## docx-wasm@0.0.276-rc26 (14. Oct, 2022)

- Support text direction (#545)
- read `<sdt>`

## docx-rs@0.4.5 (14. Oct, 2022)

- Support text direction (#545)

## docx-rs@0.4.4 (27. Sep, 2022)

- Support rotate in pic

## docx-wasm@0.0.276-rc25 (21. Sep, 2022)

- [BugFix] Fixed a bug, hyperlink is broken with special characters.

## docx-wasm@0.0.276-rc20 (9. Sep, 2022)

- Support `sectionProperty` in pPr.

## docx-rs@0.4.3 (8. Sep, 2022)

- Support table style #531

## docx-wasm@0.0.276-rc12 (26. Aug, 2022)

- [BugFix] Fixed a bug, hRule is not read in reader.

## docx-wasm@0.0.276-rc11 (25. Aug, 2022)

- [Breaking] change table row `hightRule` to `auto` in reader.

## docx-wasm@0.0.276-rc10 (25. Aug, 2022)

- Support `adjustLineHeightInTable` in document setting

## docx-rs@0.4.2 (25. Aug, 2022)

- [Breaking] Hyperlink interface.
- Support hyperlink reader.
- [Breaking] Remove default `TableCellMargin`. Please specify if needed.
- Support table cell margins reader.
- Support nested table reader.
- Add png image converter
- Add `imageData` in `Shape` reader.
- [BUGFIX] Fixed a bug, it is not able to read jpeg from js.
- [BUGFIX] Fixed a bug, it is not able to write images.
- [Breaking] Add `id` and `path` to `images` output.
- [BUGFIX] Use default `numPr` if numId not found to avoid panic.
- [Breaking] Use `self.based_on` and `self.next` in `style` instead of `Normal`.
- [Breaking] Use `i32` for `PageMargin` props instead of unsigned.
- support `columns` in document.

## docx-wasm@0.0.275 (8. Aug, 2022)

- re enable Hyperlink interface.

## docx-wasm@0.0.273 (6. Jul, 2022)

- [Breaking] Hyperlink interface.
- Support hyperlink reader.

## docx-wasm@0.0.271, @0.0.272 (15. Jun, 2022)

- [Breaking] Remove default `TableCellMargin`. Please specify if needed.
- Support table cell margins reader.

## docx-wasm@0.0.269, @0.0.270 (14. Jun, 2022)

- Support nested table reader.

## docx-wasm@0.0.268 (13. Jun, 2022)

- Add png image converter

## docx-wasm@0.0.264 (7. Jun, 2022)

- Add `imageData` in `Shape` reader.

## docx-wasm@0.0.262 (27. May, 2022)

- [BUGFIX] Fixed a bug, it is not able to read jpeg from js.

## docx-wasm@0.0.261 (25. May, 2022)

- [BUGFIX] Fixed a bug, it is not able to write images.

## docx-wasm@0.0.260 (23. May, 2022)

- [Breaking] Add `id` and `path` to `images` output.

## docx-wasm@0.0.259 (20. May, 2022)

- [BUGFIX] Use default `numPr` if numId not found to avoid panic.

## docx-wasm@0.0.258 (19. May, 2022)

- [Breaking] Use `self.based_on` and `self.next` in `style` instead of `Normal`.

## docx-wasm@0.0.257 (18. May, 2022)

- [Breaking] Use `i32` for `PageMargin` props instead of unsigned.

## docx-rs@0.4.0, docx-wasm@0.0.252 (29. March, 2022)

- [Breaking] Change image size unit `px` to `emu`.
- [Breaking] fix `drawing` json types.

## docx-wasm@0.0.249 (25. March, 2022)

- Change `widthType` case to camelCase in JS.

## docx-wasm@0.0.248 (24. March, 2022)

- Change `lineRule` case to camelCase in JS.

## docx-wasm@0.0.247 (16. March, 2022)

- Add js interface for `rStyle`.

## docx-wasm@0.0.246, docx-rs@0.3.4 (16. March, 2022)

- Support `rStyle`.

## docx-wasm@0.0.245, docx-rs@0.3.3 (15. March, 2022)

- Fixed a author of `del` in `ins`.

## docx-wasm@0.0.242 (4. February, 2022)

- Support `ins` in table row property

## docx-wasm@0.0.235 (25. January, 2022)

- Support `del` in table row property

## docx-wasm@0.0.232, 0.0.233, 0.0.234 (24. January, 2022)

- Fixed a level reader to read paragraphProperty and runProperty

## docx-wasm@0.0.231 (19. January, 2022)

- Fixed a pPr reader.
## docx-wasm@0.0.229 (19. January, 2022)

- Fixed a del/ins type in runProperty

## docx-wasm@0.0.228 (18. January, 2022)

- Fixed a bug, numPr reader panic when id is none.

## docx-wasm@0.0.226, 0.0.227 (17. January, 2022)

- Add `paragraphPropertyChange` api for JS.

## docx-wasm@0.0.222, 223, 224, 225 (14. January, 2022)

- Fixed a typo `window_control` -> `widow_control`
- Fixed a numPr reader.

## docx-wasm@0.0.219, 220, 221 (6. January, 2022)

- [Internal]Support runFonts reader.

## docx-wasm@0.0.218 (6. January, 2022)

- Support minimum ToC API for JS (see https://github.com/bokuweb/docx-rs/pull/394).

## docx-wasm@0.0.217 (21. December, 2021)

- Fix `strike` API for JS.

## docx-wasm@0.0.216 (21. December, 2021)

- Support `strike` for run.

## docx-rs@0.3.2 (9. December, 2021)

- Add `DataBinding` API. See docx-core/examples/data_binding.rs
- Add minimum `ToC` API. See docx-core/examples/dirty_toc.rs

## docx-wasm@0.0.215 (26. November, 2021)

- Add before/afterLines to LineSpacingJSON.

## docx-wasm@0.0.214 (26. November, 2021)

- Fixed a missed pkg.

## docx-rs@0.3.1/docx-wasm@0.0.212 (25. November, 2021)

- Fix #327. Disable header if empty. (https://github.com/bokuweb/docx-rs/pull/369)

## docx-rs@0.3.0/docx-wasm@0.0.211 (25. November, 2021)

- Improve lineSpacing interface (https://github.com/bokuweb/docx-rs/pull/367)
