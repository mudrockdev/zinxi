/* PCI Info
 *
 *  DEPS: pciacces / pciutil
 *  Created by 0l3d
 */
#include "pci.h"
#include <pci/pci.h>
#include <string.h>

int get_pci_info_full(PCIInfo *info) {
  memset(info, 0, sizeof(PCIInfo));
  struct pci_access *pacc = pci_alloc();
  struct pci_dev *dev;

  pci_init(pacc);
  pci_scan_bus(pacc);

  for (dev = pacc->devices; dev; dev = dev->next) {
    pci_fill_info(dev, PCI_FILL_IDENT | PCI_FILL_BASES | PCI_FILL_CLASS |
                           PCI_FILL_SUBSYS);

    if ((dev->device_class >> 8) == 0x03) {
      switch (dev->vendor_id) {
      case 0x10de: // NVIDIA
        pci_lookup_name(pacc, info->vendor, sizeof(info->vendor),
                        PCI_LOOKUP_DEVICE, dev->vendor_id, dev->device_id);
        pci_lookup_name(pacc, info->subvendor, sizeof(info->subvendor),
                        PCI_LOOKUP_VENDOR, dev->subsys_vendor_id);
        break;

        break;
      case 0x8086: // Intel
        pci_lookup_name(pacc, info->vendor, sizeof(info->vendor),
                        PCI_LOOKUP_DEVICE, dev->vendor_id, dev->device_id);
        pci_lookup_name(pacc, info->subvendor, sizeof(info->subvendor),
                        PCI_LOOKUP_VENDOR, dev->subsys_vendor_id);

        break;
      case 0x1002: // AMD / ATI
        pci_lookup_name(pacc, info->vendor, sizeof(info->vendor),
                        PCI_LOOKUP_DEVICE, dev->vendor_id, dev->device_id);
        pci_lookup_name(pacc, info->subvendor, sizeof(info->subvendor),
                        PCI_LOOKUP_VENDOR, dev->subsys_vendor_id);
        break;
      default:
        info->id = dev->vendor_id;
        break;
      }
    }
  }
  pci_cleanup(pacc);
  return 0;
}
