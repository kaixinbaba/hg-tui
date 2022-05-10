class HgTui < Formula
  version "0.1.2"
  desc "A TUI application to view www.hellogithub.com"
  homepage "https://github.com/kaixinbaba/hg-tui"

  disable! because: "it is now in homebrew core. Please reinstall it as follows:\nbrew untap kaixinbaba/hg-tui\nbrew install hg-tui\n"

  if OS.mac?
      url "https://github.com/kaixinbaba/hg-tui/releases/download/#{version}/hgtui_#{version}_macOS.tar.gz"
      sha256 "9d0fa738d8c421761536b3c4044a8a29728fc68cd60873fb552720e2d697b998"
  elsif OS.linux?
      url "https://github.com/kaixinbaba/hg-tui/releases/download/#{version}/hgtui_#{version}_linux.tar.gz"
      sha256 "6dd535c3902878e9de94a77cbe5439cfd6f11a1703929c0f07a6ee80f6e8f724"
  end

  conflicts_with "hgtui"

  def install
    bin.install "hgtui"
    ohai "Just use hgtui"
  end
end
