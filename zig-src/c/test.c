/*
 * LDFLAGS:
 * -lpci -lcpuid -lOpenCL
 * Example Test: gcc cpu.c gpu.c pci.c test.c -o test -lpci -lcpuid -lOpenCL
 *
 *  For Void Linux:
 *  OpenCL-Headers
 *  libcpuid-devel
 *  libpciacces-devel
 *
 * Created by 0l3d
 */
#include "cpu.h"
#include "gpu.h"
#include "pci.h"
#include "system.h"

#include <stdio.h>
int main(int argc, char **argv) {
  // gpu
  GPUInfo ginfo = {0};
  get_full_gpu_info(&ginfo);
  printf("GPUNAME: %s\n", ginfo.name);
  printf("GPU DRIVER VERSION: %s\n", ginfo.version);
  printf("GPU VENDOR: %s\n\n", ginfo.vendor);

  // cpu
  CPUInfo cinfo;
  get_cpu_info_full(&cinfo);
  printf("Model: %s\n", cinfo.brand);
  printf("Vendor: %s\n", cinfo.vendor);
  printf("Cores And Threads: %d | %d\n", cinfo.cores, cinfo.threads);
  printf("Stepping: %d\n", cinfo.stepping);
  printf("Model ID: %d\n", cinfo.model);
  printf("Family: %d\n\n", cinfo.family);
  get_cpu_free(&cinfo);

  // pci
  PCIInfo pci;
  get_pci_info_full(&pci);
  printf("PCI SUBSYSVENDOR: %s\n", pci.subvendor);

  // linux
  SystemInfo info;
  get_system_info(&info);

  printf("sysname: %s\n", info.sysname);
  printf("nodename: %s\n", info.nodename);
  printf("release: %s\n", info.release);
  printf("version: %s\n", info.version);
  printf("machine: %s\n", info.machine);

  printf("compiler_name: %s\n", info.compiler_name);

  printf("clc_current: %s\n", info.clc_current);
  printf("clc_avail: %s\n", info.clc_avail);

  printf("params: %s\n", info.params);

  return 0;
}
