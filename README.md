# Sample skk server
個人的に仕様を確認するためのSKKサーバ。様々な定数がマジックナンバーなので、実用には向きません。

一応ddskkから変換はできます。

富豪的に辞書を全部読み込んでオンメモリに保持しています。ちなみにSKK-JISYO.LをiconvでUTF-8に変換したファイルで6MBほどなので、現代のPCならばこれくらいは支障ないでしょう。


# How to use
## Build Dependencies
rustc 1.24.0 とそれに対応するcargo

## Runtime Dependencies and Prerequisites
~/.sampleskkserv/SKK-JISYO.L.utf8 に辞書ファイルがある事

Port 1178 が空いている事

## Build
```
git clone [this repository]
cd sampleskkserv
cargo build --release
```

## Configure ddskk
```elisp
(setq
    skk-server-prog "/path/to/the/cloned/repo/target/release/sampleskkserv"
    skk-server-inhibit-startup-server nil    
    skk-server-host "localhost"
    skk-server-portnum 1178
    skk-server-report-response t)
```