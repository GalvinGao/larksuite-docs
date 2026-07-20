---
document_id: '7330180313525714950'
directory_id: '6907567266540797954'
title: 蓝牙接入流程
full_path: /uYjL24iN/uMzNzYjLzczM24yM3MjN
breadcrumb:
- Client API
- Web app/Gadget API
- Device
- Bluetooth
- Bluetooth connection process
document_type: GuideDocumentType
updated_at: 2024-01-31T08:43:28Z
source_url: https://open.larksuite.com/document/uYjL24iN/uMzNzYjLzczM24yM3MjN
---

# 蓝牙接入流程

本文介绍在应用中接入蓝牙功能的流程，你可以点击相应的 API 文档链接查看详细说明。

1. 初始化蓝牙接口（[openBluetoothAdapter](/document/uYjL24iN/ugzNxYjL4cTM24CO3EjN)）。

2. 初始化事件监听。

    * 监听蓝牙适配器状态（[onBluetoothAdapterStateChange](/document/uYjL24iN/uADOxYjLwgTM24CM4EjN)）。

    * 监听蓝牙发现事件（[onBluetoothDeviceFound](/document/uYjL24iN/ukzNxYjL5cTM24SO3EjN)）。

    * 监听蓝牙连接状态事件（[onBLEConnectionStateChange](/document/uYjL24iN/uUTOxYjL1kTM24SN5EjN)）。

3. 搜索设备（[startBluetoothDevicesDiscovery](/document/uYjL24iN/uUzNxYjL1cTM24SN3EjN)）。

4. 查找设备并连接（[connectBLEDevice](/document/uYjL24iN/ucDOxYjL3gTM24yN4EjN)）。

5. 停止搜索设备（[stopBluetoothDevicesDiscovery](/document/uYjL24iN/uczNxYjL3cTM24yN3EjN)）。

6. 遍历蓝牙外设服务和特征。

    * 获取服务（[getBLEDeviceServices](/document/uYjL24iN/uATOxYjLwkTM24CM5EjN)）。

    * 获取特征（[getBLEDeviceCharacteristics](/document/uYjL24iN/ukDOxYjL5gTM24SO4EjN)）。

7. 监听特征值变化事件通知（[onBLECharacteristicValueChange](/document/uYjL24iN/uQTOxYjL0kTM24CN5EjN)）。

8. 设置读特征通知模式（[notifyBLECharacteristicValueChange](/document/uYjL24iN/uETOxYjLxkTM24SM5EjN)）。

9. 读写数据。

    * 向设备的特征值写数据（[writeBLECharacteristicValue](/document/uYjL24iN/ucTOxYjL3kTM24yN5EjN)）。

    * 向设备的特征值读数据（[readBLECharacteristicValue](/document/uYjL24iN/uYTOxYjL2kTM24iN5EjN)）。

10. 断开连接（[disconnectBLEDevice](/document/uYjL24iN/ugDOxYjL4gTM24CO4EjN)）。

11. 关闭蓝牙适配器（[closeBluetoothAdapter](/document/uYjL24iN/uYDOxYjL2gTM24iN4EjN)）。
