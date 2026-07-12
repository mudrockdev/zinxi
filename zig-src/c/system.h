/* Linux Specific information
 *
 *  DEPS: nodeps
 *  Created by 0l3d
 */
#ifndef SYSTEM_INFO_H
#define SYSTEM_INFO_H

typedef struct {
  // KERNEL INFO
  char sysname[64];
  char nodename[64];
  char release[32];
  char version[32];
  char machine[32];

  // COMPILER INFO
  char compiler_name[32];

  // CLOCKSOURCE
  char clc_current[32];
  char clc_avail[256];

  // PARAMETERS
  char params[4096];
} SystemInfo;

void kernel_info(SystemInfo *info);
void compiler_info(SystemInfo *info);
void clocksource(SystemInfo *info);
void parameters(SystemInfo *info);
void get_system_info(SystemInfo *info);
#endif
