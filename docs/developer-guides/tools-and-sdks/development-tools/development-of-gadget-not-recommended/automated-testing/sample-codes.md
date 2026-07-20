---
document_id: '6967331158355804166'
directory_id: '6907567269107744770'
title: 代码示例
full_path: /uYjL24iN/uETM5YjLxETO24SMxkjN
breadcrumb:
- Developer Guides
- Tools and SDKs
- Development Tools
- Development of Gadget (Not Recommended)
- Automated Testing
- Sample Codes
document_type: GuideDocumentType
updated_at: 2022-11-17T05:57:01Z
source_url: https://open.larksuite.com/document/uYjL24iN/uETM5YjLxETO24SMxkjN
---

# 代码示例
::: note
通过本文，将快速的掌握一套基于 jest 框架的自动化测试框架。预计时间：3 分钟。
:::
## 下载模板

自动化测试模板 [点击下载](https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/b71f9cd0630e8f7993c0d32ab541b1d2.gz)

## 初始化项目

1. 安装依赖，工程根目录下执行如下脚本：

``` shell
npm install
```

2. 安装 jest

```
npm i jest -g
```

3. 安装 VSCode 插件（ 非 VSCode 用户可跳过此步 ）

* 打开 VSCode 应用商店，搜索 `Path Completion in fileTree` 或 `Lark`，安装如下图插件，并重启 VSCode 生效插件


![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/cd3f2d499eab7b753005063023209c4c_RxDAeIlz0A.png)

## 新增测试用例

1. 执行自动生成脚本，填写测试用例名称

```
npm run case:create
```
2. 将 case 加入测试入口

```
import test001 from '@/cases/test001'
describe("测试描述", () => {
  describe("测试 Test001", test001);
})
```

## 编写测试用例

> 用例语法参照 jest：<https://jestjs.io/docs/en/getting-started>

在 `cases/` 下找到新增的测试用例，编写相应的case, 如下

``` js
import 'babel-polyfill'
import { MiniProgram } from '@larksuitegadget/automator'
import gadagetTestApp from '@/app'
import miniProgram from '@/miniProgram'
/**
 * 测试首页列表是否正常渲染
 */
 const TEMPLATE_NAME = () => {
  let _miniProgram: MiniProgram
  let Element
  
  beforeAll(async () => {
     // 钩子：测试开始前做一些准备工作
    _miniProgram = await miniProgram.getMiniProgram();
    Element = gadagetTestApp.element.test001
  }, timeout);
  beforeEach(async () => {
    // 钩子：单个测试开始前做一些准备工作
    jest.setTimeout(timeout);
  });
  afterAll(async () => {
    // 钩子：所有case完成后await
    gadagetTestApp.utils.command.sleepWaitFor(timeout);
  });
  test("测试描述", async () => {
    // 测试脚本await page.waitFor(Element.LIST_ITEM);
     const listItem = await page.$(gadagetTestApp.element.test001.LIST_ITEM);
     expect(!!listItem).toBe(true);
     await gadagetTestApp.utils.command.sleepWaitFor(1000);
  });
};
export default TEMPLATE_NAME
```

## 运行

关于 connect , launch 命令的使用，[请点此查看](/document/uYjL24iN/ukDM4YjL5ADO24SOwgjN)

1. connect 模式

``` shell
opdev auto <projectPath>
npm run test:connect
```

2. launch 模式运行

    于根目录下创建 `launch.config.json`，添加如下配置

``` shell
{
  "socketPort": "5555",
  "projectPath": ${project_path},
  "cliPath": "" // cli 工具地址，可为空。具体使用参考 automator.launch
}
```
运行

``` shell
npm run test:launch
```

## 开发小技巧

case编写过程中，开发者需要定义大量的常量、命令和mock等。

为避免开发中引用太多文件，也为了常量能够复用，我们推荐以下解决方案：

1.  vscode 用户推荐（配合vscode插件可获取自动补全）:

    a. 通过`gadget-e2e-runtime` 库可以在不引入对应文件的情况下，使用该文件中导出的内容

``` js
// test/app.ts
const gadagetTestApp = require('gadget-e2e-runtime');
const path = require('path');
// 测试app的实例
const app = new gadagetTestApp({
  baseDir: path.join(__dirname, '.'),
});
export default app;
// test/cases/test001.ts
import 'babel-polyfill'
import { MiniProgram } from '@larksuitegadget/automator'
import gadagetTestApp from '@/app'
import miniProgram from '@/miniProgram'
import define from '@/cases/define';
const test001 = (): void => {
  let miniProgramInstance: MiniProgram
  beforeAll(async () => {
    miniProgramInstance = await miniProgram.getMiniProgram();
  }, 5 * 60 * 1000);
  beforeEach(async () => {
    jest.setTimeout(5 * 60 * 1000);
  });
  afterAll(async () => {
    await gadagetTestApp.utils.command.sleepWaitFor(1000);
  });
  test("测试-vscode用户推荐写法", async () => {
    const page = await miniProgramInstance.switchTab(
      gadagetTestApp.constant.page.COMPONENT_PAGE
    );
    if (page) {
      await page.waitFor(gadagetTestApp.element.test001.LIST_ITEM);
      const listItem = await page.$(gadagetTestApp.element.test001.LIST_ITEM);
      expect(!!listItem).toBe(true);
      await gadagetTestApp.utils.command.sleepWaitFor(1000);
    } else {
      expect(false).toBe(true);
    }
  });
 }
```
       b. 配合插件 “Path Completion in fileTree” 智能补足提示，如下图所示：	

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/93e27333320779115153f32dce934dad_sIprjgv45O.png)

2.  非 vscode 用户推荐

参考 `cases/define`，将所有常用的模块于 `define.ts` 中定义。

``` js
// define.ts
import Page from '@/constant/page';
import Text from '@/constant/text';
import elementTest001 from '@/element/test001';
import Command from '@/utils/command';
export default {
  page: Page,
  text: Text,
  elementTest001: elementTest001,
  command: Command,
};
// case.ts
import miniProgram from '@/miniProgram'
import define from '@/cases/define';
test('测试-非vscode用户推荐写法', async () => {
    const page = await miniProgramInstance.switchTab(
      define.page.COMPONENT_PAGE,
    );
    if (page) {
      await page.waitFor(define.elementTest001.LIST_ITEM);
      const listItem = await page.$(define.elementTest001.LIST_ITEM);
      expect(!!listItem).toBe(true);
      await define.command.sleepWaitFor(1000);
    } else {
      expect(false).toBe(true);
    }
  });
```

