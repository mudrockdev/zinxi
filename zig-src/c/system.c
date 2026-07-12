/* Linux Specific information
 *
 *  DEPS: nodeps
 *  Created by 0l3d
 */
#include "system.h"
#include <stdio.h>
#include <string.h>
#include <sys/utsname.h>
#include <unistd.h>

void kernel_info(SystemInfo *info) {
  struct utsname unamebuf;
  if (uname(&unamebuf) == -1) {
    perror("uname syscall failed");
    return;
  }

  strcpy(info->sysname, unamebuf.sysname);
  strcpy(info->nodename, unamebuf.nodename);
  strcpy(info->release, unamebuf.release);
  strcpy(info->version, unamebuf.version);
  strcpy(info->machine, unamebuf.machine);
}

void compiler_info(SystemInfo *info) {
  char compiler_name[64];

  if (readlink("/usr/bin/cc", compiler_name, 64) == -1) {
    perror("readlink failed");
    return;
  }

  strcpy(info->compiler_name, compiler_name);
}

void clocksource(SystemInfo *info) {
  FILE *current = fopen(
      "/sys/devices/system/clocksource/clocksource0/current_clocksource", "r");
  if (current == NULL) {
    perror("fopen failed (clocksource current)");
    return;
    {
    }
  }
  char currentbuf[32];
  fgets(currentbuf, 32, current);
  currentbuf[strcspn(currentbuf, "\n")] = '\0';
  strcpy(info->clc_current, currentbuf);
  FILE *avail = fopen(
      "/sys/devices/system/clocksource/clocksource0/available_clocksource",
      "r");
  if (avail == NULL) {
    perror("fopen failed (clocksource avail)");
    return;
  }
  char availbuf[256];
  fgets(availbuf, 256, avail);
  info->clc_avail[0] = '\0';
  char *available = strtok(availbuf, " ");
  while (available != NULL) {
    available[strcspn(available, "\n")] = '\0';
    if (strcmp(available, currentbuf) != 0) {
      strcat(info->clc_avail, available);
      strcat(info->clc_avail, " ");
    }
    available = strtok(NULL, " ");
  }

  fclose(current);
  fclose(avail);
}

void parameters(SystemInfo *info) {
  FILE *param = fopen("/proc/cmdline", "r");
  if (param == NULL) {
    perror("param fopen failed");
    return;
  }
  fgets(info->params, 4096, param);
  fclose(param);
}

void get_system_info(SystemInfo *info) {
  kernel_info(info);
  compiler_info(info);
  clocksource(info);
  parameters(info);
}
