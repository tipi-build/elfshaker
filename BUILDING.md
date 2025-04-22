Building with tipi 
===================
The build will use the `CMakeLists.txt` as per `.tipi/deps` use_cmakelists specification.

### macOS
- `tipi . -t macos-cxx17 --test all -- -DBUILD_TESTING=ON`

### Linux
- `tipi . -t linux-cxx17 --test all -- -DBUILD_TESTING=ON`

### Windows
  - `tipi . -t vs-16-2019-win64-cxx17 -C Release` or `tipi . -t vs-16-2019-win64-cxx17 -C RelWithDebInfo`
  - `C:\tipi.exe . -t  vs-16-2019-win64-cxx17 -C Release -vv  -- -A x64 -T host=x64 -DBUILD_TESTING=ON`