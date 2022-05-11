class HgTui < Formula
  desc "使用 TUI 界面去浏览 HelloGitHub 网站"
  homepage "https://github.com/kaixinbaba/hg-tui"
  url "https://github.com/kaixinbaba/hg-tui/archive/0.1.3.tar.gz"
  sha256 "23cbd2b92010c66849c2c8832e0bb2837697c19abefbd0fb8e19580fc1f33da6"
  license "GPL-3.0"

  def install
    system "mv hgtui-0.1.1/hgtui hgtui"
    bin.install "hgtui"
  end

  test do
    system "false"
  end
end
