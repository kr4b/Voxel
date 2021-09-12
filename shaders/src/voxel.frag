#version 430

layout (location = 0) in vec2 texCoord;
layout (location = 1) in vec3 raw_dir;
layout (location = 2) in vec3 in_origin;

layout (location = 0) out vec4 color;

#define EPSILON 1e-4
#define light_dir vec3(0.3, 1.0, 0.1)

layout(binding = 1) uniform usampler3D volume;
layout(binding = 2) uniform Specs {
  uint size;
} specs;

bool is_empty(uvec4 c) {
  return (c.a >> 4) == 0;
}

vec3 get_color(uvec4 c) {
  return vec3(c.r / 255.0, c.g / 255.0, c.b / 255.0);
}

float get_transparency(uvec4 c) {
  return float(c.a >> 4) / 15.0;
}

float get_reflectivity(uvec4 c) {
  return float(c.a & 15) / 15.0;
}

vec2 intersect_ray_aabb(in vec3 origin, in vec3 dir, in vec3 aabb_min, in vec3 aabb_max) {
  const vec3 t1 = (aabb_min - origin) / dir;
  const vec3 t2 = (aabb_max - origin) / dir;
  
  const vec3 mins = min(t1, t2);
  const vec3 maxs = max(t1, t2);

  const float near = max(max(mins.x, mins.y), mins.z);
  const float far = min(min(maxs.x, maxs.y), maxs.z);

  return vec2(near, far);
}

uvec4 intersect_ray(in vec3 origin, in vec3 dir, in vec3 aabb_min, in vec3 aabb_max, in uvec4 skip_voxel, out vec3 itsct, out vec3 out_normal) {
  vec2 ts = intersect_ray_aabb(origin, dir, aabb_min - 1, aabb_max + 1);

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
    uvec4 voxel = uvec4(0);
    uint i = 0;
    bool skip = !is_empty(skip_voxel);
    bool first_skip = false;

    do {
      voxel = texelFetch(volume, pos - ivec3(aabb_min), 0);
      out_normal = normal;
      if (normal.x != 0.0) {
        itsct = start_pos + ((pos.x - start_pos.x - istep.x) / dir.x) * dir;
      } else if (normal.y != 0.0) {
        itsct = start_pos + ((pos.y - start_pos.y - istep.y) / dir.y) * dir;
      } else {
        itsct = start_pos + ((pos.z - start_pos.z - istep.z) / dir.z) * dir;
      }

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
      
      if (voxel == skip_voxel) {
        first_skip = true;
      } else if (first_skip) {
        skip = false;
      }

      i += 1;
    } while ((skip && voxel == skip_voxel) || (all(greaterThanEqual(pos, aabb_min - 1)) && all(lessThanEqual(pos, aabb_max + 1)) && is_empty(voxel) && i < specs.size * 3));

    return voxel;
  }

  return uvec4(0);
}

void main() {
  const vec3 aabb_min = vec3(-int(specs.size / 2));
  const vec3 aabb_max = vec3(int(specs.size / 2) - 1);
  vec3 dir = normalize(raw_dir);
  vec3 origin = in_origin;

  vec4 final_color = vec4(0.0);
  uvec4 skip_voxel = uvec4(0);

  while (true) {
    vec3 itsct, normal;
    const uvec4 voxel = intersect_ray(origin, dir, aabb_min, aabb_max, skip_voxel, itsct, normal);
    skip_voxel = voxel;

    vec3 shade_itsct, shade_normal;
    const uvec4 shade_voxel = intersect_ray(itsct, light_dir, aabb_min, aabb_max, uvec4(0), shade_itsct, shade_normal);
    const float shade = 1.0 - 0.5 * get_transparency(shade_voxel);
    const float transparency = get_transparency(voxel);
    const float reflectivity = get_reflectivity(voxel);

    final_color += vec4(get_color(voxel) * shade * transparency, transparency) * (1.0 - final_color.a) * (1.0 - reflectivity); 

    if (final_color.a >= 1.0 - EPSILON || (transparency >= 1.0 - EPSILON && reflectivity <= EPSILON) || is_empty(voxel)) {
      break;
    }
    
    if (reflectivity > EPSILON) {
      dir = dir - 2.0 * dot(dir, normal) * normal;
    }

    origin = itsct;
  }

  color = final_color;
}