/* GPUInfo
 *
 *  DEPS: opencl
 *  Created by 0l3d
 */
#ifndef GPU_H
#define GPU_H
#define CL_TARGET_OPENCL_VERSION 300
#include <CL/cl.h>
typedef struct {
  char *name;
  char *version;
  char *vendor;
  char *alternate;
} GPUInfo;

int get_gpu_info_from_platform(cl_platform_id platform, GPUInfo *info);
int get_full_gpu_info(GPUInfo *info);

#endif
