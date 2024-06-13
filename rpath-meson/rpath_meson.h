#pragma once
#if defined _WIN32 || defined __CYGWIN__
  #ifdef BUILDING_RPATH_MESON
    #define RPATH_MESON_PUBLIC __declspec(dllexport)
  #else
    #define RPATH_MESON_PUBLIC __declspec(dllimport)
  #endif
#else
  #ifdef BUILDING_RPATH_MESON
      #define RPATH_MESON_PUBLIC __attribute__ ((visibility ("default")))
  #else
      #define RPATH_MESON_PUBLIC
  #endif
#endif

RPATH_MESON_PUBLIC const char* hello();
