(window["webpackJsonp"] = window["webpackJsonp"] || []).push([[1],{

/***/ "./agb_web.js":
/*!********************!*\
  !*** ./agb_web.js ***!
  \********************/
/*! exports provided: __wbg_log_b6b99bda6f2d2a73, __wbg_error_c027e244c353f04e, __wbg_alert_8dc787c1a93118ac, __wbg_draw_8067c5a3a0ae46f3, load_rom, keydown, keyup, emulate, __wbindgen_throw */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_log_b6b99bda6f2d2a73\", function() { return __wbg_log_b6b99bda6f2d2a73; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_error_c027e244c353f04e\", function() { return __wbg_error_c027e244c353f04e; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_alert_8dc787c1a93118ac\", function() { return __wbg_alert_8dc787c1a93118ac; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_draw_8067c5a3a0ae46f3\", function() { return __wbg_draw_8067c5a3a0ae46f3; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"load_rom\", function() { return load_rom; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"keydown\", function() { return keydown; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"keyup\", function() { return keyup; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"emulate\", function() { return emulate; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_throw\", function() { return __wbindgen_throw; });\n/* harmony import */ var _agb_web_bg__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./agb_web_bg */ \"./agb_web_bg.wasm\");\n/* harmony import */ var _index__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./index */ \"./index.js\");\n/* tslint:disable */\n\n\n\nconst __wbg_log_b6b99bda6f2d2a73_target = console.log;\n\nconst TextDecoder = typeof self === 'object' && self.TextDecoder\n    ? self.TextDecoder\n    : __webpack_require__(/*! util */ \"../../../../../../../usr/lib/node_modules/webpack/node_modules/util/util.js\").TextDecoder;\n\nlet cachedDecoder = new TextDecoder('utf-8');\n\nlet cachegetUint8Memory = null;\nfunction getUint8Memory() {\n    if (cachegetUint8Memory === null || cachegetUint8Memory.buffer !== _agb_web_bg__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer) {\n        cachegetUint8Memory = new Uint8Array(_agb_web_bg__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachegetUint8Memory;\n}\n\nfunction getStringFromWasm(ptr, len) {\n    return cachedDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));\n}\n\nfunction __wbg_log_b6b99bda6f2d2a73(arg0, arg1) {\n    let varg0 = getStringFromWasm(arg0, arg1);\n    __wbg_log_b6b99bda6f2d2a73_target(varg0);\n}\n\nconst __wbg_error_c027e244c353f04e_target = console.error;\n\nfunction __wbg_error_c027e244c353f04e(arg0, arg1) {\n    let varg0 = getStringFromWasm(arg0, arg1);\n    __wbg_error_c027e244c353f04e_target(varg0);\n}\n\nfunction __wbg_alert_8dc787c1a93118ac(arg0, arg1) {\n    let varg0 = getStringFromWasm(arg0, arg1);\n    alert(varg0);\n}\n\nlet cachegetUint32Memory = null;\nfunction getUint32Memory() {\n    if (cachegetUint32Memory === null || cachegetUint32Memory.buffer !== _agb_web_bg__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer) {\n        cachegetUint32Memory = new Uint32Array(_agb_web_bg__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachegetUint32Memory;\n}\n\nfunction getArrayU32FromWasm(ptr, len) {\n    return getUint32Memory().subarray(ptr / 4, ptr / 4 + len);\n}\n\nfunction __wbg_draw_8067c5a3a0ae46f3(arg0, arg1, arg2, arg3) {\n    let varg2 = getArrayU32FromWasm(arg2, arg3);\n    Object(_index__WEBPACK_IMPORTED_MODULE_1__[\"draw\"])(arg0, arg1, varg2);\n}\n\nfunction passArray8ToWasm(arg) {\n    const ptr = _agb_web_bg__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_malloc\"](arg.length * 1);\n    getUint8Memory().set(arg, ptr / 1);\n    return [ptr, arg.length];\n}\n/**\n* Loads a rom + an optional save file.\n* This creates a new Gameboy object.\n* This can fail: if the rom has an invalid header an alert will be displayed  and an error message will be printed to the console\n* @param {Uint8Array} arg0\n* @returns {void}\n*/\nfunction load_rom(arg0) {\n    const [ptr0, len0] = passArray8ToWasm(arg0);\n    try {\n        return _agb_web_bg__WEBPACK_IMPORTED_MODULE_0__[\"load_rom\"](ptr0, len0);\n        \n    } finally {\n        _agb_web_bg__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_free\"](ptr0, len0 * 1);\n        \n    }\n    \n}\n\n/**\n* @param {number} arg0\n* @returns {void}\n*/\nfunction keydown(arg0) {\n    return _agb_web_bg__WEBPACK_IMPORTED_MODULE_0__[\"keydown\"](arg0);\n}\n\n/**\n* @param {number} arg0\n* @returns {void}\n*/\nfunction keyup(arg0) {\n    return _agb_web_bg__WEBPACK_IMPORTED_MODULE_0__[\"keyup\"](arg0);\n}\n\n/**\n* Emulate the gameboy for a specific number of milliseconds\n* @param {number} arg0\n* @returns {void}\n*/\nfunction emulate(arg0) {\n    return _agb_web_bg__WEBPACK_IMPORTED_MODULE_0__[\"emulate\"](arg0);\n}\n\nfunction __wbindgen_throw(ptr, len) {\n    throw new Error(getStringFromWasm(ptr, len));\n}\n\n\n\n//# sourceURL=webpack:///./agb_web.js?");

/***/ }),

/***/ "./agb_web_bg.wasm":
/*!*************************!*\
  !*** ./agb_web_bg.wasm ***!
  \*************************/
/*! exports provided: memory, __heap_base, __data_end, __rustc_debug_gdb_scripts_section__, load_rom, keydown, keyup, emulate, __wbindgen_malloc, __wbindgen_free */
/***/ (function(module, exports, __webpack_require__) {

eval("\"use strict\";\n// Instantiate WebAssembly module\nvar wasmExports = __webpack_require__.w[module.i];\n__webpack_require__.r(exports);\n// export exports from WebAssembly module\nfor(var name in wasmExports) if(name != \"__webpack_init__\") exports[name] = wasmExports[name];\n// exec imports from WebAssembly module (for esm order)\n/* harmony import */ var m0 = __webpack_require__(/*! ./agb_web */ \"./agb_web.js\");\n\n\n// exec wasm module\nwasmExports[\"__webpack_init__\"]()\n\n//# sourceURL=webpack:///./agb_web_bg.wasm?");

/***/ })

}]);