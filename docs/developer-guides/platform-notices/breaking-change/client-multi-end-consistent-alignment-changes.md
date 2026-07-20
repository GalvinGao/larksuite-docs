---
document_id: '7106762063150219270'
directory_id: '7077912803110010885'
title: 客户端多端一致对齐变更
full_path: /uAjLw4CM/ugTN1YjL4UTN24CO1UjN/breaking-change/client-multi-end-consistent-alignment-changes
breadcrumb:
- Developer Guides
- Platform Notices
- Breaking change
- Client multi-end consistent alignment changes
document_type: GuideDocumentType
updated_at: 2022-06-08T08:56:51Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/breaking-change/client-multi-end-consistent-alignment-changes
---

# 客户端多端一致对齐变更
### 变更事项
历史上我们有部分 API 的行为在三端表现不对齐（iOS，Android，PC），我们对部分主要 API 做了升级，升级后相应 API 在相同条件下的调用行为和返回结果会表现一致，并且 API 整体趋于严格校验，每个 API 的升级会影响某一端的前后表现，请开发者检查自己的代码并做好适配和升级。

变动内容如下：
* **iOS**: [share](/document/uYjL24iN/ugDM04COwQjL4ADN/thirdShare) 接口对齐 Android 逻辑，`channelType` 参数由宽松校验变更为严格校验，当前运行环境如果不支持分享渠道会返回失败。
* **PC**: [chooseImage](/document/uYjL24iN/uMTMx4yMxEjLzETM) 对齐 iOS & Android 逻辑，`sizeType` 参数由宽松校验变更为严格校验，如果传入类型不符合预期会返回失败。
* **iOS & Android**: [getStorage](/document/uYjL24iN/ukDOx4SO4EjL5gTM)/[getStorageSync](/document/uYjL24iN/uATOx4CM5EjLwkTM) 对齐 PC 逻辑，接口如果获取不到 `key` 对应的数据，返回的失败对象中不会再携带 `data` 和 `dataType` 字段。
* **PC**: [setStorage](/document/uYjL24iN/uETOx4SM5EjLxkTM)/[setStorageSync](/document/uYjL24iN/uITOx4iM5EjLykTM) 对齐 iOS & Android 逻辑，存储数据时而过传入的 `key` 为空字符串，不再返回成功，返回失败。
* **PC**: [removeStorage](/document/uYjL24iN/uMTOx4yM5EjLzkTM)/[removeStorageSync](/document/uYjL24iN/uQTOx4CN5EjL0kTM) 对齐 iOS & Android 逻辑，删除数据时如果传入的 key 不存在，不再返回失败，返回成功。
* **PC**: [openDocument](/document/uYjL24iN/ukTN24SO1YjL5UjN) 对齐 iOS & Android 逻辑，`fileType` 传入非 `cloudFile` 类型时，如果传入的 `filePath` 参数不是合法的沙箱文件路径，现在会返回失败。
* **iOS**: [saveImageToPhotosAlbum](/document/uYjL24iN/uUTMx4SNxEjL1ETM) 对齐 Android 逻辑，如果当前用户没有为Lark开启相册权限，`errMsg` 会返回相应的失败提示。
* **iOS & Android**: [chooseImage](/document/uYjL24iN/uMTMx4yMxEjLzETM) 对齐 PC 逻辑，选择图片时，保存图片到沙箱，如果部分保存失败，则会整体返回失败。
* **PC**: [getImageInfo](/document/uYjL24iN/ugjNwEjL4YDMx4CO2ATM) 对齐 iOS & Android 逻辑，如果传入的 `src` 参数不是合法的沙箱文件路径，则会在 `errMsg` 返回相应的失败提示。

<br>

是否跟版：是<br>
预计生效版本：Lark V5.11.0<br>
预计生效时间：2022-06-08<br>

### 潜在影响
客户端版本升级至 V5.11.0 后，当应用调用上述接口时，如果依赖了三端不对齐的表现，则会导致 API 在某一端表现不符合预期。

### 解决方案
请开发者检查自己的代码是否有依赖上述不对齐事项，如果依赖请按照变更后逻辑做好适配和调整。

<br>
如需适配协助，请洽技术支持。
