
let wasm;

/**
* @returns {Docx}
*/
export function createDocx() {
    const ret = wasm.createDocx();
    return Docx.__wrap(ret);
}

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
    return instance.ptr;
}

let cachegetInt32Memory = null;
function getInt32Memory() {
    if (cachegetInt32Memory === null || cachegetInt32Memory.buffer !== wasm.memory.buffer) {
        cachegetInt32Memory = new Int32Array(wasm.memory.buffer);
    }
    return cachegetInt32Memory;
}

let cachegetUint8Memory = null;
function getUint8Memory() {
    if (cachegetUint8Memory === null || cachegetUint8Memory.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory;
}

function getArrayU8FromWasm(ptr, len) {
    return getUint8Memory().subarray(ptr / 1, ptr / 1 + len);
}
/**
* @returns {Delete}
*/
export function createDelete() {
    const ret = wasm.createDelete();
    return Delete.__wrap(ret);
}

/**
* @returns {Insert}
*/
export function createInsert() {
    const ret = wasm.createInsert();
    return Insert.__wrap(ret);
}

/**
* @param {number} id
* @returns {Numbering}
*/
export function createNumbering(id) {
    const ret = wasm.createNumbering(id);
    return Numbering.__wrap(ret);
}

/**
* @returns {Table}
*/
export function createTable() {
    const ret = wasm.createTable();
    return Table.__wrap(ret);
}

/**
* @returns {TableRow}
*/
export function createTableRow() {
    const ret = wasm.createTableRow();
    return TableRow.__wrap(ret);
}

/**
* @returns {Paragraph}
*/
export function createParagraph() {
    const ret = wasm.createParagraph();
    return Paragraph.__wrap(ret);
}

let WASM_VECTOR_LEN = 0;

let cachedTextEncoder = new TextEncoder('utf-8');

const encodeString = (typeof cachedTextEncoder.encodeInto === 'function'
    ? function (arg, view) {
    return cachedTextEncoder.encodeInto(arg, view);
}
    : function (arg, view) {
    const buf = cachedTextEncoder.encode(arg);
    view.set(buf);
    return {
        read: arg.length,
        written: buf.length
    };
});

function passStringToWasm(arg) {

    let len = arg.length;
    let ptr = wasm.__wbindgen_malloc(len);

    const mem = getUint8Memory();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }

    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = wasm.__wbindgen_realloc(ptr, len, len = offset + arg.length * 3);
        const view = getUint8Memory().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);

        offset += ret.written;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

function isLikeNone(x) {
    return x === undefined || x === null;
}
/**
* @param {string} id
* @returns {Comment}
*/
export function createComment(id) {
    const ret = wasm.createComment(passStringToWasm(id), WASM_VECTOR_LEN);
    return Comment.__wrap(ret);
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

function getStringFromWasm(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));
}
/**
* @returns {Run}
*/
export function createRun() {
    const ret = wasm.createRun();
    return Run.__wrap(ret);
}

/**
* @param {number} id
* @param {number} start
* @param {string} format
* @param {string} text
* @param {string} jc
* @returns {Level}
*/
export function createLevel(id, start, format, text, jc) {
    const ret = wasm.createLevel(id, start, passStringToWasm(format), WASM_VECTOR_LEN, passStringToWasm(text), WASM_VECTOR_LEN, passStringToWasm(jc), WASM_VECTOR_LEN);
    return Level.__wrap(ret);
}

/**
* @returns {TableCell}
*/
export function createTableCell() {
    const ret = wasm.createTableCell();
    return TableCell.__wrap(ret);
}

const heap = new Array(32);

heap.fill(undefined);

heap.push(undefined, null, true, false);

let heap_next = heap.length;

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

function getObject(idx) { return heap[idx]; }

function dropObject(idx) {
    if (idx < 36) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}
/**
*/
export const SpecialIndentKind = Object.freeze({ FirstLine:0,Hanging:1, });
/**
*/
export const StyleType = Object.freeze({ Paragraph:0,Character:1, });
/**
*/
export const VMergeType = Object.freeze({ Continue:0,Restart:1, });
/**
*/
export const WidthType = Object.freeze({ DXA:0,Auto:1, });
/**
*/
export const BorderType = Object.freeze({ None:0,Single:1,Thick:2,Double:3,Dotted:4,Dashed:5,DotDash:6,DotDotDash:7,Triple:8, });
/**
*/
export const AlignmentType = Object.freeze({ Center:0,Left:1,Right:2,Justified:3, });
/**
*/
export const FontPitchType = Object.freeze({ Default:0,Fixed:1,Variable:2, });
/**
*/
export const BreakType = Object.freeze({ Page:0,Column:1,TextWrapping:2, });
/**
*/
export const TableAlignmentType = Object.freeze({ Center:0,Left:1,Right:2, });
/**
*/
export class Comment {

    static __wrap(ptr) {
        const obj = Object.create(Comment.prototype);
        obj.ptr = ptr;

        return obj;
    }

    free() {
        const ptr = this.ptr;
        this.ptr = 0;

        wasm.__wbg_comment_free(ptr);
    }
    /**
    * @param {string} author
    * @returns {Comment}
    */
    author(author) {
        const ptr = this.ptr;
        this.ptr = 0;
        const ret = wasm.comment_author(ptr, passStringToWasm(author), WASM_VECTOR_LEN);
        return Comment.__wrap(ret);
    }
    /**
    * @param {string} date
    * @returns {Comment}
    */
    date(date) {
        const ptr = this.ptr;
        this.ptr = 0;
        const ret = wasm.comment_date(ptr, passStringToWasm(date), WASM_VECTOR_LEN);
        return Comment.__wrap(ret);
    }
    /**
    * @param {Paragraph} p
    * @returns {Comment}
    */
    paragraph(p) {
        const ptr = this.ptr;
        this.ptr = 0;
        _assertClass(p, Paragraph);
        const ptr0 = p.ptr;
        p.ptr = 0;
        const ret = wasm.comment_paragraph(ptr, ptr0);
        return Comment.__wrap(ret);
    }
    /**
    * @returns {string}
    */
    id() {
        const retptr = 8;
        const ret = wasm.comment_id(retptr, this.ptr);
        const memi32 = getInt32Memory();
        const v0 = getStringFromWasm(memi32[retptr / 4 + 0], memi32[retptr / 4 + 1]).slice();
        wasm.__wbindgen_free(memi32[retptr / 4 + 0], memi32[retptr / 4 + 1] * 1);
        return v0;
    }
}
/**
*/
export class Delete {

    static __wrap(ptr) {
        const obj = Object.create(Delete.prototype);
        obj.ptr = ptr;

        return obj;
    }

    free() {
        const ptr = this.ptr;
        this.ptr = 0;

        wasm.__wbg_delete_free(ptr);
    }
}
/**
*/
export class Docx {

    static __wrap(ptr) {
        const obj = Object.create(Docx.prototype);
        obj.ptr = ptr;

        return obj;
    }

    free() {
        const ptr = this.ptr;
        this.ptr = 0;

        wasm.__wbg_docx_free(ptr);
    }
    /**
    * @param {Paragraph} p
    * @returns {Docx}
    */
    add_paragraph(p) {
        const ptr = this.ptr;
        this.ptr = 0;
        _assertClass(p, Paragraph);
        const ptr0 = p.ptr;
        p.ptr = 0;
        const ret = wasm.docx_add_paragraph(ptr, ptr0);
        return Docx.__wrap(ret);
    }
    /**
    * @param {Table} t
    * @returns {Docx}
    */
    add_table(t) {
        const ptr = this.ptr;
        this.ptr = 0;
        _assertClass(t, Table);
        const ptr0 = t.ptr;
        t.ptr = 0;
        const ret = wasm.docx_add_table(ptr, ptr0);
        return Docx.__wrap(ret);
    }
    /**
    * @param {Numbering} num
    * @returns {Docx}
    */
    add_numbering(num) {
        const ptr = this.ptr;
        this.ptr = 0;
        _assertClass(num, Numbering);
        const ptr0 = num.ptr;
        num.ptr = 0;
        const ret = wasm.docx_add_numbering(ptr, ptr0);
        return Docx.__wrap(ret);
    }
    /**
    * @returns {Uint8Array}
    */
    build() {
        const retptr = 8;
        const ret = wasm.docx_build(retptr, this.ptr);
        const memi32 = getInt32Memory();
        const v0 = getArrayU8FromWasm(memi32[retptr / 4 + 0], memi32[retptr / 4 + 1]).slice();
        wasm.__wbindgen_free(memi32[retptr / 4 + 0], memi32[retptr / 4 + 1] * 1);
        return v0;
    }
}
/**
*/
export class Insert {

    static __wrap(ptr) {
        const obj = Object.create(Insert.prototype);
        obj.ptr = ptr;

        return obj;
    }

    free() {
        const ptr = this.ptr;
        this.ptr = 0;

        wasm.__wbg_insert_free(ptr);
    }
}
/**
*/
export class Level {

    static __wrap(ptr) {
        const obj = Object.create(Level.prototype);
        obj.ptr = ptr;

        return obj;
    }

    free() {
        const ptr = this.ptr;
        this.ptr = 0;

        wasm.__wbg_level_free(ptr);
    }
    /**
    * @param {number} left
    * @param {number | undefined} special_indent_kind
    * @param {number | undefined} special_indent_size
    * @returns {Level}
    */
    indent(left, special_indent_kind, special_indent_size) {
        const ptr = this.ptr;
        this.ptr = 0;
        const ret = wasm.level_indent(ptr, left, isLikeNone(special_indent_kind) ? 2 : special_indent_kind, !isLikeNone(special_indent_size), isLikeNone(special_indent_size) ? 0 : special_indent_size);
        return Level.__wrap(ret);
    }
}
/**
*/
export class Numbering {

    static __wrap(ptr) {
        const obj = Object.create(Numbering.prototype);
        obj.ptr = ptr;

        return obj;
    }

    free() {
        const ptr = this.ptr;
        this.ptr = 0;

        wasm.__wbg_numbering_free(ptr);
    }
    /**
    * @param {Level} level
    * @returns {Numbering}
    */
    add_level(level) {
        const ptr = this.ptr;
        this.ptr = 0;
        _assertClass(level, Level);
        const ptr0 = level.ptr;
        level.ptr = 0;
        const ret = wasm.numbering_add_level(ptr, ptr0);
        return Numbering.__wrap(ret);
    }
}
/**
*/
export class Paragraph {

    static __wrap(ptr) {
        const obj = Object.create(Paragraph.prototype);
        obj.ptr = ptr;

        return obj;
    }

    free() {
        const ptr = this.ptr;
        this.ptr = 0;

        wasm.__wbg_paragraph_free(ptr);
    }
    /**
    * @param {Run} run
    * @returns {Paragraph}
    */
    add_run(run) {
        const ptr = this.ptr;
        this.ptr = 0;
        _assertClass(run, Run);
        const ptr0 = run.ptr;
        run.ptr = 0;
        const ret = wasm.paragraph_add_run(ptr, ptr0);
        return Paragraph.__wrap(ret);
    }
    /**
    * @param {Insert} i
    * @returns {Paragraph}
    */
    add_insert(i) {
        const ptr = this.ptr;
        this.ptr = 0;
        _assertClass(i, Insert);
        const ptr0 = i.ptr;
        i.ptr = 0;
        const ret = wasm.paragraph_add_insert(ptr, ptr0);
        return Paragraph.__wrap(ret);
    }
    /**
    * @param {Delete} d
    * @returns {Paragraph}
    */
    add_delete(d) {
        const ptr = this.ptr;
        this.ptr = 0;
        _assertClass(d, Delete);
        const ptr0 = d.ptr;
        d.ptr = 0;
        const ret = wasm.paragraph_add_delete(ptr, ptr0);
        return Paragraph.__wrap(ret);
    }
    /**
    * @param {string} id
    * @param {string} name
    * @returns {Paragraph}
    */
    add_bookmark_start(id, name) {
        const ptr = this.ptr;
        this.ptr = 0;
        const ret = wasm.paragraph_add_bookmark_start(ptr, passStringToWasm(id), WASM_VECTOR_LEN, passStringToWasm(name), WASM_VECTOR_LEN);
        return Paragraph.__wrap(ret);
    }
    /**
    * @param {string} id
    * @returns {Paragraph}
    */
    add_bookmark_end(id) {
        const ptr = this.ptr;
        this.ptr = 0;
        const ret = wasm.paragraph_add_bookmark_end(ptr, passStringToWasm(id), WASM_VECTOR_LEN);
        return Paragraph.__wrap(ret);
    }
    /**
    * @param {Comment} comment
    * @returns {Paragraph}
    */
    add_comment_start(comment) {
        const ptr = this.ptr;
        this.ptr = 0;
        _assertClass(comment, Comment);
        const ptr0 = comment.ptr;
        comment.ptr = 0;
        const ret = wasm.paragraph_add_comment_start(ptr, ptr0);
        return Paragraph.__wrap(ret);
    }
    /**
    * @param {string} id
    * @returns {Paragraph}
    */
    add_comment_end(id) {
        const ptr = this.ptr;
        this.ptr = 0;
        const ret = wasm.paragraph_add_comment_end(ptr, passStringToWasm(id), WASM_VECTOR_LEN);
        return Paragraph.__wrap(ret);
    }
    /**
    * @param {number} alignment_type
    * @returns {Paragraph}
    */
    align(alignment_type) {
        const ptr = this.ptr;
        this.ptr = 0;
        const ret = wasm.paragraph_align(ptr, alignment_type);
        return Paragraph.__wrap(ret);
    }
    /**
    * @param {string} style_id
    * @returns {Paragraph}
    */
    style(style_id) {
        const ptr = this.ptr;
        this.ptr = 0;
        const ret = wasm.paragraph_style(ptr, passStringToWasm(style_id), WASM_VECTOR_LEN);
        return Paragraph.__wrap(ret);
    }
    /**
    * @param {number} left
    * @param {number | undefined} special_indent_kind
    * @param {number | undefined} special_indent_size
    * @returns {Paragraph}
    */
    indent(left, special_indent_kind, special_indent_size) {
        const ptr = this.ptr;
        this.ptr = 0;
        const ret = wasm.paragraph_indent(ptr, left, isLikeNone(special_indent_kind) ? 2 : special_indent_kind, !isLikeNone(special_indent_size), isLikeNone(special_indent_size) ? 0 : special_indent_size);
        return Paragraph.__wrap(ret);
    }
    /**
    * @param {number} id
    * @param {number} level
    * @returns {Paragraph}
    */
    numbering(id, level) {
        const ptr = this.ptr;
        this.ptr = 0;
        const ret = wasm.paragraph_numbering(ptr, id, level);
        return Paragraph.__wrap(ret);
    }
}
/**
*/
export class Run {

    static __wrap(ptr) {
        const obj = Object.create(Run.prototype);
        obj.ptr = ptr;

        return obj;
    }

    free() {
        const ptr = this.ptr;
        this.ptr = 0;

        wasm.__wbg_run_free(ptr);
    }
    /**
    * @param {string} text
    * @returns {Run}
    */
    add_text(text) {
        const ptr = this.ptr;
        this.ptr = 0;
        const ret = wasm.run_add_text(ptr, passStringToWasm(text), WASM_VECTOR_LEN);
        return Run.__wrap(ret);
    }
    /**
    * @param {string} text
    * @returns {Run}
    */
    add_delete_text(text) {
        const ptr = this.ptr;
        this.ptr = 0;
        const ret = wasm.run_add_delete_text(ptr, passStringToWasm(text), WASM_VECTOR_LEN);
        return Run.__wrap(ret);
    }
    /**
    * @returns {Run}
    */
    add_tab() {
        const ptr = this.ptr;
        this.ptr = 0;
        const ret = wasm.run_add_tab(ptr);
        return Run.__wrap(ret);
    }
    /**
    * @param {number} break_type
    * @returns {Run}
    */
    add_break(break_type) {
        const ptr = this.ptr;
        this.ptr = 0;
        const ret = wasm.run_add_break(ptr, break_type);
        return Run.__wrap(ret);
    }
    /**
    * @param {number} size
    * @returns {Run}
    */
    size(size) {
        const ptr = this.ptr;
        this.ptr = 0;
        const ret = wasm.run_size(ptr, size);
        return Run.__wrap(ret);
    }
    /**
    * @param {string} color
    * @returns {Run}
    */
    color(color) {
        const ptr = this.ptr;
        this.ptr = 0;
        const ret = wasm.run_color(ptr, passStringToWasm(color), WASM_VECTOR_LEN);
        return Run.__wrap(ret);
    }
    /**
    * @param {string} color
    * @returns {Run}
    */
    highlight(color) {
        const ptr = this.ptr;
        this.ptr = 0;
        const ret = wasm.run_highlight(ptr, passStringToWasm(color), WASM_VECTOR_LEN);
        return Run.__wrap(ret);
    }
    /**
    * @returns {Run}
    */
    bold() {
        const ptr = this.ptr;
        this.ptr = 0;
        const ret = wasm.run_bold(ptr);
        return Run.__wrap(ret);
    }
    /**
    * @returns {Run}
    */
    italic() {
        const ptr = this.ptr;
        this.ptr = 0;
        const ret = wasm.run_italic(ptr);
        return Run.__wrap(ret);
    }
    /**
    * @param {string} line_type
    * @returns {Run}
    */
    underline(line_type) {
        const ptr = this.ptr;
        this.ptr = 0;
        const ret = wasm.run_underline(ptr, passStringToWasm(line_type), WASM_VECTOR_LEN);
        return Run.__wrap(ret);
    }
}
/**
*/
export class Table {

    static __wrap(ptr) {
        const obj = Object.create(Table.prototype);
        obj.ptr = ptr;

        return obj;
    }

    free() {
        const ptr = this.ptr;
        this.ptr = 0;

        wasm.__wbg_table_free(ptr);
    }
    /**
    * @param {TableRow} row
    * @returns {Table}
    */
    add_row(row) {
        const ptr = this.ptr;
        this.ptr = 0;
        _assertClass(row, TableRow);
        const ptr0 = row.ptr;
        row.ptr = 0;
        const ret = wasm.table_add_row(ptr, ptr0);
        return Table.__wrap(ret);
    }
}
/**
*/
export class TableCell {

    static __wrap(ptr) {
        const obj = Object.create(TableCell.prototype);
        obj.ptr = ptr;

        return obj;
    }

    free() {
        const ptr = this.ptr;
        this.ptr = 0;

        wasm.__wbg_tablecell_free(ptr);
    }
    /**
    * @param {Paragraph} p
    * @returns {TableCell}
    */
    add_paragraph(p) {
        const ptr = this.ptr;
        this.ptr = 0;
        _assertClass(p, Paragraph);
        const ptr0 = p.ptr;
        p.ptr = 0;
        const ret = wasm.tablecell_add_paragraph(ptr, ptr0);
        return TableCell.__wrap(ret);
    }
    /**
    * @param {number} t
    * @returns {TableCell}
    */
    vertical_merge(t) {
        const ptr = this.ptr;
        this.ptr = 0;
        const ret = wasm.tablecell_vertical_merge(ptr, t);
        return TableCell.__wrap(ret);
    }
    /**
    * @param {number} v
    * @returns {TableCell}
    */
    grid_span(v) {
        const ptr = this.ptr;
        this.ptr = 0;
        const ret = wasm.tablecell_grid_span(ptr, v);
        return TableCell.__wrap(ret);
    }
}
/**
*/
export class TableRow {

    static __wrap(ptr) {
        const obj = Object.create(TableRow.prototype);
        obj.ptr = ptr;

        return obj;
    }

    free() {
        const ptr = this.ptr;
        this.ptr = 0;

        wasm.__wbg_tablerow_free(ptr);
    }
    /**
    * @param {TableCell} cell
    * @returns {TableRow}
    */
    add_cell(cell) {
        const ptr = this.ptr;
        this.ptr = 0;
        _assertClass(cell, TableCell);
        const ptr0 = cell.ptr;
        cell.ptr = 0;
        const ret = wasm.tablerow_add_cell(ptr, ptr0);
        return TableRow.__wrap(ret);
    }
}

function init(module) {
    if (typeof module === 'undefined') {
        module = import.meta.url.replace(/\.js$/, '_bg.wasm');
    }
    let result;
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbindgen_string_new = function(arg0, arg1) {
        const ret = getStringFromWasm(arg0, arg1);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm(arg0, arg1));
    };
    imports.wbg.__wbindgen_rethrow = function(arg0) {
        throw takeObject(arg0);
    };

    if ((typeof URL === 'function' && module instanceof URL) || typeof module === 'string' || (typeof Request === 'function' && module instanceof Request)) {

        const response = fetch(module);
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            result = WebAssembly.instantiateStreaming(response, imports)
            .catch(e => {
                return response
                .then(r => {
                    if (r.headers.get('Content-Type') != 'application/wasm') {
                        console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);
                        return r.arrayBuffer();
                    } else {
                        throw e;
                    }
                })
                .then(bytes => WebAssembly.instantiate(bytes, imports));
            });
        } else {
            result = response
            .then(r => r.arrayBuffer())
            .then(bytes => WebAssembly.instantiate(bytes, imports));
        }
    } else {

        result = WebAssembly.instantiate(module, imports)
        .then(result => {
            if (result instanceof WebAssembly.Instance) {
                return { instance: result, module };
            } else {
                return result;
            }
        });
    }
    return result.then(({instance, module}) => {
        wasm = instance.exports;
        init.__wbindgen_wasm_module = module;

        return wasm;
    });
}

export default init;

