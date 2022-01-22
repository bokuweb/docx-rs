# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## docx-wasm@0.0.232 (24. January, 2022)

- Fixed a level reader to read paragraphProperty

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
