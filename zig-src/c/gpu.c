/* GPUInfo
 *
 *  DEPS: opencl
 *  Created by 0l3d
 */
#include "gpu.h"
#include <stdlib.h>
#include <string.h>

int get_gpu_info_from_platform(cl_platform_id platform, GPUInfo *info) {
  cl_uint num_devices = 0;
  clGetDeviceIDs(platform, CL_DEVICE_TYPE_GPU, 0, NULL, &num_devices);
  if (num_devices == 0) {
    return -1;
  }
  cl_device_id *devices = malloc(sizeof(cl_device_id) * num_devices);
  clGetDeviceIDs(platform, CL_DEVICE_TYPE_GPU, num_devices, devices, NULL);

  for (cl_uint j = 0; j < num_devices; j++) {
    char driver_version[128];
    clGetDeviceInfo(devices[j], CL_DRIVER_VERSION, sizeof(driver_version),
                    driver_version, NULL);
    char device_name[128];
    clGetDeviceInfo(devices[j], CL_DEVICE_NAME, sizeof(device_name),
                    device_name, NULL);
    char device_vendor[128];
    clGetDeviceInfo(devices[j], CL_DEVICE_VENDOR, sizeof(device_vendor),
                    device_vendor, NULL);
    free(info->name);
    free(info->version);
    free(info->vendor);

    info->name = malloc(strlen(device_name) + 1);
    strcpy(info->name, device_name);
    info->version = malloc(strlen(driver_version) + 1);
    strcpy(info->version, driver_version);
    info->vendor = malloc(strlen(device_vendor) + 1);
    strcpy(info->vendor, device_vendor);
  }

  free(devices);
  return 0;
}

int get_full_gpu_info(GPUInfo *info) {
  cl_uint num_platforms = 0;
  cl_int ret = clGetPlatformIDs(0, NULL, &num_platforms);
  if (ret != CL_SUCCESS || num_platforms == 0) {
    return -2;
  }
  cl_platform_id *platforms = malloc(sizeof(cl_platform_id) * num_platforms);
  clGetPlatformIDs(num_platforms, platforms, NULL);
  for (cl_uint i = 0; i < num_platforms; i++) {
    char platform_vendor[128];
    clGetPlatformInfo(platforms[i], CL_PLATFORM_VENDOR, sizeof(platform_vendor),
                      platform_vendor, NULL);

    if (strstr(platform_vendor, "NVIDIA") != NULL ||
        strstr(platform_vendor, "Intel") != NULL ||
        strstr(platform_vendor, "AMD") != NULL) {
      if (get_gpu_info_from_platform(platforms[i], info) == -1) {
        return -1;
      };
    }
  }

  free(platforms);
  return 0;
}
