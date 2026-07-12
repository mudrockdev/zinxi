/* PCI Info
 *
 *  DEPS: pciacces / pciutil
 *  Created by 0l3d
 */
#ifndef PCI_H
#define PCI_H

typedef struct {
  char subvendor[256];
  char vendor[256];
  unsigned long id;
} PCIInfo;

int get_pci_info_full(PCIInfo *info);

#endif
