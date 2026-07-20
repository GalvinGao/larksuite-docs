---
document_id: '7263040419905503238'
directory_id: '7262910572679708677'
title: 服务端 SDK
full_path: /ukTMukTMukTM/uETO1YjLxkTN24SM5UjN
breadcrumb:
- Server API
- API Call Guide
- Server SDK
document_type: GuideDocumentType
updated_at: 2023-11-10T09:36:05Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uETO1YjLxkTN24SM5UjN
---

# 服务端 SDK

为了帮助开发者更加便捷地使用Lark开放能力开发应用，简化在接入Lark开放平台时的操作步骤，开放平台提供了统一的服务端 SDK。开发者可使用 SDK，快捷地开发功能，提升开发效率。 

SDK 提供的主要能力包括：


- SDK 提供了结构化的 API 请求入参。比如发消息 API，SDK 对各种类型的消息都提供了结构化封装。
	

	![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/f8a5610ed5dd8747cb04cf14b5476393_gsDZwiiCrT.png?height=478&lazyload=true&maxWidth=700&width=1482)

- 	SDK 提供了完整的访问凭证（AccessToken）生命周期管理能力，无需开发者自己获取并刷新访问凭证。
- SDK 提供了简洁明了、易懂的使用文档。
    - [Go SDK 说明文档](https://github.com/larksuite/oapi-sdk-go/blob/v3_main/README.md)
    - [Python SDK 说明文档](https://github.com/larksuite/oapi-sdk-python/blob/v2_main/README.md)
    - [Java SDK 说明文档](https://github.com/larksuite/oapi-sdk-java/blob/v2_main/README.md)
    - [NodeJS SDK 说明文档](https://github.com/larksuite/node-sdk/blob/main/README.zh.md)

- SDK 内 API 和事件上都添加了文字注释，以及跳转到使用 Demo 和官方文档的链接。



目前我们提供了Go、Python、Java、NodeJS 四种语言的 SDK。你可以进入 GitHub 项目空间查看详情。在使用 SDK 的过程中，如果遇到问题，可以给我们提交 Issue 或进入讨论群。


**GitHub 项目**                                               | **Issues**                                                  | **场景示例** | **语言**       |
| ----------------------------------------------------------- | ----------------------------------------------------------- | -------- | ------------ |
| [oapi-sdk-go](https://github.com/larksuite/oapi-sdk-go)     | [Issues](https://github.com/larksuite/oapi-sdk-go/issues)   | [oapi-sdk-go-demo](https://github.com/larksuite/oapi-sdk-go-demo)        | Golang >= 1.5 |
| [oapi-sdk-python](https://github.com/larksuite/oapi-sdk-python) | [Issues](https://github.com/larksuite/oapi-sdk-python/issues) | [oapi-sdk-python-demo](https://github.com/larksuite/oapi-sdk-python-demo)      | Python >= 3.8    |
| [oapi-sdk-java](https://github.com/larksuite/oapi-sdk-java) | [Issues](https://github.com/larksuite/oapi-sdk-java/issues) | [oapi-sdk-java-demo](https://github.com/larksuite/oapi-sdk-java-demo)      | Java >= 1.8    |
| [oapi-sdk-nodejs](https://github.com/larksuite/node-sdk)    | [Issues](https://github.com/larksuite/node-sdk/issues)      | -   | NodeJS

## 服务端 SDK 下载

你可以参考本文内容下载各端 SDK。

### Go

  执行以下命令安装最新版 Go SDK

  ```shell
  go get -u github.com/larksuite/oapi-sdk-go/v3
  ``` 
  
### Python

  使用 pip 安装最新版 Python SDK

  ```shell
  pip install lark-oapi -U
  ``` 
	
### Java

  在 pom.xml 文件中，添加以下依赖:

  ```xml 
  <dependency>
      <groupId>com.larksuite.oapi</groupId>
      <artifactId>oapi-sdk</artifactId>
      <version>{latest version}</version>
  </dependency>
  ```  

:::note
最新版本可以在[这里](https://mvnrepository.com/artifact/com.larksuite.oapi/oapi-sdk)查看。
:::

  ![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/2748a74e144085f38b3b185186c63abb_DpreW7xUML.png?height=1664&lazyload=true&maxWidth=750&width=3168)

### Node.js 
- 使用 npm 安装

  ```shell 
  npm install @larksuiteoapi/node-sdk
  ``` 
- 使用 yarn 安装

  ```shell 
  yarn add @larksuiteoapi/node-sdk
  ``` 

## 相关链接

- [服务端 API 列表](/document/ukTMukTMukTM/uYTM5UjL2ETO14iNxkTN/server-api-list)

