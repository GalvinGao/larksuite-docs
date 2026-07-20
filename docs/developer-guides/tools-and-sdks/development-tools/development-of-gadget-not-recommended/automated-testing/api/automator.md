---
document_id: '6967331158355918854'
directory_id: '6907567269107826690'
title: Automator
full_path: /uYjL24iN/ukDM4YjL5ADO24SOwgjN
breadcrumb:
- Developer Guides
- Tools and SDKs
- Development Tools
- Development of Gadget (Not Recommended)
- Automated Testing
- API
- Automator
document_type: GuideDocumentType
updated_at: 2022-11-17T05:56:49Z
source_url: https://open.larksuite.com/document/uYjL24iN/ukDM4YjL5ADO24SOwgjN
---

# Automator
Automator 模块提供了connect(链接）、launch（启动自动化命令&链接）开发者工具的方法。
### 方法
#### automator.connect
连接开发者工具。
```
automator.connect(options: ConnectOptions): Promise<MiniProgram>
```
ConnectOptions 定义如下：

| 字段         | 类型     | 必填 | 默认值  | 说明                 |
| ---------- | ------ | -- | ---- | ------------------ |
| socketPort | string | 是  | 5555 | 开发者工具 WebSocket 地址 |

使用步骤：
1. 通过开发者工具命令行启动自动化
命令：
` opdev auto <project_path> --auto-port <auto-port>  ` 
示例：
```
opdev auto /Users/username/demo --auto-port 5555
```
> 开发者工具使用说明，请点[此处](/document/uYjL24iN/ucDOzYjL3gzM24yN4MjN)。
2. 链接开发者工具，进行自动化测试
示例：
```
  automator
  .connect()
  .then(async minigram => {
    const pageStacks = await minigram.pageStack()
    const page = await minigram.navigateTo('/page/component/pages/view/view')
  })
```
#### automator.launch
启动并连接开发者工具。
```
launch(options: LaunchOptions): Promise<Miniprogram>
```
LaunchOptions 字段定义如下：
> account 请配合开发者工具 1.8.0 及以上版本

| 字段名称 | 类型 | 必填| 默认值|说明|
| ---------- | ---- | -- | ---- | ------------------ |
|cliPath|	string|	否 |	全局安装的 opdev 地址（which / where opdev）	|开发者工具命令行工具绝对路径|
|projectPath	|string|	是|	-	|项目绝对路径|
|maxTime	|number	|否	|50000	|启动最长等待时间|
|port	|number	|否|	55555	|WebSocket 端口号|
|account	|string	|否|	-	|`Automation-Token`请通过命令行工具 `opdev whoami` 获取|


示例代码：
```
const launchOptions = {
  maxTime: 30,
  projectPath,
};
automator
  .launch(launchOptions)
  .then(async (minigram) => {
    const pageStacks = await minigram.pageStack()
    const page = await minigram.navigateTo('/page/component/pages/view/view')
  })
  .catch((err) => {
    console.log("err=>", err);
  });
```
