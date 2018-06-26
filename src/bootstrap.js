function _rpack_bootstrap(modules, entry_point_id) {
  var installed_modules = {};

  function _rpack_require(module_id) {
    if (installed_modules[module_id]) {
      return installed_modules[module_id].exports;
    }

    installed_modules[module_id] = {
      id: module_id,
      loaded: true,
      exports: {}
    };
    var module = installed_modules[module_id];
    modules[module_id].call(module, module, module.exports, _rpack_require);
    modules[module_id].loaded = true;
    return module.exports;
  }

  return _rpack_require(entry_point_id);
}
