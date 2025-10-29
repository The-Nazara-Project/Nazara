# Testing VMs

In the previous chapter, you read up on how to set up a local
NetBox instance using [netbox-docker](https://github.com/netbox-community/netbox-docker).

This guide will show you how to set up a VM that you can register/update on this container
using Nazara.

## Prerequisites

- Host machine with the `netbox-docker` deployment running locally
- Virtualization software (QEMU/KVM)
- A Linux ISO of your choosing

## Create a new Linux VM

~~~admonish tip title="Use a GUI to create/manager VMs" collapsible=true
For creating and managing VMs, we recommend you use a tool like `virt-manager`.

You can find a step-by-step guide on how to create a new VM [here](https://docs.redhat.com/en/documentation/red_hat_enterprise_linux/6/html/virtualization_getting_started_guide/sec-virtualization_getting_started-quickstart_virt-manager-create_vm)
~~~

### Using the Terminal

~~~admonish info
These examples are specifically for openSUSE Tumbleweed.

For package names on other distros, please refer to the list below
or check your distro's package lists.
~~~

#### 1. Install required packages

```bash
sudo zypper install -y qemu-kvm libvirt virt-manager virt-install bridge-utils
sudo systemctl enable --now libvirtd
```

#### 2. Create Linux VM

Replace `$ISO_PATH` with the path to the ISO file and `$DISK_PATH` to the path
where you want your virtual disk to be localted.

1. Create a new virtual disk image

```bash
qemu-img create -f qcow2 $DISK_PATH 20G
```

2. Create the Virtual Machine

```bash
sudo virt-install \
    --name nazara-test-vm \
    --ram 2048 \
    --vcpus 2 \
    --os-type linux \
    --os-variant $VARIANT \
    --network network=default \
    --graphics spice \
    --cdrom $ISO_PATH \
    --disk path=$DISK_PATH,format=qcow2
```

~~~admonish example title="Example: Creating Ubuntu VM" collapsible=true
This is an example on how to create a new VM using Ubuntu Desktop 25.10.

1. Create the virtual disk

```bash
qemu-img create -f qcow2 /var/lib/libvirt/images/ubuntu.qcow2 20G
```

2. Create the VM

```bash
sudo virt-install \
  --name nazara-test-vm-ubuntu \
  --ram 2048 \
  --vcpus 2 \
  --os-type linux \
  --os-variant ubuntu25.10 \
  --network network=default \
  --graphics spice \
  --cdrom /home/user/Downloads/ubuntu-25-10-desktop-amd64.iso \
  --disk path=/var/lib/libvirt/images/ubuntu.qcow2,format=qcow2
```
~~~

~~~admonish example title="Example: Creating openSUSE Tumbleweed VM" collapsible=true
This is an example on how to create a new VM with openSUSE Tumbleweed.


```bash
qemu-img create -f qcow2 /var/lib/libvirt/images/tumbleweed.qcow2 20G
```

2. Create the VM

```bash
sudo virt-install \
  --name nazara-test-vm-tumbleweed \
  --ram 2048 \
  --vcpus 2 \
  --os-type linux \
  --os-variant opensuse-tumbleweed \
  --network network=default \
  --graphics spice \
  --cdrom /home/user/Downloads/ubuntu-25-10-desktop-amd64.iso \
  --disk path=/var/lib/libvirt/images/ubuntu.qcow2,format=qcow2
```
~~~

**Network Notes**:

- Default NAT (`virbr0`) -> host reachable from VM at `192.168.122.1` (on openSUSE)
- Optional: Use bridge or host-only network for static IP

## Transfer Nazara to the VM

If you used the standard network option for your VM, it is most likely not able
to connect to the internet directly. This means you cannot pull Nazara's GitHub repo.

You can however, move your binary and config file from your host machine to your VM
using SSH.

First check if you can reach the NetBox instance from your VM.
Then simply run this bash script to copy everything else over:

```bash
#!/bin/bash

# Usage: ./copy_to_vm.sh <VM_IP> <VM_USER> <PATH_TO_NAZARA_PROJECT>
# Example:
# ./copy_to_vm.sh 192.168.122.123 alice /home/user/Nazara

set -e

VM_IP="$1"
VM_USER="$2"
PROJECT_PATH="$3"

if [[ -z "$VM_IP" || -z "$VM_USER" || -z "$PROJECT_PATH" ]]; then
    echo "Usage: $0 <VM_IP> <VM_USER> <PATH_TO_NAZARA_PROJECT>"
    exit 1
fi

NAZARA_BINARY="$PROJECT_PATH/target/release/nazara"
CONFIG_FILE="/root/.config/nazara/config.toml"

if [[ ! -f "$NAZARA_BINARY" ]]; then
    echo "Error: Nazara binary not found at $NAZARA_BINARY"
    exit 1
fi

if [[ ! -f "$CONFIG_FILE" ]]; then
    echo "Error: config.toml not found at $CONFIG_FILE"
    exit 1
fi

echo "Copying Nazara binary to VM home directory..."
scp "$NAZARA_BINARY" "$VM_USER@$VM_IP:/home/$VM_USER/"

echo "Copying config file to VM root config directory..."
ssh "$VM_USER@$VM_IP" "sudo mkdir -p /root/.config/nazara"
scp "$CONFIG_FILE" "$VM_USER@$VM_IP:/tmp/config.toml"
ssh "$VM_USER@$VM_IP" "sudo mv /tmp/config.toml /root/.config/nazara/config.toml"

echo "Done!"

```
