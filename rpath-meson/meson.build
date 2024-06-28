project('rpath-meson', 'c',
  version : '0.1',
  default_options : ['warning_level=3', 'pkgconfig.relocatable=true'])

# These arguments are only used to build the shared library
# not the executables that use the library.
lib_args = ['-DBUILDING_RPATH_MESON']

shlib = shared_library('rpath_meson', 'rpath_meson.c',
  install : true,
  c_args : lib_args,
  gnu_symbol_visibility : 'hidden',
)

# Make this library usable as a Meson subproject.
rpath_meson_dep = declare_dependency(
  include_directories: include_directories('.'),
  link_with : shlib)

# Make this library usable from the system's
# package manager.
install_headers('rpath_meson.h', subdir : 'rpath_meson')

pkg_mod = import('pkgconfig')
pkg_mod.generate(
  name : 'rpath-meson',
  filebase : 'rpath_meson',
  description : 'Meson sample project.',
  subdirs : 'rpath_meson',
  extra_cflags: ['-Wl,-rpath,${prefix}', '-Wl,-rpath,${prefix}/lib'],
  libraries : shlib,
  version : '0.1',
)
