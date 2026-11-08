# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
"""
Tests für ATCLang Treiber Layer (Driver Framework)
ATC-22+ | Sprint 3.1
"""
import sys
import os
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', '..'))

from atclang.runtime.driver_framework import (
    DriverRegistry, DriverState, DeviceClass, BusType,
    IoctlCode, PowerState, OpenHandle,
)
import pytest


# ════════════════════════════════════════════════════════════════
#  DRIVER REGISTRATION
# ════════════════════════════════════════════════════════════════

class TestDriverRegistration:

    def test_register_driver(self):
        reg = DriverRegistry()
        did = reg.register_driver(
            "DisplayDriver", "1.0.0", DeviceClass.DISPLAY,
            [0x1234], "init_display", "cleanup_display", gas_per_io=15
        )
        assert did == 1
        driver = reg.get_driver_info(did)
        assert driver.name == "DisplayDriver"
        assert driver.version == "1.0.0"
        assert driver.device_class == DeviceClass.DISPLAY
        assert driver.state == DriverState.LOADED
        assert driver.gas_per_io == 15

    def test_register_multiple_drivers(self):
        reg = DriverRegistry()
        d1 = reg.register_driver("DisplayDriver", "1.0", DeviceClass.DISPLAY, [], "init", "cleanup")
        d2 = reg.register_driver("KeyboardDriver", "1.0", DeviceClass.INPUT, [], "init", "cleanup")
        d3 = reg.register_driver("DiskDriver", "1.0", DeviceClass.STORAGE, [], "init", "cleanup")
        assert d1 == 1
        assert d2 == 2
        assert d3 == 3

    def test_init_driver(self):
        reg = DriverRegistry()
        did = reg.register_driver("TestDriver", "1.0", DeviceClass.NETWORK, [], "init", "cleanup")
        assert reg.init_driver(did) is True
        driver = reg.get_driver_info(did)
        assert driver.state == DriverState.INITIALIZED
        assert driver.sandbox_id > 0

    def test_init_driver_wrong_state(self):
        reg = DriverRegistry()
        did = reg.register_driver("Test", "1.0", DeviceClass.CUSTOM, [], "i", "c")
        reg.init_driver(did)
        with pytest.raises(ValueError, match="not in LOADED state"):
            reg.init_driver(did)

    def test_activate_driver(self):
        reg = DriverRegistry()
        did = reg.register_driver("Test", "1.0", DeviceClass.DISPLAY, [], "i", "c")
        reg.init_driver(did)
        assert reg.activate_driver(did) is True
        driver = reg.get_driver_info(did)
        assert driver.state == DriverState.ACTIVE

    def test_activate_uninitialized_driver(self):
        reg = DriverRegistry()
        did = reg.register_driver("Test", "1.0", DeviceClass.DISPLAY, [], "i", "c")
        with pytest.raises(ValueError, match="not initialized"):
            reg.activate_driver(did)

    def test_unload_driver(self):
        reg = DriverRegistry()
        did = reg.register_driver("Test", "1.0", DeviceClass.DISPLAY, [], "i", "c")
        reg.init_driver(did)
        reg.activate_driver(did)
        assert reg.unload_driver(did, "test unload") is True
        driver = reg.get_driver_info(did)
        assert driver.state == DriverState.UNLOADING

    def test_list_drivers_by_class(self):
        reg = DriverRegistry()
        d1 = reg.register_driver("Disp", "1.0", DeviceClass.DISPLAY, [], "i", "c")
        d2 = reg.register_driver("Disp2", "1.0", DeviceClass.DISPLAY, [], "i", "c")
        d3 = reg.register_driver("KB", "1.0", DeviceClass.INPUT, [], "i", "c")
        reg.init_driver(d1)
        reg.activate_driver(d1)
        reg.init_driver(d2)
        reg.activate_driver(d2)
        display_drivers = reg.list_drivers_by_class(DeviceClass.DISPLAY)
        assert d1 in display_drivers
        assert d2 in display_drivers
        assert d3 not in display_drivers


# ════════════════════════════════════════════════════════════════
#  DEVICE MANAGEMENT
# ════════════════════════════════════════════════════════════════

class TestDeviceManagement:

    def test_enumerate_device(self):
        reg = DriverRegistry()
        did = reg.enumerate_device(
            DeviceClass.DISPLAY, BusType.PCI, 0x1234, 0x5678,
            bus_address=0x0100, irq_line=11, mmio_base=0xE0000000,
            mmio_size=0x100000, name="GPU0", description="Test GPU"
        )
        assert did == 1
        dev = reg.get_device_info(did)
        assert dev.name == "GPU0"
        assert dev.vendor_id == 0x1234
        assert dev.bus == BusType.PCI
        assert dev.driver_id == 0

    def test_enumerate_multiple_devices(self):
        reg = DriverRegistry()
        d1 = reg.enumerate_device(DeviceClass.DISPLAY, BusType.PCI, 0x1234, 0x01, name="GPU")
        d2 = reg.enumerate_device(DeviceClass.INPUT, BusType.USB, 0x5678, 0x02, name="Keyboard")
        d3 = reg.enumerate_device(DeviceClass.STORAGE, BusType.PCI, 0x9090, 0x03, name="NVMe")
        assert d1 == 1
        assert d2 == 2
        assert d3 == 3

    def test_bind_driver(self):
        reg = DriverRegistry()
        driver_id = reg.register_driver("GPU", "1.0", DeviceClass.DISPLAY, [0x1234], "i", "c")
        reg.init_driver(driver_id)
        reg.activate_driver(driver_id)
        dev_id = reg.enumerate_device(DeviceClass.DISPLAY, BusType.PCI, 0x1234, 0x01, name="GPU0")
        assert reg.bind_driver(dev_id, driver_id) is True
        dev = reg.get_device_info(dev_id)
        assert dev.driver_id == driver_id

    def test_bind_wrong_class(self):
        reg = DriverRegistry()
        driver_id = reg.register_driver("GPU", "1.0", DeviceClass.DISPLAY, [], "i", "c")
        reg.init_driver(driver_id)
        reg.activate_driver(driver_id)
        dev_id = reg.enumerate_device(DeviceClass.STORAGE, BusType.PCI, 0x1234, 0x01, name="Disk")
        with pytest.raises(ValueError, match="class mismatch"):
            reg.bind_driver(dev_id, driver_id)

    def test_bind_wrong_vendor(self):
        reg = DriverRegistry()
        driver_id = reg.register_driver("GPU", "1.0", DeviceClass.DISPLAY, [0x1234], "i", "c")
        reg.init_driver(driver_id)
        reg.activate_driver(driver_id)
        dev_id = reg.enumerate_device(DeviceClass.DISPLAY, BusType.PCI, 0xFFFF, 0x01, name="Unknown GPU")
        with pytest.raises(ValueError, match="vendor not supported"):
            reg.bind_driver(dev_id, driver_id)

    def test_bind_already_bound(self):
        reg = DriverRegistry()
        driver_id = reg.register_driver("GPU", "1.0", DeviceClass.DISPLAY, [], "i", "c")
        reg.init_driver(driver_id)
        reg.activate_driver(driver_id)
        dev_id = reg.enumerate_device(DeviceClass.DISPLAY, BusType.PCI, 0x1234, 0x01, name="GPU0")
        reg.bind_driver(dev_id, driver_id)
        with pytest.raises(ValueError, match="already bound"):
            reg.bind_driver(dev_id, driver_id)

    def test_unbind_driver(self):
        reg = DriverRegistry()
        driver_id = reg.register_driver("GPU", "1.0", DeviceClass.DISPLAY, [], "i", "c")
        reg.init_driver(driver_id)
        reg.activate_driver(driver_id)
        dev_id = reg.enumerate_device(DeviceClass.DISPLAY, BusType.PCI, 0x1234, 0x01, name="GPU0")
        reg.bind_driver(dev_id, driver_id)
        assert reg.unbind_driver(dev_id) is True
        assert reg.get_device_info(dev_id).driver_id == 0

    def test_list_devices_by_class(self):
        reg = DriverRegistry()
        d1 = reg.enumerate_device(DeviceClass.DISPLAY, BusType.PCI, 0x01, 0x01, name="GPU")
        d2 = reg.enumerate_device(DeviceClass.DISPLAY, BusType.PCI, 0x02, 0x02, name="GPU2")
        d3 = reg.enumerate_device(DeviceClass.INPUT, BusType.USB, 0x03, 0x03, name="KB")
        displays = reg.list_devices_by_class(DeviceClass.DISPLAY)
        assert d1 in displays
        assert d2 in displays
        assert d3 not in displays

    def test_list_devices_by_bus(self):
        reg = DriverRegistry()
        d1 = reg.enumerate_device(DeviceClass.DISPLAY, BusType.PCI, 0x01, 0x01, name="GPU")
        d2 = reg.enumerate_device(DeviceClass.INPUT, BusType.USB, 0x03, 0x03, name="KB")
        pci_devices = reg.list_devices_by_bus(BusType.PCI)
        assert d1 in pci_devices
        assert d2 not in pci_devices


# ════════════════════════════════════════════════════════════════
#  I/O OPERATIONS
# ════════════════════════════════════════════════════════════════

class TestIOOperations:

    def setup_driver_and_device(self, reg=None):
        reg = reg or DriverRegistry()
        driver_id = reg.register_driver("Test", "1.0", DeviceClass.STORAGE, [], "i", "c")
        reg.init_driver(driver_id)
        reg.activate_driver(driver_id)
        dev_id = reg.enumerate_device(DeviceClass.STORAGE, BusType.PCI, 0x01, 0x01, name="Disk")
        reg.bind_driver(dev_id, driver_id)
        return reg, driver_id, dev_id

    def test_open_device(self):
        reg, driver_id, dev_id = self.setup_driver_and_device()
        handle_id = reg.open(dev_id, flags=3, owner_pid=1)
        assert handle_id >= 1
        handle = reg.open_handles[handle_id]
        assert handle.device_id == dev_id
        assert handle.owner_pid == 1

    def test_open_no_driver(self):
        reg = DriverRegistry()
        dev_id = reg.enumerate_device(DeviceClass.STORAGE, BusType.PCI, 0x01, 0x01, name="Disk")
        with pytest.raises(ValueError, match="no driver bound"):
            reg.open(dev_id)

    def test_open_exclusive(self):
        reg, driver_id, dev_id = self.setup_driver_and_device()
        h1 = reg.open(dev_id, flags=0x0B, owner_pid=1)  # READ+WRITE+EXCLUSIVE
        with pytest.raises(ValueError, match="exclusively"):
            reg.open(dev_id, flags=3, owner_pid=2)

    def test_read_write(self):
        reg, driver_id, dev_id = self.setup_driver_and_device()
        handle_id = reg.open(dev_id, flags=3, owner_pid=1)  # READ+WRITE
        data = reg.read(handle_id, 4096)
        assert data == "storage_data"
        written = reg.write(handle_id, "test data")
        assert written == len("test data")

    def test_read_wrong_flags(self):
        reg, driver_id, dev_id = self.setup_driver_and_device()
        handle_id = reg.open(dev_id, flags=2, owner_pid=1)  # WRITE_ONLY
        with pytest.raises(ValueError, match="not opened for reading"):
            reg.read(handle_id)

    def test_write_wrong_flags(self):
        reg, driver_id, dev_id = self.setup_driver_and_device()
        handle_id = reg.open(dev_id, flags=1, owner_pid=1)  # READ_ONLY
        with pytest.raises(ValueError, match="not opened for writing"):
            reg.write(handle_id, "test")

    def test_close(self):
        reg, driver_id, dev_id = self.setup_driver_and_device()
        handle_id = reg.open(dev_id, flags=3, owner_pid=1)
        assert reg.close(handle_id) is True
        assert handle_id not in reg.open_handles

    def test_seek(self):
        reg, driver_id, dev_id = self.setup_driver_and_device()
        handle_id = reg.open(dev_id, flags=3, owner_pid=1)
        assert reg.seek(handle_id, 1024, 0) == 1024
        assert reg.seek(handle_id, 512, 1) == 1536

    def test_ioctl_get_info(self):
        reg, driver_id, dev_id = self.setup_driver_and_device()
        handle_id = reg.open(dev_id, flags=3, owner_pid=1)
        result = reg.ioctl(handle_id, IoctlCode.GET_INFO)
        assert result == dev_id

    def test_ioctl_get_status(self):
        reg, driver_id, dev_id = self.setup_driver_and_device()
        handle_id = reg.open(dev_id, flags=3, owner_pid=1)
        result = reg.ioctl(handle_id, IoctlCode.GET_STATUS)
        assert result == int(DriverState.ACTIVE)


# ════════════════════════════════════════════════════════════════
#  IRQ MANAGEMENT
# ════════════════════════════════════════════════════════════════

class TestIRQManagement:

    def test_register_irq(self):
        reg = DriverRegistry()
        assert reg.register_irq(11, 1, 1, "irq_handler", 128) is True
        route = reg.irq_routes[11]
        assert route.irq_line == 11
        assert route.enabled is True

    def test_trigger_irq(self):
        reg = DriverRegistry()
        reg.register_irq(11, 1, 1, "handler", 128)
        assert reg.trigger_irq(11) is True
        assert reg.irq_routes[11].trigger_count == 1

    def test_trigger_unregistered(self):
        reg = DriverRegistry()
        with pytest.raises(ValueError, match="not registered"):
            reg.trigger_irq(99)

    def test_trigger_disabled(self):
        reg = DriverRegistry()
        reg.register_irq(11, 1, 1, "handler", 128)
        reg.irq_routes[11].enabled = False
        with pytest.raises(ValueError, match="disabled"):
            reg.trigger_irq(11)

    def test_unregister_irq(self):
        reg = DriverRegistry()
        reg.register_irq(11, 1, 1, "handler", 128)
        assert reg.unregister_irq(11) is True
        assert 11 not in reg.irq_routes


# ════════════════════════════════════════════════════════════════
#  POWER MANAGEMENT
# ════════════════════════════════════════════════════════════════

class TestPowerManagement:

    def setup_method(self):
        self.reg = DriverRegistry()
        self.driver_id = self.reg.register_driver("Test", "1.0", DeviceClass.DISPLAY, [], "i", "c")
        self.reg.init_driver(self.driver_id)
        self.reg.activate_driver(self.driver_id)
        self.dev_id = self.reg.enumerate_device(DeviceClass.DISPLAY, BusType.PCI, 0x01, 0x01, name="GPU")
        self.reg.bind_driver(self.dev_id, self.driver_id)

    def test_set_power_off(self):
        assert self.reg.set_power_state(self.dev_id, PowerState.OFF) is True
        driver = self.reg.get_driver_info(self.driver_id)
        assert driver.state == DriverState.SUSPENDED

    def test_set_power_on(self):
        self.reg.set_power_state(self.dev_id, PowerState.OFF)
        assert self.reg.set_power_state(self.dev_id, PowerState.ON) is True
        driver = self.reg.get_driver_info(self.driver_id)
        assert driver.state == DriverState.ACTIVE


# ════════════════════════════════════════════════════════════════
#  STATS & ERRORS
# ════════════════════════════════════════════════════════════════

class TestStatsErrors:

    def test_get_stats(self):
        reg = DriverRegistry()
        reg.register_driver("Test", "1.0", DeviceClass.DISPLAY, [], "i", "c")
        reg.enumerate_device(DeviceClass.DISPLAY, BusType.PCI, 0x01, 0x01, name="GPU")
        stats = reg.get_stats()
        assert stats[0] == 1  # drivers
        assert stats[1] == 1  # devices
        assert stats[2] == 0  # handles
        assert stats[3] == 0  # io_ops
        assert stats[4] == 0  # errors

    def test_report_error(self):
        reg = DriverRegistry()
        did = reg.register_driver("Test", "1.0", DeviceClass.DISPLAY, [], "i", "c")
        reg.report_error(did, "test error")
        driver = reg.get_driver_info(did)
        assert driver.error_count == 1
        assert driver.last_error == "test error"

    def test_io_count_increments(self):
        reg = DriverRegistry()
        driver_id = reg.register_driver("Test", "1.0", DeviceClass.STORAGE, [], "i", "c")
        reg.init_driver(driver_id)
        reg.activate_driver(driver_id)
        dev_id = reg.enumerate_device(DeviceClass.STORAGE, BusType.PCI, 0x01, 0x01, name="Disk")
        reg.bind_driver(dev_id, driver_id)
        handle_id = reg.open(dev_id, flags=3, owner_pid=1)
        reg.read(handle_id, 1024)
        reg.write(handle_id, "data")
        stats = reg.get_stats()
        assert stats[3] == 3  # open + read + write


# ════════════════════════════════════════════════════════════════
#  FULL LIFECYCLE INTEGRATION
# ════════════════════════════════════════════════════════════════

class TestFullLifecycle:

    def test_complete_driver_lifecycle(self):
        reg = DriverRegistry()

        # 1. Register
        did = reg.register_driver("NVMeDriver", "2.0", DeviceClass.STORAGE,
                                  [0x8086], "init_nvme", "cleanup_nvme", gas_per_io=5)
        assert reg.get_driver_info(did).state == DriverState.LOADED

        # 2. Init
        assert reg.init_driver(did) is True
        assert reg.get_driver_info(did).state == DriverState.INITIALIZED

        # 3. Enumerate device
        dev_id = reg.enumerate_device(DeviceClass.STORAGE, BusType.PCI,
                                      0x8086, 0x0953, irq_line=10,
                                      mmio_base=0xE0000000, mmio_size=0x40000,
                                      name="NVMe0", description="Intel NVMe SSD")

        # 4. Activate + Bind
        assert reg.activate_driver(did) is True
        assert reg.bind_driver(dev_id, did) is True
        assert reg.get_device_info(dev_id).driver_id == did

        # 5. I/O
        handle_id = reg.open(dev_id, flags=3, owner_pid=1)
        data = reg.read(handle_id, 4096)
        assert data == "storage_data"
        written = reg.write(handle_id, "hello world")
        assert written == 11

        # 6. IRQ
        assert reg.register_irq(10, dev_id, did, "nvme_irq", 200) is True
        assert reg.trigger_irq(10) is True

        # 7. Power management
        assert reg.set_power_state(dev_id, PowerState.STANDBY) is True
        assert reg.get_driver_info(did).state == DriverState.SUSPENDED
        assert reg.set_power_state(dev_id, PowerState.ON) is True
        assert reg.get_driver_info(did).state == DriverState.ACTIVE

        # 8. Close + Unload
        assert reg.close(handle_id) is True
        assert reg.unbind_driver(dev_id) is True
        assert reg.unload_driver(did, "shutdown") is True
        assert reg.get_driver_info(did).state == DriverState.UNLOADING

        # 9. Stats
        stats = reg.get_stats()
        assert stats[0] == 1  # 1 driver registered
        assert stats[1] == 1  # 1 device enumerated
        assert stats[3] >= 3  # at least open + read + write
