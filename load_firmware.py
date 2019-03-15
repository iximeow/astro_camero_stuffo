import sys
import os
import subprocess

def get_path(dev_id):
    if len(dev_id) != 9:
        raise Exception("Invalid device id: {}. Expected a string 9 characters long (ex: aaaa:bbbb)".format(dev_id))

    for line in subprocess.check_output("lsusb").split('\n'):
        if dev_id in line:
            parts = line.split(' ')
            bus = parts[1]
            device = parts[3][:-1]
            path = "/dev/bus/usb/{}/{}".format(bus, device)
#            path = "/sys/bus/usb/devices/{}-{}".format(bus.replace('0', ''), device.replace('0', ''))
            print("Found {} at {}".format(dev_id, path))
            return path

    return None

def fix_perms(camera_path):
    me = os.getuid()
    camera_perms = os.stat(camera_path).st_uid
    if camera_perms != me:
        print("Fixing permissions..")
        os.system("sudo chown iximeow:iximeow {}".format(camera_path))

camera_path = get_path("1618:c367")

if camera_path == None:
    print("couldn't find the camera, firmware may have been loaded already?")
    sys.exit(0)

fix_perms(camera_path)

fw_cmd = "./fxload -t fx3 -I firmware/qhy/{} -D {}".format(
        "QHY367.img",
        camera_path
    )

print(fw_cmd)

fw_load = subprocess.check_output([
    "./fxload", '-t', 'fx3', '-I', 'firmware/qhy/{}'.format("QHY367.img"), '-D', camera_path])

print(fw_load)
