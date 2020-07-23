class Unit < Formula
  version "0.1.0"
  desc "Universal Test Generator"
  homepage "https://github.com/rogeriochaves/unit"
  url "https://github.com/rogeriochaves/unit/releases/download/#{version}/unit-#{version}-x86_64-apple-darwin.tar.gz"
  sha256 "8231b4b309ce346d4b69d4adadd3da9f86f4caafc401a955b91e587d47d0b128"

  def install
    bin.install "unit"
  end
end
