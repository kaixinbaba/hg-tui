class HgTui < Formula
  desc "使用 TUI 界面去浏览 HelloGitHub 网站"
  homepage "https://github.com/kaixinbaba/hg-tui"
  url "https://github.com/kaixinbaba/hg-tui/releases/download/0.1.1/hgtui_0_1_1_macOS.zip"
  sha256 "baf577c90a9671357ee02ab6e5f2987c9eb2dc93daaf71c8c43baeb70a76383b"
  license "GPL-3.0"

  def install
    system "mv hgtui-0.1.1/hgtui hgtui"
    bin.install "hgtui"
  end

  test do
    system "false"
  end
end
