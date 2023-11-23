#ifdef _WIN32
  // both win32 and win64 are defined here
  #include "../../../contrib/ports/win32/include/arch/cc.h"
#else
  #include "../../../contrib/ports/unix/port/include/arch/cc.h"
#endif
