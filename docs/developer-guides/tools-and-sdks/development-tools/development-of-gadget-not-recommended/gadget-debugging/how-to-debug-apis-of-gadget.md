---
document_id: '7164602851313893381'
directory_id: '7027037607365754885'
title: 小程序API的调试方法概览
full_path: /uYjL24iN/uEzMzUjLxMzM14SMzMTN/how-to-debug-apis-of-gadget
breadcrumb:
- Developer Guides
- Tools and SDKs
- Development Tools
- Development of Gadget (Not Recommended)
- Gadget Debugging
- how to debug APIs of Gadget
document_type: GuideDocumentType
updated_at: 2022-11-17T05:56:40Z
source_url: https://open.larksuite.com/document/uYjL24iN/uEzMzUjLxMzM14SMzMTN/how-to-debug-apis-of-gadget
---

# 小程序API的调试方法概览
小程序API需要使用Lark客户端的能力，目前Lark开发者工具的模拟器尚不支持。因此需要你通过mock方式或真机调试方式来辅助开发。


| 业务模块         | 接口           | 模拟器        | API mock         | 真机调试        |
| --------- | --------------- | -------   | ----------- | --------- |
|`生物认证` | [所有接口](/document/uYjL24iN/uQjMuQjMuQjM//face-comparison) | 不支持 | 支持 | 支持 |
|`聊天` | [所有接口](/document/getBlockActionSourceDetail) | 不支持 | 支持 | 支持 |
|`联系人` | [所有接口](/document/uYjL24iN/uMTM04yMxQjLzEDN)| 不支持 | 支持 | 支持 |
|`系统认证` | [startDeviceCredential](/document/uYjL24iN/uIDN14iM0UjLyQTN) | 不支持 | 支持 | 支持 |
|`邮件` | [mailto](/document/uYjL24iN/uAjNwEjLwYDMx4CM2ATM)| 不支持 | 支持 | 支持 |
|`水印` |[checkWatermark](/document/uYjL24iN/ukTM1EjL5ETNx4SOxUTM) | 不支持 | 支持 | 支持 |
|`安全密码验证` | [startPasswordVerify](/document/uYjL24iN/ugTO3IjL4kzNy4CO5cjM)| 不支持 | 支持 | 支持 |
|`Lark启动参数` | [getHostLaunchQuery](/document/uYjL24iN/ugzM4UjL4MDO14COzgTN) | 不支持 | 支持 | 可以获取，但是不支持自定义 |
|`Lark启动参数` | [getLaunchOptionsSync](/document/uYjL24iN/uAzM1YjLwMTN24CMzUjN) | 不支持 | 支持 | 支持 |
|`网络` |[所有接口](/document/uYjL24iN/ucDMx4yNwEjL3ATM)| 不支持 | 不支持 | 支持 |
|`文件` | [docsPicker](/document/uYjL24iN/ukTN3UjL5UzN14SO1cTN)， [openDocument](/document/uYjL24iN/ukTN24SO1YjL5UjN) | 不支持 | 不支持 | 支持 |
|`文件` | [saveFile](/document/uYjL24iN/ugDOz4CO4MjL4gzM)，[saveFileAs](/document/uYjL24iN/uQjN3UjL0YzN14CN2cTN) ， [filePicker](/document/uYjL24iN/uETM04SMxQjLxEDN)| 不支持 | 支持 | 不支持 |
|`设备/系统信息` | [所有接口](/document/uYjL24iN/uUjNx4SN2EjL1YTM) | 不支持 | 支持 | 支持 |
|`WiFi` | [所有接口](/document/uYjL24iN/ugjNx4CO2EjL4YTM)| 不支持 | 支持 | 支持 |
|`NFC` | [所有接口](/document/uYjL24iN/ukzM4YjL5MDO24SOzgjN)| 不支持 | 不支持 | 支持 |
|`加速度计` | [所有接口](/document/uYjL24iN/ukjNx4SO2EjL5YTM)| 工具箱 ![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/c10aa0ff67c35f40f08f81bd538e601a_XlyZqlcMlt.png?lazyload=true&width=832&height=1116)| 不支持 | 支持 |
|`罗盘` | [所有接口](/document/uYjL24iN/uMzNx4yM3EjLzcTM)| 工具箱 ![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/e2d838a9c7c912060869a5d4cc02439f_Uzociw5aiG.png?lazyload=true&width=846&height=924)| 不支持 | 支持 |
|`拨打电话` | [makePhoneCall](/document/uYjL24iN/uUzNx4SN3EjL1cTM) | 不支持 | 支持 | 支持 |
|`扫码` | [scanCode](/document/uYjL24iN/uYzNx4iN3EjL2cTM)| 工具箱 ![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/c8fefa36972d83cb4a30fe90c38f8639_6jz7NOQMHa.png?lazyload=true&width=852&height=800) | 支持 | 支持 |
|`屏幕亮度` | [所有接口](/document/uYjL24iN/uIjNx4iM2EjLyYTM/get-screen-brightness)| 支持 | 支持 |
|`震动` | [所有接口](/document/uYjL24iN/uEDOx4SM4EjLxgTM)|不支持| 支持 | 支持 |
|`截屏监听` | [所有接口](/document/uYjL24iN/uMjNwEjLzYDMx4yM2ATM) | 工具箱 ![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/46f70c98d73c80118cd92542a08a4c0f_nPMfHxqt52.png?lazyload=true&width=1038&height=502)| 支持 | 支持 |
|`CustomizedInput` | [getCustomizedInput](/document/uYjL24iN/uEDN1EjLxQTNx4SM0UTM)|只支持文本输入 | 支持 | 支持 |
|`窗口` | [所有界面](/document/uYjL24iN/uADO3UjLwgzN14CM4cTN)| 不支持 | 不支持 | 支持 |
|`更新` | [getUpdateManager](/document/uYjL24iN/uEzM04SMzQjLxMDN)| 工具箱 | 不支持 | 不支持 |




