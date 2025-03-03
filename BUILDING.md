Required to build:
==================


- tipi build with https://github.com/nxxm/nxxm-src/pull/1488
- `$env:CXXFLAGS="-MD"` and `$env:CFLAGS="-MD"`
- build with 
   - `tipi . -t vs-16-2019-win64-cxx17 -C Release` or `tipi . -t vs-16-2019-win64-cxx17 -C RelWithDebInfo`
   - `C:\tipi-for-arm-cmake-tipi-provider.exe . -t  vs-16-2019-win64-cxx17 -C Release -vv  -- -A x64 -T host=x64`