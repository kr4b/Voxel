from os import system

command = "glslc.exe shaders/src/{0} -o shaders/spv/{0}.spv"
shaders = [
  "shader",
  "voxel"
]

for shader in shaders:
  if '.' in shader:
    system(command.format(shader))
  else:
    system(command.format(shader + ".vert"))
    system(command.format(shader + ".frag"))