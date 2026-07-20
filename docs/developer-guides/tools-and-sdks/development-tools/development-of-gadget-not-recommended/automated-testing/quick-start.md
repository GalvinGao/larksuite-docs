---
document_id: '6967331158355902470'
directory_id: '6907567269107744770'
title: 快速入门
full_path: /uYjL24iN/uYDM4YjL2ADO24iNwgjN
breadcrumb:
- Developer Guides
- Tools and SDKs
- Development Tools
- Development of Gadget (Not Recommended)
- Automated Testing
- Quick Start
document_type: GuideDocumentType
updated_at: 2022-11-17T05:56:46Z
source_url: https://open.larksuite.com/document/uYjL24iN/uYDM4YjL2ADO24iNwgjN
---

# 快速入门
## 运行环境
* 安装 `Node.js` 10.16 及以上，不推荐 15.0 以上版本
* 安装Lark开发者工具 1.6.0 及以上 [安装指南](/document/uYjL24iN/ucDOzYjL3gzM24yN4MjN)
* 在模拟器工具栏 - 详情 - 项目信息中，设置调试基础版本 `>= 1.9.30.2` [开发者工具调试指南](/document/uYjL24iN/ugDOzYjL4gzM24CO4MjN)
## 安装
使用小程序自动化 SDK，直接执行以下命令：
```
npm i @larksuitegadget/automator --save-dev
```
## 使用
然后直接引入 SDK 开始编写控制脚本，参考下边例子：
```js
const automator = require('@larksuitegadget/automator')
async function runTest(minigram) {
    const pageStacks = await minigram.pageStack()
    const page = await minigram.navigateTo('/page/component/pages/view/view')
    await page.waitFor(500)
    const element = await page.$('.page-body')
    await minigram.close()
}
const launchOptions = {
  maxTime: 30,
  projectPath,
};
automator
  .launch(launchOptions)
  .then(async (minigram) => {
    await runTest(minigram);
  })
  .catch((err) => {
    console.log("err=>", err);
  });
```
上例保存至文件 case.js，配合[小程序示例](/document/uYjL24iN/uYDM04iNwQjL2ADN)， 命令终端执行 `node [path]/case.js` ，可观测自动化执行结果。
