/* CPU info
 *
 *  DEPS: cpuid
 *  Created by 0l3d
 */
#ifndef CPU_H
#define CPU_H

typedef struct {
  char *brand;
  char *vendor;
  int family;
  int model;
  int stepping;
  int cores;
  int threads;
} CPUInfo;

int get_cpu_info_full(CPUInfo *info);
void get_cpu_free(CPUInfo *info);

#endif
