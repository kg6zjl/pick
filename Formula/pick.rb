class Pick < Formula
    desc "Turn any piped newline separated input into an interactive menu."
    homepage "https://github.com/kg6zjl/pick"
    url "https://github.com/kg6zjl/pick/releases/download/v0.1.0/pick.tar.gz"
    sha256 "a751ca9c449c033a0b531e12197aa6ccfb68a655f8c8e676cbe4ecb374052cba"
    version "0.1.0"
  
    def install
      bin.install "pick"
    end
  end