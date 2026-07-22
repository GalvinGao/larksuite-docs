---
document_id: '7180270043523219462'
directory_id: '7179507279661907974'
title: 配置参考
full_path: /uAjLw4CM/uYjL24iN/block/block-frame/config
breadcrumb:
- Developer Guides
- Develop Workplace Blocks
- Block Framework
- Configuration
document_type: GuideDocumentType
updated_at: 2022-12-27T10:18:35Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/block/block-frame/config
---

# 配置参考
## 项目配置

项目配置位于项目根目录下的 `project.config.json` 中，主要包含以下配置：

| 字段名         | 类型        | 说明                         |
| ----------- | --------- | -------------------------- |
| appid       | string    | 应用 ID，从Lark开放平台获取            |
| projectname | string    | 项目名称，在Lark开发者工具中展示           |
| blocks      | string[] | Block 相对路径，支持项目下配置多个 Block |
| setting     | object    | 编译选项                       |
|∟ es6|boolean|是否启用 es6 转 es5，默认为 true|
|∟ minified |boolean|是否自动压缩，默认为 true|



**示例代码**

```json
{
  "appid": "cli_1234567890",
  "projectname": "Block Demo",
  "blocks": [
    "blockA/index",
    "blockB/index"
  ],
  "setting": {
    "es6": true,
    "minified": true
  }
}
```

## Block 配置

Block 配置位于每个 Block 文件夹下，与 `.ttml` `.ttss` 等文件同级。例如对于 BlockA，其配置路径为 `blockA/index.json`。配置中包含：

| 字段名 | 类型 | 说明 |
| --- | --- | --- |
| blockTypeID | string | blockTypeID，在Lark开放平台开启小组件能力后获取 |
| creator | string | Creator 相对路径 |
| useStartLoading | boolean | 是否启动时使用 Block 框架提供的 Loading，默认值为 false。<br>开启后将使用 Block 框架提供的 Loading 动画，直到调用 `tt.hideBlockLoading` 隐藏，提升用户体验。<br>**注意**：如果开启`useStartLoading` 但长时间未调用 `tt.hideBlockLoading`，会导致组件渲染超时。 |
| darkmode | boolean | 是否开启 DarkMode，用于 Block 适配黑色主题 |
| showFrame | boolean | 是否使用 Block 框架提供的边框，默认值 `true`。开启后将使用 Block 提供的边框，在云文档中为 1px 的灰色边框，工作台中为带阴影的卡片 |
| schema | object | <md-alert type="tip">schema 配置项已经废弃，不再使用。</md-alert><br>用于定义 BlockEntity 中 SourceData 的数据结构，在 tt.setBlockInfo 中使用 BlockEntity 中的 SourceData 或数据协同能力时必填。设置或修改 schema 需要在开发者工具执行上传后生效。具体配置规则参见下文 |








**示例代码**

```json
{
  "blockTypeID": "blk_6041a9c6c800001c53e5d34a",
  "creator": "blockA/creator/index",
  "useStartLoading": false,
  "darkmode": true,
  "showFrame": true,
  "schema": {
    "sourceData": {
      "type": "object",
      "properties": {
        "point": {
          "type": "integer"
        }
      }
    }
  }
}
```

## Creator 配置

Creator 配置位于 Creator 的文件夹下，与 `ttml` `ttss` 等文件同级。例如对于 BlockA，其 Creator配置路径为 `blockA/creator/index.json`。配置中包含：

| 字段名 | 类型 | 说明 |
| --- | --- | --- |
| useStartLoading | boolean | 是否启动时使用 Block 框架提供的 Loading，默认值 false |
| needInterface | boolean | Creator 是否需要界面，默认值 true。当 `needInterface` 为 `false` 时，`useStartLoading` 设置的值无效 |




**示例代码**

```json
{
  "needInterface": true,
  "useStartLoading": true
}
```

## 宿主特有配置
Block 配置中的宿主字段记录了宿主的特有配置，这些配置仅在该宿主下运行 Block 时生效。

### 云文档
云文档暂无特有配置。

### 工作台

工作台中具有一些特定的配置，包含：

| 字段名              | 类型            | 说明                               |
| ---------------- | ------------- | -------------------------------- |
| needHeader       | boolean       | 是否显示卡片的 Header                   |
| title            | i18n<string> | 卡片 Header 中的标题，支持国际化             |
| titleIconUrl     | string        | 卡片 Header 中应用图标                  |
| pcHeaderLink     | string        | 卡片 Header 中应用图标点击后的跳转链接，仅 PC 端有效 |
| mobileHeaderLink | string        | 卡片 Header 中应用图标点击后的跳转链接，仅移动端有效   |
| menuItems        | MenuItem[]   | 卡片操作菜单中的选项                       |
|∟   key           |  string     |  菜单项的唯一标识      |
|∟   name          |   i18n<string> |    菜单项的名称，支持国际化      |
|∟   iconUrl       |  string  |  菜单项的图片地址        |


![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/18203bae8d5f4bb87d74ad3d9a58dd7d_toQaFrRdMC.png?lazyload=true&width=1948&height=1140)

其中：

-   可通过 `tt.updateMenuItem` 等 API 更新菜单项。
-   菜单的点击事件通过 `tt.onMenuItemTap` 监听。
- 未配置用户环境对应的国际化文案时，降级展示英文。

**示例代码**

```json
{
  "blockTypeID": "REPLACE_WITH_ACTUAL_BLOCK_TYPE_ID",
  "creator": "block/creator/index",
  "workplace": {
    "needHeader": true,
    "pcHeaderLink": "https://open.feishu.cn/",
    "mobileHeaderLink": "https://open.feishu.cn/m",
    "title": {
      "zh_cn": "工作台小组件",
      "en_us": "Workplace Widget"
    },
    "titleIconUrl": "https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/e844bc748f30713db4b8d46a1379a6a7.png",
    "menuItems": [
      {
        "key": "setting",
        "name": {
          "zh_cn": "设置",
          "en_us": "Settings"
        },
        "iconUrl": "https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/setting.svg"
      },
      {
        "key": "copy",
        "name": {
          "zh_cn": "复制",
          "en_us": "Copy"
        },
        "iconUrl": "https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/copy.svg"
      }
    ]
  }
}
```
  

