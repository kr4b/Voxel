#version 430

layout (location = 0) in vec2 texCoord;
layout (location = 1) in vec3 raw_dir;
layout (location = 2) in vec3 origin;

layout (location = 0) out vec4 color;

#define EPSILON 1e-4

layout(binding = 1) uniform usampler3D volume;
layout(binding = 2) uniform Specs {
  uint size;
} specs;

bool is_empty(uint n) {
  return (n & (1 << 15)) == 0;
}

vec3 get_color(uint n) {
  uint r = n & (31 << 10);
  uint g = n & (31 << 5);
  uint b = n & 31;
  return vec3(float(r) / float(31 << 10), float(g) / float(31 << 5), float(b) / 31.0);
}

vec2 intersect_ray_aabb(in vec3 origin, in vec3 dir, in vec3 AABBMin, in vec3 AABBMax) {
  const vec3 t1 = (AABBMin - origin) / dir;
  const vec3 t2 = (AABBMax - origin) / dir;
  
  const vec3 mins = min(t1, t2);
  const vec3 maxs = max(t1, t2);

  const float near = max(max(mins.x, mins.y), mins.z);
  const float far = min(min(maxs.x, maxs.y), maxs.z);

  return vec2(near, far);
}

uint intersect_ray(in vec3 origin, in vec3 dir, in vec3 AABBMin, in vec3 AABBMax, out vec3 itsct, out vec3 out_normal) {
  vec2 ts = intersect_ray_aabb(origin, dir, AABBMin - 1, AABBMax + 1);

  if (ts.x <= ts.y && ts.y >= 0.0) {
    if (ts.x < 0.0) {
      ts.x = 0.0;
    }

    const vec3 start_pos = origin + ts.x * dir;
    ivec3 pos = ivec3(floor(start_pos + EPSILON));
    const ivec3 istep = ivec3(sign(dir));
    const vec3 delta = 1.0 / abs(dir);
    const vec3 boundary = vec3(pos + max(istep, 0.0));

    vec3 current = (boundary - origin) / (dir + vec3(equal(dir, vec3(0.0))) * EPSILON);
    vec3 normal = vec3(0.0);
    uint voxel = 0;
    uint i = 0;

    do {
      voxel = texelFetch(volume, pos - ivec3(AABBMin), 0).r;
      itsct = vec3(pos);
      out_normal = normal;

      if (current.x < current.y && current.x < current.z) {
        current.x += delta.x;
        pos.x += istep.x;
        normal = vec3(-istep.x, 0.0, 0.0);
      } else if (current.y < current.z) {
        current.y += delta.y;
        pos.y += istep.y;
        normal = vec3(0.0, -istep.y, 0.0);
      } else {
        current.z += delta.z;
        pos.z += istep.z;
        normal = vec3(0.0, 0.0, -istep.z);
      }

      i += 1;
    } while (all(greaterThanEqual(pos, AABBMin - 1)) && all(lessThanEqual(pos, AABBMax + 1)) && is_empty(voxel) && i < specs.size * 3);

    return voxel;
  }

  return 0;
}

void main() {
  const vec3 AABBMin = vec3(-specs.size / 2);
  const vec3 AABBMax = vec3(specs.size / 2 - 1);
  const vec3 dir = normalize(raw_dir);

  vec3 itsct, normal;
  uint voxel = intersect_ray(origin, dir, AABBMin, AABBMax, itsct, normal);
  vec3 new_origin = itsct + normal;

  if (!is_empty(voxel)) {
    uint v = intersect_ray(new_origin, vec3(0.0, 1.0, 0.0), AABBMin, AABBMax, itsct, normal);
    color = vec4(get_color(voxel) * (is_empty(v) ? 1.0 : 0.5), 1.0);
  } else {
    color = vec4(0.0);
  }
}