/* CPU info
 *
 *  DEPS: cpuid
 *  Created by 0l3d
 */
#include "cpu.h"
#include <libcpuid/libcpuid.h>
#include <stdlib.h>
#include <string.h>

int get_cpu_info_full(CPUInfo *info) {
  struct cpu_raw_data_t raw;
  struct cpu_id_t data;

  if (cpuid_get_raw_data(&raw) < 0) {
    return -1;
  }

  if (cpu_identify(&raw, &data) < 0) {
    return -2;
  }

  info->brand = malloc(strlen(data.brand_str) + 1);
  strcpy(info->brand, data.brand_str);
  info->vendor = malloc(strlen(data.vendor_str) + 1);
  strcpy(info->vendor, data.vendor_str);
  info->cores = data.num_cores;
  info->family = data.x86.family;
  info->model = data.x86.model;
  info->stepping = data.x86.stepping;
  info->threads = data.num_logical_cpus;
  return 0;
}

void get_cpu_free(CPUInfo *info) {
  free(info->vendor);
  free(info->brand);
}
