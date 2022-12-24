project('13', 'cpp')


json = dependency('nlohmann_json', static: true)

incdir = include_directories('/home/panky/personal/AOC/2022')
src = ['main.cpp']
executable('out', sources : src, include_directories : incdir, dependencies: json, cpp_args: '-DDEMO -Wno-unused-function')
executable('out_full', sources : src, include_directories : incdir, dependencies: json, cpp_args: '-Wno-unused-function')
