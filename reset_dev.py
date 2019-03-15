import fcntl
import os

# USBDEVFS_RESET
fd = os.open("/dev/bus/usb/004/006", os.O_WRONLY)
fcntl.ioctl(fd, (ord('U') << (4 * 2)) | 20, 0)
