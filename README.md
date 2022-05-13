
<div align="center">
  <a href="https://github.com/kaixinbaba/hg-tui">
    <img src="doc/img/logo.png" alt="Logo" width="80" height="80">
  </a>

  <h3 align="center">hg-tui</h3>

  <p align="center">
    在命令行里浏览、搜索、访问 <a href="https://github.com/521xueweihan">HelloGitHub</a> 的开源项目。
    <br />
    <a href="https://github.com/kaixinbaba/hg-tui/doc/instruction.md"><strong>使用说明 »</strong></a>
    <br />
    <br />
    <a href="https://cdn.jsdelivr.net/gh/521xueweihan/img_logo@main/logo/weixin.png"><img src="https://img.shields.io/badge/Talk-%E5%BE%AE%E4%BF%A1%E7%BE%A4-brightgreen.svg?style=popout-square" alt="WeiXin"></a>
    <a href="https://github.com/kaixinbaba/hg-tui/stargazers"><img src="https://img.shields.io/github/stars/kaixinbaba/hg-tui.svg?style=popout-square" alt="GitHub stars"></a>
    <a href="https://github.com/kaixinbaba/hg-tui/issues"><img src="https://img.shields.io/github/issues/kaixinbaba/hg-tui.svg?style=popout-square" alt="GitHub issues"></a>
        <a href="https://weibo.com/hellogithub"><img src="https://img.shields.io/badge/%E6%96%B0%E6%B5%AA-Weibo-red.svg?style=popout-square" alt="Sina Weibo"></a>
  </p>
</div>

## 一、介绍

<p align="center"><img src='https://raw.githubusercontent.com/kaixinbaba/hg-tui/doc/img/cover.png' style="max-width:80%; max-height=80%;"></img></p>

在终端轻松浏览 HelloGitHub 的命令行工具。

快速上手指南：

1. 安装后启动的命令：hg-tui
2. 进入程序后，默认展示最新一期月刊
3. 可通过 `j/k` 上下移动光标，按下 `o` 查看详细介绍
4. 同时按下 `Ctrl+j` 进入搜索模式，输入关键字即可搜索项目
5. 遇到问题，同时按下 `Ctrl+h` 获取帮助
6. 按下 `q` 键退出程序


## 二、安装

**第一种：直接下载使用**

下载就能用！Windows、Linux、macOS 系统对应的可执行文件，[点击下载](https://github.com/kaixinbaba/hg-tui/releases)

<p align="center"><img src='https://raw.githubusercontent.com/kaixinbaba/hg-tui/doc/img/releases.png' style="max-width:80%; max-height=80%;"></img></p>


**第二种：自行编译**

```bash
$ git clone https://github.com/kaixinbaba/hg-tui.git
$ cd hg-tui
$ cargo install --path .
$ hgtui
```

## 三、快捷键

![](doc/img/help.png)

基本：
- `j/k`：移动（上/下）
- `h/l`：翻页（上/下）
- `gg`：移动至首行
- `G`：移动至末行
- `o`：查看/关闭详细介绍
- `回车`：访问开源项目页
- `s`：Star [HelloGitHub](https://github.com/521xueweihan/HelloGitHub)
- `q`：退出

组合快捷键：
- `Ctrl+h`：获得帮助
- `Ctrl+k/j`：切换到搜索/浏览模式

高级搜索：
- `#{数字}`：按期搜索
- `${类别}`：按类搜索

## 四、技术

项目中使用到的技术：

- 基础设施： anyhow、thiserror、lazy_static、better-panic
- 绘制 UI：tui、crossterm
- HTTP client：reqwest
- 缓存：cached
- HTML 解析：nipper
- 工具：regex、crossbeam-channel
- 命令行：clap

目录结构：

```
src
├── app.rs		// 统一管理整个应用的状态
├── cli.rs		// 命令行解析
├── draw.rs		// 绘制 UI
├── events.rs   // UI 事件、输入事件、通知
├── fetch.rs	// HTTP 请求
├── main.rs		// 入口
├── parse.rs	// HTML 解析
├── utils.rs	// 工具
└── widget 		// 自定义组件
    ├── ...
```

## 更新计划

欢迎加入我们一起贡献。

### 0.1.1
- [ ] 本地加速 GitHub 访问
- [ ] split 解析 usize 需要抽取方法或者更 rusty 的，限制最大期数不应该分散再各个地方，在 parse 结束的时候就应该能使用 usize 返回

## 0.1.0
- [x] 修复翻页期数突破最大期数
- [x] 关键词搜索
- [x] 按期数搜索
- [x] 按类别搜索
- [x] 通过浏览器打开项目地址

## 联系我
- 🔭 热爱开源，方向是 Java、Python、Rust 
- 🌱 微信公众号： 代码科学家
- 💬 bilibili： 老荀
- <a href="mailto:595666367@qq.com">点击</a>给我发邮件
