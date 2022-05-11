class HgTui < Formula
  desc "使用 TUI 界面去浏览 HelloGitHub 网站"
  homepage "https://github.com/kaixinbaba/hg-tui"
  url "https://github.com/kaixinbaba/hg-tui/archive/0.1.2.tar.gz"
  sha256 "1e3eed1ad8f18d7649285e9874c1d441730884e1701af536f573dbae4a2f6afd"
  license "GPL-3.0"

  def install
    system "mv hgtui-0.1.1/hgtui hgtui"
    bin.install "hgtui"
  end

  test do
    system "false"
  end
end
