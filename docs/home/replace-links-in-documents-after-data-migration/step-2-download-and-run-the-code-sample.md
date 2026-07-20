---
document_id: '7233612551992180742'
directory_id: '7199928167142178821'
title: 步骤二：下载并运行示例代码
full_path: /home/replace-links-in-documents-after-data-migration/run
breadcrumb:
- Home
- Replace links in documents after data migration
- 'Step 2: Download and run the code sample'
document_type: GuideDocumentType
updated_at: 2023-11-07T12:40:42Z
source_url: https://open.larksuite.com/document/home/replace-links-in-documents-after-data-migration/run
---

# 步骤二：下载并运行示例代码
在本步骤，你将下载并运行教程提供的示例代码。示例代码使用 Python 语言编写，请确保你已安装 Python 运行环境。

## 操作步骤
1. 执行以下命令，下载示例代码到本地。
   * Mac OS 或 Linux 执行以下命令。
      ```Shell
      curl https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/23b78c84fbc59b1696e04e5d3875baff_FiF1C5ogTW.zip -o link_quick_start.zip
      unzip link_quick_start.zip
      cd link_quick_start/python
      ```

   * Windows 执行以下命令。
      ```Shell
      curl https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/23b78c84fbc59b1696e04e5d3875baff_FiF1C5ogTW.zip -o link_quick_start.zip
      link_quick_start.zip
      cd link_quick_start/python
      ```

2. 执行以下命令，修改`.env`文件中的应用凭证为[步骤一](/document/home/replace-links-in-documents-after-data-migration/list-of-apis)的测试应用凭证。

	Windows用户可直接使用记事本进行修改。

    ```PowerShell
    vim .env
    ```

   1. 按 `i` 进入编辑模式，然后修改 `.env` 文件中的 **APP_ID** 和 **APP_SECRET** 属性替换为你的应用的凭证数据。
      
      应用凭证信息可以在 [开发者后台](https://open.larksuite.com/app) 的 **凭证与基础信息** 页查看。
      ![图片](https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/be069cb96e07a3512076c0e6aba814db.png?height=1024&lazyload=true&width=2690)
   
   2. 修改完成后，按 `:wq` 保存退出。
  
      私有化部署时要修改`.env`文件中 **Lark_HOST** 为私有化部署所用的域名。
      
3. 执行以下命令，创建并激活Python虚拟环境。
   * Mac OS 或 Linux执行以下命令。
      ```Shell
      python3 -m venv venv
      . venv/bin/activate
      ```

   * Windows执行以下命令。
      ```Shell
      python3 -m venv venv
      venv\Scripts\activate
      ```

4. 执行以下命令，安装代码依赖。
   ```Shell
   pip3 install -r requirements.txt
   ```

5. 依赖安装成功后，执行以下命令，运行示例代码。
   ```Shell
   python3 link.py
   ```

   示例代码运行时，会依次调用   教程简介   中提到的所有接口，你可以在控制台查看调用过程。
   成功的运行结果如下图所示：
   :::html
   <img src="https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/9b037fa7cc7db4f6c11a29c23ef3d99d.png?lazyload=true&width=1806&height=1688" style="width:70%">
   :::
